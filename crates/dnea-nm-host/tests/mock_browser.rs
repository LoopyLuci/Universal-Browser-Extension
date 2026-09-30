//! End-to-end test against the real bridge binary and a real supervisor.

use std::{path::PathBuf, sync::Arc, time::Duration};

use async_trait::async_trait;
use dnea_core::{
    envelope::{Envelope, MessageType, Party, PROTOCOL_VERSION},
    DneaResult, SessionId,
};
use dnea_nm_host::{
    session::{ClientId, ClientInfo, SessionSink},
    supervisor::{NmHostConfig, NmSupervisor},
};
use serde_json::json;
use tempfile::TempDir;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{Child, Command},
};

struct EchoSink { session_id: SessionId }

impl EchoSink {
    fn new() -> Self { Self { session_id: SessionId::new() } }
}

#[async_trait]
impl SessionSink for EchoSink {
    async fn on_client_connected(&self, _: ClientId, _: ClientInfo) -> DneaResult<SessionId> {
        Ok(self.session_id)
    }
    async fn on_client_disconnected(&self, _: ClientId) {}
    async fn handle_client_envelope(
        &self, _: ClientId, env: Envelope,
    ) -> DneaResult<Option<Envelope>> {
        match env.kind {
            MessageType::Hello => {
                let mut w = env.reply(MessageType::Welcome);
                w.payload = Some(json!({
                    "runtimeVersion": "test",
                    "protocolVersion": PROTOCOL_VERSION,
                    "sessionId": self.session_id.to_string(),
                    "mtu": 1_048_576,
                    "encodings": ["identity"],
                    "features": { "chunking": true },
                }));
                let ext_id = env.payload.as_ref()
                    .and_then(|p| p.get("extensionId"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                w.meta.insert("client_extension_id".into(), json!(ext_id));
                Ok(Some(w))
            }
            _ => {
                let mut r = env.reply(MessageType::Result);
                r.payload = env.payload;
                Ok(Some(r))
            }
        }
    }
}

struct MockBrowser {
    child: Child,
    stdin: tokio::process::ChildStdin,
    stdout: tokio::process::ChildStdout,
}

impl MockBrowser {
    async fn spawn(bridge_path: &PathBuf, socket: &PathBuf) -> std::io::Result<Self> {
        let mut child = Command::new(bridge_path)
            .env("DNEA_SOCKET", socket)
            .env("DNEA_BROWSER_FAMILY", "chrome")
            .env("DNEA_EXTENSION_ID", "abcdefghijklmnopabcdefghijklmnop")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;
        let stdin = child.stdin.take().expect("piped stdin");
        let stdout = child.stdout.take().expect("piped stdout");
        Ok(Self { child, stdin, stdout })
    }

    async fn send(&mut self, env: &Envelope) -> std::io::Result<()> {
        let bytes = serde_json::to_vec(env).unwrap();
        let len = u32::try_from(bytes.len()).unwrap();
        self.stdin.write_all(&len.to_ne_bytes()).await?;
        self.stdin.write_all(&bytes).await?;
        self.stdin.flush().await?;
        Ok(())
    }

    async fn recv(&mut self) -> std::io::Result<Envelope> {
        let mut len_buf = [0u8; 4];
        self.stdout.read_exact(&mut len_buf).await?;
        let len = u32::from_ne_bytes(len_buf) as usize;
        let mut buf = vec![0u8; len];
        self.stdout.read_exact(&mut buf).await?;
        let env: Envelope = serde_json::from_slice(&buf).unwrap();
        Ok(env)
    }

    async fn kill(&mut self) { let _ = self.child.kill().await; }
}

fn bridge_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_dnea-nm-bridge"))
}

#[tokio::test(flavor = "multi_thread")]
async fn t01_hello_welcome() {
    let tmp = TempDir::new().unwrap();
    let socket = tmp.path().join("nm.sock");

    let sink = Arc::new(EchoSink::new());
    let config = NmHostConfig::new(socket.clone());
    let supervisor = NmSupervisor::new(config, sink.clone());
    let sup_task = tokio::spawn(supervisor.run());
    tokio::time::sleep(Duration::from_millis(100)).await;

    let bridge = bridge_binary();
    let mut browser = MockBrowser::spawn(&bridge, &socket).await.unwrap();

    let mut hello = Envelope::new(MessageType::Hello, Party::Client, Party::Runtime);
    hello.payload = Some(json!({
        "clientVersion": "1.0.0",
        "browserFamily": "chromium",
        "browserVersion": "test",
        "extensionId": "abcdefghijklmnopabcdefghijklmnop",
        "mtu": 1_048_576,
        "encodings": ["identity"],
        "features": { "chunking": true },
    }));
    browser.send(&hello).await.unwrap();

    let welcome = tokio::time::timeout(Duration::from_secs(5), browser.recv())
        .await.expect("welcome within 5s").unwrap();
    assert_eq!(welcome.kind, MessageType::Welcome);
    assert_eq!(welcome.reply_to, Some(hello.id));
    assert_eq!(
        welcome.meta.get("client_extension_id").and_then(|v| v.as_str()),
        Some("abcdefghijklmnopabcdefghijklmnop"),
    );

    browser.kill().await;
    sup_task.abort();
}

#[tokio::test(flavor = "multi_thread")]
async fn t12_reconnect_after_restart() {
    let tmp = TempDir::new().unwrap();
    let socket = tmp.path().join("nm.sock");

    let sink = Arc::new(EchoSink::new());
    let config = NmHostConfig::new(socket.clone());
    let supervisor = NmSupervisor::new(config.clone(), sink.clone());
    let sup_task = tokio::spawn(supervisor.run());
    tokio::time::sleep(Duration::from_millis(100)).await;

    let bridge = bridge_binary();
    let mut browser = MockBrowser::spawn(&bridge, &socket).await.unwrap();
    let mut hello = Envelope::new(MessageType::Hello, Party::Client, Party::Runtime);
    hello.payload = Some(json!({
        "clientVersion": "1.0.0",
        "browserFamily": "chromium",
        "extensionId": "abcdefghijklmnopabcdefghijklmnop",
        "mtu": 1_048_576,
    }));
    browser.send(&hello).await.unwrap();
    let _welcome = browser.recv().await.unwrap();

    sup_task.abort();
    tokio::time::sleep(Duration::from_millis(100)).await;
    browser.kill().await;
    let _ = browser.child.wait().await;

    let supervisor2 = NmSupervisor::new(config, sink.clone());
    let sup_task2 = tokio::spawn(supervisor2.run());
    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut browser2 = MockBrowser::spawn(&bridge, &socket).await.unwrap();
    let mut hello2 = Envelope::new(MessageType::Hello, Party::Client, Party::Runtime);
    hello2.payload = Some(json!({
        "clientVersion": "1.0.0",
        "browserFamily": "chromium",
        "extensionId": "abcdefghijklmnopabcdefghijklmnop",
        "mtu": 1_048_576,
    }));
    browser2.send(&hello2).await.unwrap();
    let welcome2 = tokio::time::timeout(Duration::from_secs(5), browser2.recv())
        .await.expect("reconnect welcome").unwrap();
    assert_eq!(welcome2.kind, MessageType::Welcome);
    assert_eq!(welcome2.reply_to, Some(hello2.id));

    browser2.kill().await;
    sup_task2.abort();
}