//! Windows named-pipe accept loop.

#![cfg(windows)]

use std::{
    path::PathBuf,
    sync::Arc,
};

use anyhow::{Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::ServerOptions;
use tokio::sync::mpsc;
use tracing::{debug, warn};

use dnea_core::{
    codec::{Codec, Decoded, DEFAULT_MTU},
    envelope::{Envelope, MessageType, Party, TransportDirection, PROTOCOL_VERSION},
    DneaError, DneaResult, EnvelopeId,
};

use crate::{
    session::{ClientId, ClientInfo, SessionSink},
    supervisor::NmHostConfig,
};

const MAX_FRAME_BYTES: usize = 64 * 1024 * 1024;

pub async fn run_windows_listener(
    config: NmHostConfig,
    sink: Arc<dyn SessionSink>,
) -> Result<(), crate::supervisor::NmHostError> {
    let pipe_name = pipe_name_from(&config.endpoint);
    tracing::info!(pipe = %pipe_name, "nm supervisor listening (Windows named pipe)");

    loop {
        let server = ServerOptions::new()
            .first_pipe_instance(false)
            .create(&pipe_name)
            .map_err(crate::supervisor::NmHostError::Io)?;

        server.connect().await.map_err(crate::supervisor::NmHostError::Io)?;

        let sink = sink.clone();
        let mtu = config.mtu;
        tokio::spawn(async move {
            if let Err(e) = handle_windows_client(server, sink, mtu).await {
                warn!(error = %e, "Windows NM client error");
            }
        });
    }
}

fn pipe_name_from(endpoint: &std::path::Path) -> String {
    let s = endpoint.to_string_lossy();
    if s.starts_with(r"\\.\pipe\") {
        return s.to_string();
    }
    let user = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
    format!(r"\\.\pipe\dnea-nm-{user}")
}

async fn handle_windows_client(
    mut pipe: tokio::net::windows::named_pipe::NamedPipeServer,
    sink: Arc<dyn SessionSink>,
    mtu: usize,
) -> DneaResult<()> {
    // Preamble.
    let preamble = read_frame(&mut pipe).await?;
    let preamble: serde_json::Value = serde_json::from_slice(&preamble)
        .map_err(|e| DneaError::InvalidEnvelope(format!("bad preamble JSON: {e}")))?;
    let browser_family = preamble.get("browser_family").and_then(|v| v.as_str())
        .unwrap_or("unknown").to_string();
    let extension_id = preamble.get("extension_id").and_then(|v| v.as_str()).map(str::to_string);
    let bridge_pid = preamble.get("pid").and_then(|v| v.as_u64())
        .and_then(|n| u32::try_from(n).ok());

    let client_id = ClientId::new();
    let info = ClientInfo {
        browser_family,
        extension_id,
        bridge_pid,
        connected_at: std::time::SystemTime::now(),
    };
    sink.on_client_connected(client_id, info).await
        .map_err(|e| DneaError::Internal(e.to_string()))?;

    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(64);
    let (mut read_half, mut write_half) = tokio::io::split(pipe);

    let writer = tokio::spawn(async move {
        while let Some(bytes) = rx.recv().await {
            if write_frame(&mut write_half, &bytes).await.is_err() { return; }
        }
    });

    let codec = Codec::new(mtu);
    loop {
        let frame = match read_frame_opt(&mut read_half).await? {
            Some(f) => f,
            None => break,
        };
        let decoded = match codec.decode(&frame) {
            Ok(d) => d,
            Err(e) => {
                warn!(%client_id, error = %e, "framing error");
                continue;
            }
        };
        let env = match decoded {
            Decoded::Incomplete => continue,
            Decoded::Envelope(e) => *e,
        };
        if let Err(e) = env.validate_direction(TransportDirection::Runtime) {
            debug!(%client_id, error = %e, "direction violation");
            continue;
        }
        match sink.handle_client_envelope(client_id, env).await {
            Ok(Some(reply)) => { send_envelope(&codec, &tx, &reply).await?; }
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
                        retry_after_ms: None,
                        details: None,
                        cause: None,
                    }),
                    meta: Default::default(),
                };
                let _ = send_envelope(&codec, &tx, &err_env).await;
            }
        }
    }

    drop(tx);
    let _ = writer.await;
    sink.on_client_disconnected(client_id).await;
    Ok(())
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
    r.read_exact(&mut buf).await.map_err(|e| DneaError::Internal(format!("read body: {e}")))?;
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