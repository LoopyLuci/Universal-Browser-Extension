//! The DNEA runtime daemon.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use base64::Engine as _;
use clap::Parser;
use dnea_config::Config;
use dnea_local_api::{ApiConfig, LocalApi};
use dnea_manifest::trust::{ResolvedKey, SigAlg, StaticTrustStore};
use dnea_manifest::validate::Validator;
use dnea_module_host::engine::EngineConfig;
use dnea_module_host::grant::GrantSet;
use dnea_module_host::host::HostState;
use dnea_module_host::instance::ModuleHost;
use dnea_nm_host::supervisor::{NmHostConfig, NmSupervisor};
use dnea_session::install::{HostStateFactory, InstallPipeline};
use dnea_session::Session;
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser, Debug)]
#[command(name = "dnea-daemon", version, about = "DNEA runtime daemon")]
struct Args {
    #[arg(long, env = "DNEA_CONFIG", default_value_os_t = Config::default_path())]
    config: PathBuf,
    #[arg(long, env = "DNEA_LOG", default_value = "info")]
    log: String,
    #[arg(long)]
    log_json: bool,
    #[arg(long)]
    init_key: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let filter = EnvFilter::try_new(&args.log).context("invalid log filter")?;
    let registry = tracing_subscriber::registry().with(filter);
    if args.log_json {
        registry.with(fmt::layer().json()).init();
    } else {
        registry.with(fmt::layer().compact()).init();
    }

    let config = if args.config.exists() {
        Config::load_from(&args.config)
            .with_context(|| format!("load config from {}", args.config.display()))?
    } else {
        let mut c = Config::default();
        c.validate().context("validate default config")?;
        c
    };

    if args.init_key { return init_master_key(&config); }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("dnea")
        .build()?;
    rt.block_on(run(config))
}

fn init_master_key(config: &Config) -> Result<()> {
    let path = config.paths.state_dir.join("master.key");
    std::fs::create_dir_all(&config.paths.state_dir)?;
    if path.exists() {
        anyhow::bail!("master key already exists at {}", path.display());
    }
    let mut key = [0u8; 32];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut key);
    std::fs::write(&path, key)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&path)?.permissions();
        perms.set_mode(0o600);
        std::fs::set_permissions(&path, perms)?;
    }
    info!(path = %path.display(), "master key initialized");
    Ok(())
}

struct DaemonHostStateFactory;

impl HostStateFactory for DaemonHostStateFactory {
    fn build(&self, module_id: &str, version: &str, grants: Arc<GrantSet>) -> HostState {
        HostState::new(
            module_id.to_string(),
            version.to_string(),
            grants,
            wasmtime::StoreLimitsBuilder::new().memory_size(64 * 1024 * 1024).build(),
        )
    }
}

async fn run(config: Config) -> Result<()> {
    info!(version = env!("CARGO_PKG_VERSION"), "dnea-daemon starting");

    // Single-instance guard.
    let lock_path = config.paths.pid_file.clone();
    let _lock = SingleInstance::acquire(&lock_path)
        .context("another runtime is already running")?;

    for dir in [
        &config.paths.state_dir, &config.paths.run_dir,
        &config.paths.log_dir, &config.paths.modules_dir,
    ] {
        std::fs::create_dir_all(dir)?;
    }

    // 1. Module host.
    let engine_cfg = EngineConfig {
        total_memory_slots: config.modules.pooling_slots,
        total_stack_slots: config.modules.pooling_slots,
        ..EngineConfig::default()
    };
    let module_host = Arc::new(ModuleHost::new(&engine_cfg).context("build module host")?);

    // 2. Session.
    let session = Session::new();

    // 3. Trust store.
    let trust: Arc<dyn dnea_manifest::trust::TrustStore> =
        Arc::new(build_trust_store(config.modules.trust_store.as_deref()));

    // 4. Validator.
    let validator = Arc::new(Validator::new().context("build manifest validator")?);

    // 5. Install pipeline.
    let pipeline = Arc::new(
        InstallPipeline::new(
            session.clone(),
            module_host.clone(),
            validator,
            trust,
            Arc::new(DaemonHostStateFactory),
        )
        .with_install_roots(config.api.install_roots.clone()),
    );

    // 6. NM supervisor.
    let nm_config = NmHostConfig::new(config.nm_endpoint());
    let nm_supervisor = NmSupervisor::new(nm_config, Arc::new(session.clone()));

    // 7. Local API.
    let api_config = ApiConfig {
        bind: "127.0.0.1:0".parse().context("invalid API bind address")?,
        discovery_path: config.api.discovery_path.clone()
            .unwrap_or_else(|| config.paths.run_dir.join("api.json")),
        token_ttl: config.token_ttl(),
    };
    let api = LocalApi::bind_with_pipeline(session.clone(), api_config, pipeline)
        .await.context("bind local api")?;

    // 8. Watchdog.
    let engine_clone = module_host.engine_clone();
    let epoch_interval = config.epoch_interval();
    let watchdog = tokio::spawn(async move {
        let mut ticker = tokio::time::interval(epoch_interval);
        loop {
            ticker.tick().await;
            engine_clone.increment_epoch();
        }
    });

    // 9. Consent sweeper.
    let session_clone = session.clone();
    let sweeper = tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(30));
        loop {
            ticker.tick().await;
            session_clone.sweep_consents(Duration::from_secs(300)).await;
        }
    });

    // 10. Run.
    let nm_task = tokio::spawn(async move {
        if let Err(e) = nm_supervisor.run().await {
            error!(error = %e, "nm supervisor failed");
        }
    });
    let api_task = tokio::spawn(async move {
        if let Err(e) = api.serve().await {
            error!(error = %e, "local api failed");
        }
    });

    // 11. Shutdown.
    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())?;
    tokio::select! {
        _ = signal::ctrl_c() => info!("ctrl-c received"),
        _ = sigterm.recv() => info!("SIGTERM received"),
    }
    info!("shutting down");
    watchdog.abort();
    sweeper.abort();
    nm_task.abort();
    api_task.abort();
    let _ = tokio::time::timeout(Duration::from_secs(5), session.shutdown()).await;
    Ok(())
}

fn build_trust_store(path: Option<&std::path::Path>) -> StaticTrustStore {
    let mut store = StaticTrustStore::new();
    if let Some(p) = path {
        if let Ok(bytes) = std::fs::read(p) {
            if let Ok(v) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                if let Some(keys) = v.get("keys").and_then(|k| k.as_array()) {
                    for k in keys {
                        let Some(id) = k.get("keyId").and_then(|v| v.as_str()) else { continue };
                        let Some(alg) = k.get("alg").and_then(|v| v.as_str()).and_then(SigAlg::parse)
                            else { continue };
                        let Some(pk_b64) = k.get("publicKey").and_then(|v| v.as_str()) else { continue };
                        let Ok(pk) = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(pk_b64)
                            else { continue };
                        let revoked = k.get("revoked").and_then(|v| v.as_bool()).unwrap_or(false);
                        store.insert(id, ResolvedKey { alg, public_key: pk, revoked });
                    }
                }
            }
        }
    }
    store
}

#[cfg(unix)]
struct SingleInstance { _file: std::fs::File }

#[cfg(unix)]
impl SingleInstance {
    fn acquire(path: &std::path::Path) -> Result<Self> {
        use std::os::unix::io::AsRawFd;
        if let Some(parent) = path.parent() { std::fs::create_dir_all(parent).ok(); }
        let file = std::fs::OpenOptions::new()
            .create(true).write(true).truncate(false).open(path)?;
        let rc = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
        if rc != 0 {
            anyhow::bail!("another runtime is already running (lock held on {})", path.display());
        }
        std::fs::write(path, std::process::id().to_string()).ok();
        Ok(Self { _file: file })
    }
}

#[cfg(windows)]
struct SingleInstance { _handle: isize }

#[cfg(windows)]
impl SingleInstance {
    fn acquire(_path: &std::path::Path) -> Result<Self> {
        use windows_sys::Win32::System::Threading::CreateMutexW;
        use windows_sys::Win32::Foundation::GetLastError;
        use windows_sys::Win32::Foundation::ERROR_ALREADY_EXISTS;
        let name: Vec<u16> = "Global\\DNEA-Runtime\0".encode_utf16().collect();
        let handle = unsafe { CreateMutexW(std::ptr::null(), 0, name.as_ptr()) };
        if handle == 0 { anyhow::bail!("CreateMutexW failed"); }
        let err = unsafe { GetLastError() };
        if err == ERROR_ALREADY_EXISTS {
            anyhow::bail!("another runtime is already running");
        }
        Ok(Self { _handle: handle as isize })
    }
}