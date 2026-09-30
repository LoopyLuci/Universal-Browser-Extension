//! The central hub.

use std::sync::Arc;
use std::time::Duration;

use dnea_core::{
    envelope::{Envelope, MessageType, Party},
    ids::ModuleId,
    DneaError, DneaResult, SessionId,
};
use dnea_module_host::grant::GrantSet;
use dnea_nm_host::session::{ClientId, ClientInfo, SessionSink};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{
    consent::{ConsentDecision, ConsentRequest, ConsentState, RememberFor},
    grants::{GrantChange, GrantStore, GrantUpdate, OriginGrant},
    registry::{ClientRecord, ClientRegistry, ModuleRecord, ModuleRegistry},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "topic", content = "payload", rename_all = "snake_case")]
pub enum SessionEvent {
    ModuleLoaded { module: String, version: String },
    ModuleUnloaded { module: String },
    GrantChanged(GrantUpdate),
    ConsentOpened(ConsentRequest),
    ConsentResolved { prompt_id: Uuid, granted: bool },
    ClientConnected { client_id: String, browser_family: String, extension_id: Option<String> },
    ClientDisconnected { client_id: String },
    OriginGranted { module: String, capability: String, origin: String },
    OriginRevoked { module: String, capability: String, origin: String },
}

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("module not loaded: {0}")]
    ModuleNotLoaded(String),
    #[error("session shutting down")]
    ShuttingDown,
    #[error("internal: {0}")]
    Internal(String),
}

#[derive(Clone)]
pub struct Session { inner: Arc<SessionInner> }

struct SessionInner {
    clients: ClientRegistry,
    modules: ModuleRegistry,
    grants: GrantStore,
    consent: ConsentState,
    events: broadcast::Sender<SessionEvent>,
}

pub type SessionHandle = Session;

impl Session {
    #[must_use]
    pub fn new() -> Self {
        let (events, _) = broadcast::channel(1024);
        Self {
            inner: Arc::new(SessionInner {
                clients: ClientRegistry::default(),
                modules: ModuleRegistry::default(),
                grants: GrantStore::default(),
                consent: ConsentState::default(),
                events,
            }),
        }
    }

    pub fn clients(&self) -> Vec<Arc<ClientRecord>> { self.inner.clients.all() }
    pub fn modules(&self) -> Vec<Arc<ModuleRecord>> { self.inner.modules.all() }
    pub fn module(&self, id: &ModuleId) -> Option<Arc<ModuleRecord>> { self.inner.modules.get(id) }
    pub fn grants(&self, id: &ModuleId) -> Option<GrantSet> { self.inner.grants.get(id) }
    pub fn grant_store(&self) -> &GrantStore { &self.inner.grants }
    pub fn session_id(&self) -> SessionId { self.inner.clients.session_id() }
    pub fn subscribe(&self) -> broadcast::Receiver<SessionEvent> { self.inner.events.subscribe() }
    pub fn open_consents(&self) -> Vec<ConsentRequest> { self.inner.consent.list() }
    pub fn consent(&self, id: &Uuid) -> Option<ConsentRequest> { self.inner.consent.get(id) }

    pub fn set_grants(&self, module: &ModuleId, grants: Arc<GrantSet>) {
        self.inner.grants.set(module.clone(), (*grants).clone());
    }

    pub fn grants_arc(&self, module: &ModuleId) -> Option<Arc<GrantSet>> {
        self.inner.grants.get(module).map(Arc::new)
    }

    pub async fn request_consent(&self, request: ConsentRequest) -> DneaResult<ConsentDecision> {
        let rx = self.inner.consent.open(request.clone());
        self.broadcast(SessionEvent::ConsentOpened(request));
        rx.await.map_err(|_| DneaError::Internal("consent waiter dropped".into()))
    }

    pub fn resolve_consent(&self, decision: ConsentDecision) -> DneaResult<()> {
        let req = self.inner.consent.resolve(decision.clone())
            .ok_or_else(|| DneaError::Internal(format!("no such prompt: {}", decision.prompt_id)))?;
        self.broadcast(SessionEvent::ConsentResolved {
            prompt_id: decision.prompt_id,
            granted: decision.granted,
        });
        if decision.granted {
            let module = ModuleId::parse(&req.module)
                .map_err(|e| DneaError::Internal(e.to_string()))?;
            if let Some(update) = self.inner.grants.grant(
                &module, &req.capability,
                decision.scope.unwrap_or(req.constraints),
                dnea_module_host::grant::GrantSource::UserConsent,
            ) {
                self.broadcast(SessionEvent::GrantChanged(update));
            }
        }
        Ok(())
    }

    pub fn register_module(&self, record: Arc<ModuleRecord>) {
        let id = record.id.clone();
        let version = record.version.clone();
        if self.inner.grants.get(&id).is_none() {
            self.inner.grants.set(id.clone(), GrantSet::new());
        }
        self.inner.modules.insert(record);
        self.broadcast(SessionEvent::ModuleLoaded { module: id.to_string(), version });
    }

    pub fn unregister_module(&self, id: &ModuleId) {
        if self.inner.modules.remove(id).is_some() {
            self.broadcast(SessionEvent::ModuleUnloaded { module: id.to_string() });
        }
    }

    pub async fn send_to_client_by_id(&self, client_id: ClientId, env: Envelope) -> DneaResult<()> {
        let rec = self.inner.clients.get(client_id).ok_or(DneaError::Transport)?;
        let codec = dnea_core::Codec::default_mtu();
        for f in codec.encode(&env)? {
            rec.tx.send(f).await.map_err(|_| DneaError::Transport)?;
        }
        Ok(())
    }

    pub async fn broadcast_to_clients(&self, env: Envelope) -> DneaResult<()> {
        let codec = dnea_core::Codec::default_mtu();
        for rec in self.inner.clients.all() {
            if let Ok(frames) = codec.encode(&env) {
                for f in frames {
                    let _ = rec.tx.send(f).await;
                }
            }
        }
        Ok(())
    }

    pub fn broadcast_origin_granted(&self, grant: &OriginGrant) {
        self.broadcast(SessionEvent::OriginGranted {
            module: grant.module.clone(),
            capability: grant.capability.clone(),
            origin: grant.origin.clone(),
        });
    }

    pub fn broadcast_origin_revoked(&self, module: &str, capability: &str, origin: &str) {
        self.broadcast(SessionEvent::OriginRevoked {
            module: module.to_string(),
            capability: capability.to_string(),
            origin: origin.to_string(),
        });
    }

    pub async fn sweep_consents(&self, ttl: Duration) {
        self.inner.consent.sweep(ttl);
    }

    pub async fn shutdown(&self) {
        let env = Envelope::new(MessageType::Goodbye, Party::Runtime, Party::Client);
        self.broadcast_to_clients(env).await.ok();
    }

    pub fn attach_client(
        &self,
        client_id: ClientId,
        info: ClientInfo,
        tx: mpsc::Sender<Vec<u8>>,
    ) {
        self.inner.clients.insert(Arc::new(ClientRecord {
            id: client_id,
            browser_family: info.browser_family,
            extension_id: info.extension_id,
            bridge_pid: info.bridge_pid,
            connected_at: info.connected_at,
            tx,
        }));
    }

    fn broadcast(&self, event: SessionEvent) {
        let _ = self.inner.events.send(event);
    }
}

impl Default for Session {
    fn default() -> Self { Self::new() }
}

#[async_trait::async_trait]
impl SessionSink for Session {
    async fn on_client_connected(
        &self,
        client_id: ClientId,
        info: ClientInfo,
    ) -> DneaResult<SessionId> {
        if self.inner.clients.get(client_id).is_none() {
            return Err(DneaError::Internal(
                "client was not attached before on_client_connected".into(),
            ));
        }
        info!(%client_id, browser = %info.browser_family, "client connected");
        self.broadcast(SessionEvent::ClientConnected {
            client_id: client_id.to_string(),
            browser_family: info.browser_family.clone(),
            extension_id: info.extension_id.clone(),
        });
        Ok(self.inner.clients.session_id())
    }

    async fn on_client_disconnected(&self, client_id: ClientId) {
        self.inner.clients.remove(client_id);
        self.broadcast(SessionEvent::ClientDisconnected {
            client_id: client_id.to_string(),
        });
    }

    async fn handle_client_envelope(
        &self,
        client_id: ClientId,
        env: Envelope,
    ) -> DneaResult<Option<Envelope>> {
        debug!(%client_id, kind = %env.kind.as_str(), "client envelope");
        match env.kind {
            MessageType::Hello => {
                let mut w = env.reply(MessageType::Welcome);
                w.payload = Some(serde_json::json!({
                    "runtimeVersion": env!("CARGO_PKG_VERSION"),
                    "protocolVersion": dnea_core::PROTOCOL_VERSION,
                    "sessionId": self.inner.clients.session_id().to_string(),
                    "mtu": dnea_core::DEFAULT_MTU,
                    "encodings": ["identity"],
                    "features": { "chunking": true, "consentUI": true },
                }));
                let ext_id = env.payload.as_ref()
                    .and_then(|p| p.get("extensionId"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                w.meta.insert("client_extension_id".into(), serde_json::json!(ext_id));
                Ok(Some(w))
            }
            MessageType::Ping => Ok(Some(env.reply(MessageType::Pong))),
            MessageType::Goodbye => Ok(None),
            MessageType::Invoke => {
                let module_id = match &env.module {
                    Some(m) => m.clone(),
                    None => return Ok(Some(Envelope::from_error(
                        &env, &DneaError::InvalidEnvelope("invoke without module".into())))),
                };
                let Some(record) = self.inner.modules.get(&module_id) else {
                    return Ok(Some(Envelope::from_error(
                        &env, &DneaError::ModuleUnavailable(module_id.to_string()))));
                };
                let method = env.meta.get("method").and_then(|v| v.as_str())
                    .unwrap_or("").to_string();
                let args = env.payload.as_ref()
                    .map(|v| serde_json::to_vec(v).unwrap_or_default())
                    .unwrap_or_default();
                let result = record.handle.invoke(&method, args).await;
                let mut reply = env.reply(MessageType::Result);
                match result {
                    Ok(bytes) => reply.payload = serde_json::from_slice(&bytes).ok(),
                    Err(e) => return Ok(Some(Envelope::from_error(&env, &e))),
                }
                Ok(Some(reply))
            }
            MessageType::ConsentResponse => {
                let decision: ConsentDecision = serde_json::from_value(
                    env.payload.clone().unwrap_or_default(),
                ).map_err(|e| DneaError::InvalidEnvelope(e.to_string()))?;
                self.resolve_consent(decision)?;
                Ok(Some(env.reply(MessageType::Result)))
            }
            MessageType::CapabilityList => {
                let mut out = env.reply(MessageType::CapabilityListResult);
                let grants: Vec<_> = self.inner.modules.all().into_iter()
                    .flat_map(|m| {
                        let set = self.inner.grants.get(&m.id);
                        let id = m.id.clone();
                        set.into_iter().flat_map(move |s| {
                            s.iter().map(|(cap, g)| serde_json::json!({
                                "module": id.to_string(),
                                "capability": cap,
                                "constraints": g.constraints,
                                "source": g.source,
                            })).collect::<Vec<_>>()
                        })
                    })
                    .collect();
                out.payload = Some(serde_json::json!({ "grants": grants }));
                Ok(Some(out))
            }
            _ => Ok(Some(env.reply(MessageType::Result))),
        }
    }

    async fn send_to_client(&self, client_id: ClientId, env: Envelope) -> DneaResult<()> {
        self.send_to_client_by_id(client_id, env).await
    }
}

const _: Option<RememberFor> = None;
const _: Option<GrantChange> = None;