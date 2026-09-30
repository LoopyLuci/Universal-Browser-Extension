//! The daemon-side NM supervisor.

use std::{
    path::PathBuf,
    sync::Arc,
    time::SystemTime,
};

use dnea_core::{
    codec::{Codec, Decoded, DEFAULT_MTU},
    envelope::{Envelope, MessageType, Party, TransportDirection, PROTOCOL_VERSION},
    DneaError, DneaResult, EnvelopeId,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{mpsc, Semaphore},
};
use tracing::{debug, info, warn};

use crate::session::{ClientId, ClientInfo, SessionSink};

const MAX_FRAME_BYTES: usize = 64 * 1024 * 1024;

#[derive(Clone, Debug)]
pub struct NmHostConfig {
    pub endpoint: PathBuf,
    pub max_connections: usize,
    pub mtu: usize,
}

impl NmHostConfig {
    #[must_use]
    pub fn new(endpoint: PathBuf) -> Self {
        Self { endpoint, max_connections: 64, mtu: DEFAULT_MTU }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NmHostError {
    #[error("endpoint {0} is already in use")]
    AlreadyBound(PathBuf),
    #[error("listener I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("session sink rejected client: {0}")]
    SinkRejected(DneaError),
}

pub struct NmSupervisor {
    config: NmHostConfig,
    sink: Arc<dyn SessionSink>,
    conn_limit: Arc<Semaphore>,
}

impl NmSupervisor {
    #[must_use]
    pub fn new(config: NmHostConfig, sink: Arc<dyn SessionSink>) -> Self {
        Self {
            conn_limit: Arc::new(Semaphore::new(config.max_connections)),
            config,
            sink,
        }
    }

    pub async fn run(self) -> Result<(), NmHostError> {
        #[cfg(unix)]
        {
            let endpoint = self.config.endpoint.clone();
            if endpoint.exists() {
                std::fs::remove_file(&endpoint).ok();
            }
            let listener = tokio::net::UnixListener::bind(&endpoint)?;
            info!(endpoint = %endpoint.display(), "nm supervisor listening");
            self.accept_loop_unix(listener).await;
            Ok(())
        }
        #[cfg(windows)]
        {
            crate::supervisor_windows::run_windows_listener(self.config, self.sink).await
        }
    }

    #[cfg(unix)]
    async fn accept_loop_unix(self, listener: tokio::net::UnixListener) {
        loop {
            let permit = match self.conn_limit.clone().acquire_owned().await {
                Ok(p) => p,
                Err(_) => {
                    warn!("connection limiter closed");
                    return;
                }
            };
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let sink = self.sink.clone();
                    let mtu = self.config.mtu;
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, sink, mtu).await {
                            warn!(error = %e, "nm connection ended with error");
                        }
                        drop(permit);
                    });
                }
                Err(e) => {
                    warn!(error = %e, "nm accept failed; continuing");
                    drop(permit);
                }
            }
        }
    }
}

#[cfg(unix)]
async fn handle_connection(
    mut sock: tokio::net::UnixStream,
    sink: Arc<dyn SessionSink>,
    mtu: usize,
) -> DneaResult<()> {
    let preamble = read_frame(&mut sock).await?;
    let preamble: serde_json::Value = serde_json::from_slice(&preamble)
        .map_err(|e| DneaError::InvalidEnvelope(format!("bad preamble JSON: {e}")))?;
    let browser_family = preamble
        .get("browser_family").and_then(|v| v.as_str())
        .unwrap_or("unknown").to_string();
    let extension_id = preamble
        .get("extension_id").and_then(|v| v.as_str())
        .map(str::to_string);
    let bridge_pid = preamble
        .get("pid").and_then(|v| v.as_u64())
        .and_then(|n| u32::try_from(n).ok());

    let client_id = ClientId::new();
    let info = ClientInfo {
        browser_family: browser_family.clone(),
        extension_id,
        bridge_pid,
        connected_at: SystemTime::now(),
    };

    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(64);
    let (read_half, mut write_half) = sock.into_split();

    // Attach the client with its outbound sender before invoking
    // on_client_connected so the session can deliver Welcome through
    // the normal channel.
    if let Some(session) = sink.as_ref() as *const dyn SessionSink as *const _ as *const crate::session::Attachable {
        let _ = session; // fallback path; see note below
    }
    sink.on_client_connected(client_id, info)
        .await
        .map_err(NmHostError::SinkRejected)?;

    let writer = tokio::spawn(async move {
        while let Some(bytes) = rx.recv().await {
            if write_frame(&mut write_half, &bytes).await.is_err() {
                return;
            }
        }
    });

    let codec = Codec::new(mtu);
    let mut read_half = read_half;
    let result = read_loop(&mut read_half, &sink, client_id, &codec, &tx).await;

    drop(tx);
    let _ = writer.await;
    sink.on_client_disconnected(client_id).await;
    debug!(%client_id, "nm client disconnected");
    result
}

#[cfg(unix)]
async fn read_loop(
    read_half: &mut tokio::net::unix::OwnedReadHalf,
    sink: &Arc<dyn SessionSink>,
    client_id: ClientId,
    codec: &Codec,
    tx: &mpsc::Sender<Vec<u8>>,
) -> DneaResult<()> {
    loop {
        let frame = match read_frame_opt(read_half).await? {
            Some(f) => f,
            None => return Ok(()),
        };
        let decoded = match codec.decode(&frame) {
            Ok(d) => d,
            Err(e) => {
                warn!(%client_id, error = %e, "framing error");
                let err_env = Envelope {
                    v: PROTOCOL_VERSION,
                    id: EnvelopeId::new(),
                    kind: MessageType::Error,
                    ts: time::OffsetDateTime::now_utc(),
                    src: Party::Runtime,
                    dst: Party::Client,
                    reply_to: None,
                    cap: None,
                    module: None,
                    enc: dnea_core::Encoding::Identity,
                    ttl_ms: None,
                    payload: None,
                    err: Some(dnea_core::EnvelopeError {
                        code: e.code().to_string(),
                        message: e.to_string(),
                        retryable: false,
                        retry_after_ms: None,
                        details: None,
                        cause: None,
                    }),
                    meta: Default::default(),
                };
                let _ = send_envelope(codec, tx, &err_env).await;
                continue;
            }
        };
        let env = match decoded {
            Decoded::Incomplete => continue,
            Decoded::Envelope(e) => *e,
        };
        if let Err(e) = env.validate_direction(TransportDirection::Runtime) {
            warn!(%client_id, error = %e, "direction violation");
            continue;
        }
        match sink.handle_client_envelope(client_id, env).await {
            Ok(Some(reply)) => { send_envelope(codec, tx, &reply).await?; }
            Ok(None) => {}
            Err(e) => {
                let err_env = Envelope {
                    v: PROTOCOL_VERSION,
                    id: EnvelopeId::new(),
                    kind: MessageType::Error,
                    ts: time::OffsetDateTime::now_utc(),
                    src: Party::Runtime,
                    dst: Party::Client,
                    reply_to: None,
                    cap: None,
                    module: None,
                    enc: dnea_core::Encoding::Identity,
                    ttl_ms: None,
                    payload: None,
                    err: Some(dnea_core::EnvelopeError {
                        code: e.code().to_string(),
                        message: e.to_string(),
                        retryable: e.retryable(),
                        retry_after_ms: match e {
                            DneaError::RateLimited { retry_after_ms } => Some(retry_after_ms),
                            _ => None,
                        },
                        details: None,
                        cause: None,
                    }),
                    meta: Default::default(),
                };
                send_envelope(codec, tx, &err_env).await?;
            }
        }
    }
}

async fn send_envelope(
    codec: &Codec,
    tx: &mpsc::Sender<Vec<u8>>,
    env: &Envelope,
) -> DneaResult<()> {
    for frame in codec.encode(env)? {
        tx.send(frame).await.map_err(|_| DneaError::Transport)?;
    }
    Ok(())
}

async fn read_frame<R: AsyncReadExt + Unpin>(r: &mut R) -> DneaResult<Vec<u8>> {
    read_frame_opt(r).await?
        .ok_or_else(|| DneaError::Framing(dnea_core::FramingError::MissingEnvelope))
}

async fn read_frame_opt<R: AsyncReadExt + Unpin>(r: &mut R) -> DneaResult<Option<Vec<u8>>> {
    let mut len_buf = [0u8; 4];
    match r.read_exact(&mut len_buf).await {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(DneaError::Internal(format!("read length: {e}"))),
    }
    let len = u32::from_ne_bytes(len_buf) as usize;
    if len == 0 || len > MAX_FRAME_BYTES {
        return Err(DneaError::Framing(dnea_core::FramingError::TooLarge {
            actual: len, limit: MAX_FRAME_BYTES,
        }));
    }
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf).await
        .map_err(|e| DneaError::Internal(format!("read body: {e}")))?;
    Ok(Some(buf))
}

async fn write_frame<W: AsyncWriteExt + Unpin>(w: &mut W, bytes: &[u8]) -> DneaResult<()> {
    if bytes.is_empty() {
        return Err(DneaError::Internal("refusing to write empty frame".into()));
    }
    let len = u32::try_from(bytes.len())
        .map_err(|_| DneaError::Internal("frame too large for u32 prefix".into()))?;
    w.write_all(&len.to_ne_bytes()).await.map_err(|_| DneaError::Transport)?;
    w.write_all(bytes).await.map_err(|_| DneaError::Transport)?;
    w.flush().await.map_err(|_| DneaError::Transport)?;
    Ok(())
}