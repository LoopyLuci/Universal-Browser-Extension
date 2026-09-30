//! Host-side implementations of the DNEA module ABI.

use std::{sync::Arc, time::Duration};

use dnea_abi::dnea::host::{
    crypto, lifecycle, log as log_iface, net, schedule, storage, tabs, types, ui,
};
use dnea_core::{DneaError, DneaResult};
use tracing::{debug, error, info, trace, warn};

use crate::grant::GrantSet;

#[async_trait::async_trait]
pub trait TabSource: Send + Sync + 'static {
    async fn query(&self, filter_json: &str) -> DneaResult<Vec<TabInfo>>;
    async fn update(&self, tab_id: u64, props_json: &str) -> DneaResult<()>;
    async fn create(&self, props_json: &str) -> DneaResult<TabInfo>;
    async fn close(&self, tab_id: u64) -> DneaResult<()>;
}

#[derive(Clone, Debug)]
pub struct TabInfo {
    pub id: u64,
    pub url: String,
    pub title: String,
    pub active: bool,
    pub window_id: u64,
}

#[async_trait::async_trait]
pub trait ModuleStore: Send + Sync + 'static {
    async fn get(&self, module: &str, key: &str) -> DneaResult<Option<Vec<u8>>>;
    async fn put(&self, module: &str, key: &str, value: &[u8]) -> DneaResult<()>;
    async fn delete(&self, module: &str, key: &str) -> DneaResult<()>;
    async fn list(&self, module: &str, prefix: &str) -> DneaResult<Vec<String>>;
}

#[async_trait::async_trait]
pub trait UiSink: Send + Sync + 'static {
    async fn set_panel_html(&self, panel_id: &str, html: &str) -> DneaResult<()>;
    async fn notify(&self, title: &str, body: &str) -> DneaResult<()>;
}

#[async_trait::async_trait]
pub trait CryptoSigner: Send + Sync + 'static {
    async fn sign(&self, key_id: &str, data: &[u8]) -> DneaResult<Vec<u8>>;
    async fn verify(&self, key_id: &str, data: &[u8], sig: &[u8]) -> DneaResult<bool>;
}

pub struct HostState {
    pub module_id: String,
    pub module_version: String,
    pub grants: Arc<GrantSet>,
    pub tabs: Option<Arc<dyn TabSource>>,
    pub store: Option<Arc<dyn ModuleStore>>,
    pub ui: Option<Arc<dyn UiSink>>,
    pub crypto: Option<Arc<dyn CryptoSigner>>,
    pub limits: wasmtime::StoreLimits,
}

impl HostState {
    #[must_use]
    pub fn new(
        module_id: String,
        module_version: String,
        grants: Arc<GrantSet>,
        limits: wasmtime::StoreLimits,
    ) -> Self {
        Self {
            module_id, module_version, grants,
            tabs: None, store: None, ui: None, crypto: None, limits,
        }
    }

    fn check(&self, capability: &str) -> Result<(), types::HostError> {
        if self.grants.get(capability).is_some() {
            Ok(())
        } else if self.grants.is_denied(capability) {
            Err(types::HostError::Denied(capability.to_string()))
        } else {
            Err(types::HostError::NotGranted(capability.to_string()))
        }
    }
}

impl log_iface::Host for HostState {
    async fn emit(
        &mut self,
        level: types::LogLevel,
        message: String,
        fields_json: Option<String>,
    ) {
        let fields = fields_json.unwrap_or_default();
        match level {
            types::LogLevel::Trace => trace!(module = %self.module_id, fields, "{message}"),
            types::LogLevel::Debug => debug!(module = %self.module_id, fields, "{message}"),
            types::LogLevel::Info  => info! (module = %self.module_id, fields, "{message}"),
            types::LogLevel::Warn  => warn! (module = %self.module_id, fields, "{message}"),
            types::LogLevel::Error => error!(module = %self.module_id, fields, "{message}"),
        }
    }
}

impl storage::Host for HostState {
    async fn get(&mut self, key: String) -> Result<Option<Vec<u8>>, types::HostError> {
        self.check("storage:module")?;
        let store = self.store.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("storage backend not wired".into()))?;
        store.get(&self.module_id, &key).await.map_err(map_err)
    }

    async fn put(&mut self, key: String, value: Vec<u8>) -> Result<(), types::HostError> {
        self.check("storage:module")?;
        let store = self.store.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("storage backend not wired".into()))?;
        store.put(&self.module_id, &key, &value).await.map_err(map_err)
    }

    async fn delete(&mut self, key: String) -> Result<(), types::HostError> {
        self.check("storage:module")?;
        let store = self.store.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("storage backend not wired".into()))?;
        store.delete(&self.module_id, &key).await.map_err(map_err)
    }

    async fn list(&mut self, prefix: String) -> Result<Vec<String>, types::HostError> {
        self.check("storage:module")?;
        let store = self.store.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("storage backend not wired".into()))?;
        store.list(&self.module_id, &prefix).await.map_err(map_err)
    }
}

impl schedule::Host for HostState {
    async fn after(&mut self, _delay_ms: u64, _callback_id: String) -> Result<u64, types::HostError> {
        Ok(next_handle())
    }
    async fn every(&mut self, _interval_ms: u64, _callback_id: String) -> Result<u64, types::HostError> {
        Ok(next_handle())
    }
    async fn cancel(&mut self, _handle: u64) -> Result<(), types::HostError> { Ok(()) }
}

fn next_handle() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

impl tabs::Host for HostState {
    async fn query(&mut self, filter_json: String) -> Result<Vec<tabs::Tab>, types::HostError> {
        self.check("browser:tabs:query")?;
        let src = self.tabs.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("tabs backend not wired".into()))?;
        let rows = src.query(&filter_json).await.map_err(map_err)?;
        Ok(rows.into_iter().map(to_wit_tab).collect())
    }
    async fn update(&mut self, tab_id: u64, props_json: String) -> Result<(), types::HostError> {
        self.check("browser:tabs:update")?;
        let src = self.tabs.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("tabs backend not wired".into()))?;
        src.update(tab_id, &props_json).await.map_err(map_err)
    }
    async fn create(&mut self, props_json: String) -> Result<tabs::Tab, types::HostError> {
        self.check("browser:tabs:create")?;
        let src = self.tabs.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("tabs backend not wired".into()))?;
        let tab = src.create(&props_json).await.map_err(map_err)?;
        Ok(to_wit_tab(tab))
    }
    async fn close(&mut self, tab_id: u64) -> Result<(), types::HostError> {
        self.check("browser:tabs:close")?;
        let src = self.tabs.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("tabs backend not wired".into()))?;
        src.close(tab_id).await.map_err(map_err)
    }
}

fn to_wit_tab(t: TabInfo) -> tabs::Tab {
    tabs::Tab { id: t.id, url: t.url, title: t.title, active: t.active, window_id: t.window_id }
}

impl net::Host for HostState {
    async fn request(&mut self, _req_json: String) -> Result<Vec<u8>, types::HostError> {
        self.check("net:connect")?;
        Err(types::HostError::Unavailable(
            "net:connect is not implemented in this build".into(),
        ))
    }
}

impl ui::Host for HostState {
    async fn set_panel_html(&mut self, panel_id: String, html: String) -> Result<(), types::HostError> {
        self.check("ui:companion:panel")?;
        let sink = self.ui.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("ui sink not wired".into()))?;
        sink.set_panel_html(&panel_id, &html).await.map_err(map_err)
    }
    async fn notify(&mut self, title: String, body: String) -> Result<(), types::HostError> {
        self.check("ui:companion:notify")?;
        let sink = self.ui.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("ui sink not wired".into()))?;
        sink.notify(&title, &body).await.map_err(map_err)
    }
}

impl crypto::Host for HostState {
    async fn sign(&mut self, key_id: String, data: Vec<u8>) -> Result<Vec<u8>, types::HostError> {
        self.check("identity:crypto:sign")?;
        let signer = self.crypto.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("crypto signer not wired".into()))?;
        signer.sign(&key_id, &data).await.map_err(map_err)
    }
    async fn verify(&mut self, key_id: String, data: Vec<u8>, sig: Vec<u8>) -> Result<bool, types::HostError> {
        self.check("identity:crypto:verify")?;
        let signer = self.crypto.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("crypto signer not wired".into()))?;
        signer.verify(&key_id, &data, &sig).await.map_err(map_err)
    }
}

impl lifecycle::Host for HostState {
    async fn activate(&mut self) -> Result<(), types::HostError> {
        Err(types::HostError::Internal("lifecycle.activate is a module export".into()))
    }
    async fn deactivate(&mut self) -> Result<(), types::HostError> {
        Err(types::HostError::Internal("lifecycle.deactivate is a module export".into()))
    }
    async fn upgrade(&mut self, _from_version: String, _from_storage_schema: u32) -> Result<(), types::HostError> {
        Err(types::HostError::Internal("lifecycle.upgrade is a module export".into()))
    }
    async fn health_check(&mut self) -> Result<(), types::HostError> {
        Err(types::HostError::Internal("lifecycle.health-check is a module export".into()))
    }
}

fn map_err(e: DneaError) -> types::HostError {
    match e {
        DneaError::CapabilityDenied { capability, .. } => types::HostError::NotGranted(capability),
        DneaError::ModuleUnavailable(s) => types::HostError::Unavailable(s),
        DneaError::Timeout(ms) => types::HostError::Timeout(ms),
        DneaError::RateLimited { retry_after_ms } => {
            types::HostError::RateLimited(u32::try_from(retry_after_ms).unwrap_or(u32::MAX))
        }
        DneaError::Internal(s) => types::HostError::Internal(s),
        other => types::HostError::Internal(other.to_string()),
    }
}

const _: Duration = Duration::from_secs(0);