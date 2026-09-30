//! Consent state.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RememberFor { Session, ModuleVersion, Always }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsentRequest {
    pub prompt_id: Uuid,
    pub module: String,
    pub module_name: String,
    pub capability: String,
    pub tier: String,
    pub reason: String,
    pub constraints: serde_json::Value,
    pub target_description: Option<String>,
    pub expires_at: SystemTime,
    pub can_remember: Vec<RememberFor>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsentDecision {
    pub prompt_id: Uuid,
    pub granted: bool,
    #[serde(default)]
    pub scope: Option<serde_json::Value>,
    #[serde(default)]
    pub remember_for: Option<RememberFor>,
}

#[derive(Default)]
pub struct ConsentState {
    open: RwLock<HashMap<Uuid, OpenPrompt>>,
}

struct OpenPrompt {
    request: ConsentRequest,
    created_at: SystemTime,
    waiter: Option<oneshot::Sender<ConsentDecision>>,
}

impl ConsentState {
    pub fn open(&self, request: ConsentRequest) -> oneshot::Receiver<ConsentDecision> {
        let (tx, rx) = oneshot::channel();
        let mut g = self.open.write();
        g.insert(request.prompt_id, OpenPrompt {
            request,
            created_at: SystemTime::now(),
            waiter: Some(tx),
        });
        rx
    }

    pub fn resolve(&self, decision: ConsentDecision) -> Option<ConsentRequest> {
        let mut g = self.open.write();
        let mut entry = g.remove(&decision.prompt_id)?;
        if let Some(tx) = entry.waiter.take() {
            let _ = tx.send(decision);
        }
        Some(entry.request)
    }

    pub fn list(&self) -> Vec<ConsentRequest> {
        self.open.read().values().map(|p| p.request.clone()).collect()
    }

    pub fn get(&self, id: &Uuid) -> Option<ConsentRequest> {
        self.open.read().get(id).map(|p| p.request.clone())
    }

    pub fn sweep(&self, ttl: Duration) {
        let now = SystemTime::now();
        let mut g = self.open.write();
        let expired: Vec<Uuid> = g.iter()
            .filter(|(_, p)| {
                now.duration_since(p.created_at).map(|d| d > ttl).unwrap_or(false)
            })
            .map(|(id, _)| *id)
            .collect();
        for id in expired {
            if let Some(mut p) = g.remove(&id) {
                if let Some(tx) = p.waiter.take() {
                    let _ = tx.send(ConsentDecision {
                        prompt_id: id,
                        granted: false,
                        scope: None,
                        remember_for: None,
                    });
                }
            }
        }
    }
}