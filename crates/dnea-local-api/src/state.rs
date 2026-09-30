//! API state and configuration.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use dnea_session::{install::InstallPipeline, Session};
use rand::RngCore;
use serde::Serialize;
use tokio::net::TcpListener;
use tracing::info;

#[derive(Clone, Debug)]
pub struct ApiConfig {
    pub bind: SocketAddr,
    pub discovery_path: PathBuf,
    pub token_ttl: Duration,
}

impl ApiConfig {
    #[must_use]
    pub fn default_config() -> Self {
        let proj = directories::ProjectDirs::from("org", "dnea", "runtime")
            .expect("no home directory");
        let discovery_path = proj.runtime_dir()
            .unwrap_or_else(std::env::temp_dir).join("api.json");
        Self {
            bind: "127.0.0.1:0".parse().expect("valid loopback"),
            discovery_path,
            token_ttl: Duration::from_secs(600),
        }
    }
}

pub struct ApiState {
    pub session: Session,
    pub secret: Vec<u8>,
    pub token_ttl: Duration,
    pub install_pipeline: Option<Arc<InstallPipeline>>,
}

impl ApiState {
    #[must_use]
    pub fn new(session: Session, config: &ApiConfig) -> Self {
        let mut secret = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut secret);
        Self {
            session,
            secret,
            token_ttl: config.token_ttl,
            install_pipeline: None,
        }
    }

    #[must_use]
    pub fn with_pipeline(mut self, pipeline: Arc<InstallPipeline>) -> Self {
        self.install_pipeline = Some(pipeline);
        self
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LocalApiError {
    #[error("bind {addr}: {source}")]
    Bind { addr: SocketAddr, source: std::io::Error },
    #[error("write discovery file: {0}")]
    Discovery(std::io::Error),
}

#[derive(Serialize)]
struct Discovery<'a> {
    v: u32,
    url: String,
    ws: String,
    pid: u32,
    started_at: String,
    session_id: &'a str,
    scopes: Vec<&'static str>,
}

pub struct LocalApi {
    state: Arc<ApiState>,
    listener: TcpListener,
    config: ApiConfig,
}

impl LocalApi {
    pub async fn bind(session: Session, config: ApiConfig) -> Result<Self, LocalApiError> {
        let listener = TcpListener::bind(config.bind).await
            .map_err(|source| LocalApiError::Bind { addr: config.bind, source })?;
        let state = Arc::new(ApiState::new(session, &config));
        Ok(Self { state, listener, config })
    }

    pub async fn bind_with_pipeline(
        session: Session,
        config: ApiConfig,
        pipeline: Arc<InstallPipeline>,
    ) -> Result<Self, LocalApiError> {
        let listener = TcpListener::bind(config.bind).await
            .map_err(|source| LocalApiError::Bind { addr: config.bind, source })?;
        let state = Arc::new(ApiState::new(session, &config).with_pipeline(pipeline));
        Ok(Self { state, listener, config })
    }

    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    pub async fn serve(self) -> anyhow::Result<()> {
        let addr = self.listener.local_addr()?;
        let session_id = self.state.session.session_id().to_string();
        let discovery = Discovery {
            v: 1,
            url: format!("http://{addr}"),
            ws: format!("ws://{addr}/ws"),
            pid: std::process::id(),
            started_at: time::OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_else(|_| "unknown".into()),
            session_id: &session_id,
            scopes: crate::auth::Scope::companion_set().iter()
                .map(|s| s.as_str()).collect(),
        };
        let bytes = serde_json::to_vec_pretty(&discovery)?;
        if let Some(parent) = self.config.discovery_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::fs::write(&self.config.discovery_path, &bytes)
            .map_err(LocalApiError::Discovery)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = std::fs::metadata(&self.config.discovery_path) {
                let mut perms = meta.permissions();
                perms.set_mode(0o600);
                let _ = std::fs::set_permissions(&self.config.discovery_path, perms);
            }
        }

        info!(%addr, "local api listening");
        let router = routes::router(self.state.clone());
        axum::serve(self.listener, router).await?;
        Ok(())
    }
}