//! Client and module registries.

use std::collections::HashMap;
use std::sync::Arc;

use dnea_core::{ids::ModuleId, SessionId};
use dnea_manifest::Manifest;
use dnea_module_host::InstanceHandle;
use dnea_nm_host::session::ClientId;
use parking_lot::RwLock;
use tokio::sync::mpsc;

pub struct ClientRecord {
    pub id: ClientId,
    pub browser_family: String,
    pub extension_id: Option<String>,
    pub bridge_pid: Option<u32>,
    pub connected_at: std::time::SystemTime,
    pub tx: mpsc::Sender<Vec<u8>>,
}

pub struct ModuleRecord {
    pub id: ModuleId,
    pub version: String,
    pub handle: Arc<InstanceHandle>,
    pub manifest: Option<Arc<Manifest>>,
    pub loaded_at: std::time::SystemTime,
}

impl ModuleRecord {
    #[must_use]
    pub fn manifest(&self) -> Option<&Manifest> {
        self.manifest.as_deref()
    }
}

#[derive(Default)]
pub struct ClientRegistry {
    inner: RwLock<HashMap<ClientId, Arc<ClientRecord>>>,
    session_id: RwLock<Option<SessionId>>,
}

impl ClientRegistry {
    pub fn insert(&self, rec: Arc<ClientRecord>) {
        self.inner.write().insert(rec.id, rec);
    }

    pub fn remove(&self, id: ClientId) -> Option<Arc<ClientRecord>> {
        self.inner.write().remove(&id)
    }

    pub fn get(&self, id: ClientId) -> Option<Arc<ClientRecord>> {
        self.inner.read().get(&id).cloned()
    }

    pub fn all(&self) -> Vec<Arc<ClientRecord>> {
        self.inner.read().values().cloned().collect()
    }

    pub fn len(&self) -> usize { self.inner.read().len() }

    pub fn is_empty(&self) -> bool { self.inner.read().is_empty() }

    pub fn session_id(&self) -> SessionId {
        let mut g = self.session_id.write();
        *g.get_or_insert_with(SessionId::new)
    }
}

#[derive(Default)]
pub struct ModuleRegistry {
    inner: RwLock<HashMap<ModuleId, Arc<ModuleRecord>>>,
}

impl ModuleRegistry {
    pub fn insert(&self, rec: Arc<ModuleRecord>) {
        self.inner.write().insert(rec.id.clone(), rec);
    }

    pub fn remove(&self, id: &ModuleId) -> Option<Arc<ModuleRecord>> {
        self.inner.write().remove(id)
    }

    pub fn get(&self, id: &ModuleId) -> Option<Arc<ModuleRecord>> {
        self.inner.read().get(id).cloned()
    }

    pub fn all(&self) -> Vec<Arc<ModuleRecord>> {
        self.inner.read().values().cloned().collect()
    }

    pub fn len(&self) -> usize { self.inner.read().len() }

    pub fn is_empty(&self) -> bool { self.inner.read().is_empty() }
}