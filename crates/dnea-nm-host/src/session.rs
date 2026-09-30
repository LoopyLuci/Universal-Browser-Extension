//! The interface the NM supervisor uses to talk to the runtime core.

use async_trait::async_trait;
use dnea_core::{DneaResult, Envelope, SessionId};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClientId(Uuid);

impl ClientId {
    #[must_use]
    pub fn new() -> Self { Self(Uuid::now_v7()) }
    #[must_use]
    pub fn as_uuid(&self) -> Uuid { self.0 }
}

impl Default for ClientId {
    fn default() -> Self { Self::new() }
}

impl std::fmt::Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct ClientInfo {
    pub browser_family: String,
    pub extension_id: Option<String>,
    pub bridge_pid: Option<u32>,
    pub connected_at: std::time::SystemTime,
}

#[async_trait]
pub trait SessionSink: Send + Sync + 'static {
    async fn on_client_connected(
        &self,
        client_id: ClientId,
        info: ClientInfo,
    ) -> DneaResult<SessionId>;

    async fn on_client_disconnected(&self, client_id: ClientId);

    async fn handle_client_envelope(
        &self,
        client_id: ClientId,
        env: Envelope,
    ) -> DneaResult<Option<Envelope>>;

    async fn send_to_client(
        &self,
        _client_id: ClientId,
        _env: Envelope,
    ) -> DneaResult<()> {
        Err(dnea_core::DneaError::UnsupportedFeature(
            "this SessionSink does not support server-initiated messages".into(),
        ))
    }
}