//! Instance lifecycle and message routing.

use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use dnea_abi::dnea::host::lifecycle::Lifecycle;
use dnea_abi::dnea::host::module_api::ModuleApi;
use dnea_core::{DneaError, DneaResult};
use tokio::sync::{mpsc, oneshot};
use tracing::{info, warn};
use wasmtime::{
    component::{Component, Linker},
    Store, StoreLimitsBuilder,
};

use crate::{
    engine::{build_engine, EngineConfig},
    grant::GrantSet,
    host::HostState,
    link::build_linker,
};

#[derive(Debug, thiserror::Error)]
pub enum ModuleHostError {
    #[error("compile error: {0}")]
    Compile(String),
    #[error("instantiate error: {0}")]
    Instantiate(String),
    #[error("lifecycle {hook} failed: {message}")]
    Lifecycle { hook: String, message: String },
    #[error("instance task stopped")]
    Stopped,
}

pub enum InstanceMessage {
    Invoke {
        method: String,
        args: Vec<u8>,
        reply: oneshot::Sender<DneaResult<Vec<u8>>>,
    },
    Event { kind: String, data: Vec<u8> },
    Shutdown(oneshot::Sender<()>),
}

pub struct InstanceHandle {
    pub id: String,
    pub version: String,
    pub grants: Arc<GrantSet>,
    tx: mpsc::Sender<InstanceMessage>,
    join: Option<tokio::task::JoinHandle<()>>,
}

impl InstanceHandle {
    pub async fn invoke(&self, method: &str, args: Vec<u8>) -> DneaResult<Vec<u8>> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(InstanceMessage::Invoke { method: method.to_string(), args, reply: tx })
            .await
            .map_err(|_| DneaError::ModuleUnavailable(self.id.clone()))?;
        rx.await
            .map_err(|_| DneaError::ModuleUnavailable(self.id.clone()))?
    }

    pub async fn event(&self, kind: &str, data: Vec<u8>) -> DneaResult<()> {
        self.tx
            .send(InstanceMessage::Event { kind: kind.to_string(), data })
            .await
            .map_err(|_| DneaError::ModuleUnavailable(self.id.clone()))
    }

    pub async fn shutdown(mut self) {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(InstanceMessage::Shutdown(tx)).await;
        let _ = tokio::time::timeout(Duration::from_secs(5), rx).await;
        if let Some(join) = self.join.take() { join.abort(); }
    }

    /// Constructs a handle for tests. Never invokes any method.
    #[doc(hidden)]
    pub fn new_for_test(id: String, version: String, grants: Arc<GrantSet>) -> Self {
        let (tx, _rx) = mpsc::channel(1);
        Self { id, version, grants, tx, join: None }
    }
}

pub struct ModuleHost {
    engine: wasmtime::Engine,
}

impl ModuleHost {
    pub fn new(cfg: &EngineConfig) -> Result<Self> {
        let engine = build_engine(cfg)?;
        Ok(Self { engine })
    }

    #[must_use]
    pub fn engine_clone(&self) -> wasmtime::Engine { self.engine.clone() }

    pub async fn load(
        &self,
        module_id: String,
        module_version: String,
        component_path: PathBuf,
        grants: GrantSet,
        host_state_builder: impl FnOnce(Arc<GrantSet>) -> HostState,
    ) -> Result<InstanceHandle, ModuleHostError> {
        let component = Component::from_file(&self.engine, &component_path)
            .with_context(|| format!("load {}", component_path.display()))
            .map_err(|e| ModuleHostError::Compile(e.to_string()))?;

        let grants = Arc::new(grants);
        let linker: Linker<HostState> = build_linker(&self.engine, &grants)
            .map_err(|e| ModuleHostError::Instantiate(format!("linker: {e}")))?;

        let mut host_state = host_state_builder(grants.clone());
        host_state.limits = StoreLimitsBuilder::new()
            .memory_size(64 * 1024 * 1024)
            .instances(1)
            .memories(4)
            .tables(4)
            .build();

        let mut store = Store::new(&self.engine, host_state);
        store.limiter(|s| &mut s.limits);
        store.set_fuel(10_000_000)
            .map_err(|e| ModuleHostError::Instantiate(format!("fuel: {e}")))?;
        store.set_epoch_deadline(1);

        let instance = linker
            .instantiate_async(&mut store, &component)
            .await
            .map_err(|e| ModuleHostError::Instantiate(e.to_string()))?;

        let lifecycle = Lifecycle::new(&mut store, &instance)
            .map_err(|e| ModuleHostError::Lifecycle { hook: "new".into(), message: e.to_string() })?;
        lifecycle
            .call_activate(&mut store)
            .await
            .map_err(|e| ModuleHostError::Lifecycle { hook: "activate".into(), message: e.to_string() })?;

        if let Err(e) = lifecycle.call_health_check(&mut store).await {
            warn!(module = %module_id, error = %e, "initial health check failed");
        }

        info!(module = %module_id, version = %module_version, "module loaded");

        let (tx, rx) = mpsc::channel::<InstanceMessage>(64);
        let task_id = module_id.clone();
        let join = tokio::spawn(instance_task(task_id, store, instance, rx));

        Ok(InstanceHandle {
            id: module_id,
            version: module_version,
            grants,
            tx,
            join: Some(join),
        })
    }
}

async fn instance_task(
    module_id: String,
    mut store: Store<HostState>,
    instance: wasmtime::component::Instance,
    mut rx: mpsc::Receiver<InstanceMessage>,
) {
    while let Some(msg) = rx.recv().await {
        match msg {
            InstanceMessage::Invoke { method, args, reply } => {
                let result = dispatch_invoke(&module_id, &mut store, &instance, &method, args).await;
                let _ = reply.send(result);
            }
            InstanceMessage::Event { kind, data } => {
                let _ = dispatch_invoke(&module_id, &mut store, &instance, &kind, data).await;
            }
            InstanceMessage::Shutdown(ack) => {
                if let Ok(lifecycle) = Lifecycle::new(&mut store, &instance) {
                    let _ = lifecycle.call_deactivate(&mut store).await;
                }
                let _ = ack.send(());
                return;
            }
        }
    }
}

async fn dispatch_invoke(
    module_id: &str,
    store: &mut Store<HostState>,
    instance: &wasmtime::component::Instance,
    method: &str,
    args: Vec<u8>,
) -> DneaResult<Vec<u8>> {
    // Prefer the generic module-api export.
    if let Ok(api) = ModuleApi::new(store, instance) {
        let args_json = String::from_utf8_lossy(&args).into_owned();
        return match api.call_invoke(store, method, &args_json).await {
            Ok(Ok(json)) => Ok(json.into_bytes()),
            Ok(Err(e)) => Err(DneaError::Module {
                module: module_id.to_string(),
                code: "host_error".into(),
                message: format!("{e:?}"),
            }),
            Err(e) => Err(DneaError::Module {
                module: module_id.to_string(),
                code: "invoke".into(),
                message: e.to_string(),
            }),
        };
    }

    // Fallback: lifecycle-only modules.
    if method == "healthCheck" {
        let lifecycle = Lifecycle::new(store, instance)
            .map_err(|e| DneaError::Module {
                module: module_id.to_string(),
                code: "lifecycle_new".into(),
                message: e.to_string(),
            })?;
        lifecycle
            .call_health_check(store)
            .await
            .map_err(|e| DneaError::Module {
                module: module_id.to_string(),
                code: "health_check".into(),
                message: e.to_string(),
            })?;
        return Ok(Vec::new());
    }

    Err(DneaError::Module {
        module: module_id.to_string(),
        code: "unknown_method".into(),
        message: format!("no such export: {method}"),
    })
}