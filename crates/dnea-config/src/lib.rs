//! DNEA configuration.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("read config: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse config: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("no home directory")]
    NoHome,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub paths: Paths,
    pub modules: ModulesConfig,
    pub nm: NmConfig,
    pub api: ApiConfig,
    pub store: StoreConfig,
    pub audit: AuditConfig,
    pub policy: PolicyConfig,
    pub watchdog: WatchdogConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paths: Paths::default(), modules: ModulesConfig::default(),
            nm: NmConfig::default(), api: ApiConfig::default(),
            store: StoreConfig::default(), audit: AuditConfig::default(),
            policy: PolicyConfig::default(), watchdog: WatchdogConfig::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Paths {
    pub state_dir: PathBuf,
    pub run_dir: PathBuf,
    pub log_dir: PathBuf,
    pub modules_dir: PathBuf,
    pub pid_file: PathBuf,
}

impl Default for Paths {
    fn default() -> Self {
        let proj = directories::ProjectDirs::from("org", "dnea", "runtime")
            .expect("no home directory");
        let state_dir = proj.state_dir().map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("/var/lib/dnea"));
        let run_dir = proj.runtime_dir().map(Path::to_path_buf)
            .unwrap_or_else(std::env::temp_dir);
        let log_dir = proj.state_dir().map(|p| p.join("log"))
            .unwrap_or_else(|| PathBuf::from("/var/log/dnea"));
        Self {
            modules_dir: state_dir.join("modules"),
            pid_file: run_dir.join("dnea.pid"),
            state_dir, run_dir, log_dir,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ModulesConfig {
    pub allow_dev: bool,
    pub trust_store: Option<PathBuf>,
    pub max_loaded: usize,
    pub pooling_slots: u32,
}

impl Default for ModulesConfig {
    fn default() -> Self {
        Self { allow_dev: false, trust_store: None, max_loaded: 32, pooling_slots: 16 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct NmConfig {
    pub endpoint: Option<PathBuf>,
    pub max_connections: usize,
    pub mtu: usize,
}

impl Default for NmConfig {
    fn default() -> Self {
        Self { endpoint: None, max_connections: 64, mtu: 1_048_576 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiConfig {
    pub bind: String,
    pub discovery_path: Option<PathBuf>,
    pub token_ttl_secs: u64,
    pub install_roots: Vec<PathBuf>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1:0".into(),
            discovery_path: None,
            token_ttl_secs: 600,
            install_roots: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct StoreConfig {
    pub db_path: Option<PathBuf>,
    pub master_key: MasterKeySource,
    pub default_quota_bytes: u64,
}

impl Default for StoreConfig {
    fn default() -> Self {
        Self {
            db_path: None,
            master_key: MasterKeySource::Auto,
            default_quota_bytes: 10 * 1024 * 1024,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum MasterKeySource {
    Auto,
    OsKeystore,
    File { path: PathBuf },
    Passphrase,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AuditConfig {
    pub retention_days: u32,
    pub max_bytes_per_module: u64,
    pub sinks: Vec<AuditSinkKind>,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            retention_days: 30,
            max_bytes_per_module: 64 * 1024 * 1024,
            sinks: vec![AuditSinkKind::Sqlite, AuditSinkKind::File],
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AuditSinkKind { Sqlite, File, OsLog }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct PolicyConfig {
    pub extra_sources: Vec<PathBuf>,
    pub poll_interval_secs: u64,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self { extra_sources: Vec::new(), poll_interval_secs: 60 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct WatchdogConfig {
    pub epoch_interval_ms: u64,
    pub default_fuel: u64,
    pub default_timeout_ms: u64,
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        Self { epoch_interval_ms: 100, default_fuel: 10_000_000, default_timeout_ms: 30_000 }
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::default_path();
        let mut cfg = if path.exists() {
            let text = std::fs::read_to_string(&path)?;
            toml::from_str(&text)?
        } else {
            Self::default()
        };
        cfg.apply_env();
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn load_from(path: &Path) -> Result<Self, ConfigError> {
        let text = std::fs::read_to_string(path)?;
        let mut cfg: Self = toml::from_str(&text)?;
        cfg.apply_env();
        cfg.validate()?;
        Ok(cfg)
    }

    /// Validates configuration invariants that MUST hold at startup.
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Local API MUST bind to loopback only.
        let bind = &self.api.bind;
        let is_loopback = bind.starts_with("127.")
            || bind.starts_with("[::1]")
            || bind.starts_with("localhost");
        if !is_loopback {
            return Err(ConfigError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("api.bind must be a loopback address, got {bind}"),
            )));
        }
        Ok(())
    }

    #[must_use]
    pub fn default_path() -> PathBuf {
        if let Ok(p) = std::env::var("DNEA_CONFIG") {
            return PathBuf::from(p);
        }
        directories::ProjectDirs::from("org", "dnea", "runtime")
            .map(|p| p.config_dir().join("dnea.toml"))
            .unwrap_or_else(|| PathBuf::from("/etc/dnea/dnea.toml"))
    }

    fn apply_env(&mut self) {
        if let Ok(v) = std::env::var("DNEA_STATE_DIR") { self.paths.state_dir = PathBuf::from(v); }
        if let Ok(v) = std::env::var("DNEA_RUN_DIR") { self.paths.run_dir = PathBuf::from(v); }
        if let Ok(v) = std::env::var("DNEA_LOG_DIR") { self.paths.log_dir = PathBuf::from(v); }
        if let Ok(v) = std::env::var("DNEA_ALLOW_DEV") {
            self.modules.allow_dev = v == "1" || v.eq_ignore_ascii_case("true");
        }
        if let Ok(v) = std::env::var("DNEA_EPOCH_INTERVAL_MS") {
            if let Ok(n) = v.parse() { self.watchdog.epoch_interval_ms = n; }
        }
    }

    #[must_use]
    pub fn epoch_interval(&self) -> Duration {
        Duration::from_millis(self.watchdog.epoch_interval_ms)
    }

    #[must_use]
    pub fn token_ttl(&self) -> Duration {
        Duration::from_secs(self.api.token_ttl_secs)
    }

    #[must_use]
    pub fn nm_endpoint(&self) -> PathBuf {
        if let Some(p) = &self.nm.endpoint { return p.clone(); }
        #[cfg(unix)]
        { self.paths.run_dir.join("nm.sock") }
        #[cfg(windows)]
        {
            let user = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
            PathBuf::from(format!(r"\\.\pipe\dnea-nm-{user}"))
        }
    }

    #[must_use]
    pub fn db_path(&self) -> PathBuf {
        self.store.db_path.clone().unwrap_or_else(|| self.paths.state_dir.join("dnea.db"))
    }
}