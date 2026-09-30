//! The `dnea` CLI.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result};
use base64::Engine as _;
use clap::{Parser, Subcommand};
use dnea_manifest::signature::{sign_manifest, verify_manifest};
use dnea_manifest::trust::{ResolvedKey, SigAlg, StaticTrustStore};
use dnea_manifest::validate::Validator;
use dnea_nm_host::registration::{register, Browser};
use sha2::Digest;

#[derive(Parser)]
#[command(name = "dnea", version, about = "DNEA developer CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Validate {
        manifest: PathBuf,
        #[arg(long)] artifacts: Option<PathBuf>,
        #[arg(long)] no_sig: bool,
        #[arg(long)] trust: Option<PathBuf>,
    },
    Sign {
        manifest: PathBuf,
        #[arg(long)] key: PathBuf,
        #[arg(long)] key_id: String,
        #[arg(long, default_value = "author")] role: String,
        #[arg(long)] out: Option<PathBuf>,
    },
    Verify {
        manifest: PathBuf,
        #[arg(long)] trust: PathBuf,
    },
    Keygen {
        #[arg(long, default_value = "dnea-signing.key")] out: PathBuf,
    },
    RegisterHost {
        #[arg(long, value_enum)] browser: BrowserArg,
        #[arg(long)] extension_id: String,
        #[arg(long)] bridge: PathBuf,
    },
    Status {
        #[arg(long)] discovery: Option<PathBuf>,
    },
    InstallModule {
        manifest: PathBuf,
        #[arg(long)] artifacts: PathBuf,
        #[arg(long)] discovery: Option<PathBuf>,
    },
    Doctor {
        #[arg(long)] discovery: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum BrowserArg { Chrome, Chromium, Edge, Brave, Vivaldi, Opera, Firefox }

impl From<BrowserArg> for Browser {
    fn from(a: BrowserArg) -> Self {
        match a {
            BrowserArg::Chrome => Browser::Chrome,
            BrowserArg::Chromium => Browser::Chromium,
            BrowserArg::Edge => Browser::Edge,
            BrowserArg::Brave => Browser::Brave,
            BrowserArg::Vivaldi => Browser::Vivaldi,
            BrowserArg::Opera => Browser::Opera,
            BrowserArg::Firefox => Browser::Firefox,
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(code) => code,
        Err(e) => { eprintln!("dnea: {e:#}"); ExitCode::from(1) }
    }
}

fn run(cli: Cli) -> Result<ExitCode> {
    match cli.cmd {
        Cmd::Validate { manifest, artifacts, no_sig, trust } =>
            validate(&manifest, artifacts.as_deref(), no_sig, trust.as_deref()),
        Cmd::Sign { manifest, key, key_id, role, out } =>
            sign(&manifest, &key, &key_id, &role, out.as_deref()),
        Cmd::Verify { manifest, trust } => verify(&manifest, &trust),
        Cmd::Keygen { out } => keygen(&out),
        Cmd::RegisterHost { browser, extension_id, bridge } =>
            register_host(browser.into(), &extension_id, &bridge),
        Cmd::Status { discovery } =>
            tokio::runtime::Runtime::new()?.block_on(status(discovery.as_deref())),
        Cmd::InstallModule { manifest, artifacts, discovery } =>
            tokio::runtime::Runtime::new()?.block_on(install_module(&manifest, &artifacts, discovery.as_deref())),
        Cmd::Doctor { discovery } =>
            tokio::runtime::Runtime::new()?.block_on(doctor(discovery.as_deref())),
    }
}

fn validate(
    manifest: &PathBuf,
    artifacts: Option<&Path>,
    no_sig: bool,
    trust: Option<&Path>,
) -> Result<ExitCode> {
    let bytes = std::fs::read(manifest)?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)?;

    let validator = Validator::new()?;
    validator.validate(&value).context("semantic validation failed")?;
    println!("✓ schema and semantics");

    if !no_sig {
        let trust_path = trust.context("--trust required unless --no-sig")?;
        let store = load_trust(trust_path)?;
        let info = verify_manifest(&value, &store).context("signature verification failed")?;
        println!("✓ signatures: {} signer(s)", info.signers.len());
        for s in &info.signers {
            println!("  - {} ({})", s.key_id, s.role);
        }
    }

    if let Some(dir) = artifacts {
        let manifest: dnea_manifest::Manifest = serde_json::from_value(value)?;
        for (name, ep) in &manifest.entrypoints {
            let path = dir.join(&ep.path);
            let bytes = std::fs::read(&path)
                .with_context(|| format!("read {}", path.display()))?;
            let mut hasher = sha2::Sha256::new();
            Digest::update(&mut hasher, &bytes);
            let hash = hasher.finalize();
            let actual = format!("sha-256:{}",
                base64::engine::general_purpose::URL_SAFE_NO_PAD
                    .encode(hash));
            if actual != ep.digest.value {
                anyhow::bail!("artifact {name} digest mismatch");
            }
            println!("✓ artifact {name}");
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn sign(
    manifest: &PathBuf, key_path: &PathBuf, key_id: &str, role: &str, out: Option<&Path>,
) -> Result<ExitCode> {
    let key_bytes = std::fs::read(key_path)?;
    let signing_key = ed25519_dalek::SigningKey::from_bytes(
        key_bytes.as_slice().try_into().context("signing key must be 32 bytes")?,
    );
    let bytes = std::fs::read(manifest)?;
    let mut value: serde_json::Value = serde_json::from_slice(&bytes)?;
    sign_manifest(&mut value, &signing_key, key_id, role)?;
    let target = out.unwrap_or(manifest.as_path());
    std::fs::write(target, serde_json::to_vec_pretty(&value)?)?;
    println!("signed manifest written to {}", target.display());
    Ok(ExitCode::SUCCESS)
}

fn verify(manifest: &PathBuf, trust: &PathBuf) -> Result<ExitCode> {
    let bytes = std::fs::read(manifest)?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)?;
    let store = load_trust(trust)?;
    let info = verify_manifest(&value, &store)?;
    println!("verified: {} signer(s)", info.signers.len());
    for s in &info.signers { println!("  - {} ({})", s.key_id, s.role); }
    Ok(ExitCode::SUCCESS)
}

fn keygen(out: &PathBuf) -> Result<ExitCode> {
    use rand::rngs::OsRng;
    let sk = ed25519_dalek::SigningKey::generate(&mut OsRng);
    std::fs::write(out, sk.to_bytes())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(out)?.permissions();
        perms.set_mode(0o600);
        std::fs::set_permissions(out, perms)?;
    }
    let vk = sk.verifying_key();
    let vk_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(vk.to_bytes());
    println!("private key: {}", out.display());
    println!("public key (base64url): {}", vk_b64);
    println!();
    println!("Add to your trust store:");
    println!("  {{ \"keyId\": \"...\", \"alg\": \"EdDSA\", \"publicKey\": \"{vk_b64}\" }}");
    Ok(ExitCode::SUCCESS)
}

fn register_host(browser: Browser, extension_id: &str, bridge: &PathBuf) -> Result<ExitCode> {
    let written = register(browser, &[extension_id], bridge, env!("CARGO_PKG_VERSION"))?;
    for p in written { println!("✓ {}", p.display()); }
    Ok(ExitCode::SUCCESS)
}

async fn status(discovery: Option<&Path>) -> Result<ExitCode> {
    let disc = load_discovery(discovery)?;
    let client = reqwest::Client::new();
    let token: serde_json::Value = client
        .post(format!("{}/v1/auth/token", disc["url"].as_str().unwrap()))
        .json(&serde_json::json!({ "grantType": "cli", "requestedScopes": ["ui.read"] }))
        .send().await?.json().await?;
    let token = token["token"].as_str().context("no token")?;
    let status: serde_json::Value = client
        .get(format!("{}/v1/status", disc["url"].as_str().unwrap()))
        .bearer_auth(token).send().await?.json().await?;
    println!("{}", serde_json::to_string_pretty(&status)?);
    Ok(ExitCode::SUCCESS)
}

async fn install_module(
    manifest: &PathBuf, artifacts: &PathBuf, discovery: Option<&Path>,
) -> Result<ExitCode> {
    let disc = load_discovery(discovery)?;
    let client = reqwest::Client::new();
    let token: serde_json::Value = client
        .post(format!("{}/v1/auth/token", disc["url"].as_str().unwrap()))
        .json(&serde_json::json!({
            "grantType": "cli", "requestedScopes": ["ui.modules", "ui.read"] }))
        .send().await?.json().await?;
    let token = token["token"].as_str().context("no token")?;
    let report: serde_json::Value = client
        .post(format!("{}/v1/modules", disc["url"].as_str().unwrap()))
        .bearer_auth(token)
        .json(&serde_json::json!({
            "source": "local",
            "manifestPath": manifest,
            "artifactDir": artifacts,
        }))
        .send().await?.json().await?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(ExitCode::SUCCESS)
}

async fn doctor(discovery: Option<&Path>) -> Result<ExitCode> {
    let disc = load_discovery(discovery)?;
    println!("Runtime: {}", disc["url"]);
    println!("PID:     {}", disc["pid"]);
    println!("Session: {}", disc["session_id"]);
    let cfg = dnea_config::Config::load()?;
    println!("Config:  {}", dnea_config::Config::default_path().display());
    println!("State:   {}", cfg.paths.state_dir.display());
    println!("Run:     {}", cfg.paths.run_dir.display());
    println!("Log:     {}", cfg.paths.log_dir.display());
    for browser in Browser::all() {
        let dirs = browser.manifest_dirs();
        let mut found = false;
        for dir in &dirs {
            if dir.join(format!("{}.json", browser.host_name())).exists() { found = true; break; }
        }
        println!("Host manifest for {:?}: {}", browser, if found { "present" } else { "missing" });
    }
    Ok(ExitCode::SUCCESS)
}

fn load_trust(path: &Path) -> Result<StaticTrustStore> {
    let bytes = std::fs::read(path)?;
    let v: serde_json::Value = serde_json::from_slice(&bytes)?;
    let mut store = StaticTrustStore::new();
    let keys = v.get("keys").and_then(|k| k.as_array())
        .context("trust store must have a `keys` array")?;
    for k in keys {
        let id = k.get("keyId").and_then(|v| v.as_str()).context("keyId")?;
        let alg = k.get("alg").and_then(|v| v.as_str()).and_then(SigAlg::parse).context("alg")?;
        let pk_b64 = k.get("publicKey").and_then(|v| v.as_str()).context("publicKey")?;
        let pk = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(pk_b64)?;
        let revoked = k.get("revoked").and_then(|v| v.as_bool()).unwrap_or(false);
        store.insert(id, ResolvedKey { alg, public_key: pk, revoked });
    }
    Ok(store)
}

fn load_discovery(path: Option<&Path>) -> Result<serde_json::Value> {
    let p = match path {
        Some(p) => p.to_path_buf(),
        None => {
            let cfg = dnea_config::Config::load()?;
            cfg.api.discovery_path.clone()
                .unwrap_or_else(|| cfg.paths.run_dir.join("api.json"))
        }
    };
    let bytes = std::fs::read(&p).with_context(|| format!("read {}", p.display()))?;
    Ok(serde_json::from_slice(&bytes)?)
}