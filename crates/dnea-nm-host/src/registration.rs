//! Per-browser Native Messaging host manifest writers.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Browser { Chrome, Chromium, Edge, Brave, Vivaldi, Opera, Firefox }

impl Browser {
    #[must_use]
    pub fn all() -> &'static [Browser] {
        &[Browser::Chrome, Browser::Chromium, Browser::Edge,
          Browser::Brave, Browser::Vivaldi, Browser::Opera, Browser::Firefox]
    }

    #[must_use]
    pub fn host_name(self) -> &'static str { "org.dnea.runtime" }

    #[must_use]
    pub fn allowed_origin(self, extension_id: &str) -> String {
        match self {
            Browser::Firefox => format!("moz-extension://{extension_id}/"),
            _ => format!("chrome-extension://{extension_id}/"),
        }
    }

    #[must_use]
    pub fn manifest_dirs(self) -> Vec<PathBuf> {
        #[cfg(target_os = "linux")]
        {
            match self {
                Browser::Chrome => vec![PathBuf::from("/etc/opt/chrome/native-messaging-hosts")],
                Browser::Chromium => vec![PathBuf::from("/etc/chromium/native-messaging-hosts")],
                Browser::Edge => vec![PathBuf::from("/etc/opt/edge/native-messaging-hosts")],
                Browser::Firefox => vec![PathBuf::from("/usr/lib/mozilla/native-messaging-hosts")],
                Browser::Brave | Browser::Vivaldi | Browser::Opera =>
                    vec![PathBuf::from("/etc/opt/chrome/native-messaging-hosts")],
            }
        }
        #[cfg(target_os = "macos")]
        {
            match self {
                Browser::Chrome | Browser::Chromium | Browser::Brave
                | Browser::Vivaldi | Browser::Opera =>
                    vec![PathBuf::from("/Library/Google/Chrome/NativeMessagingHosts")],
                Browser::Edge => vec![PathBuf::from("/Library/Microsoft/Edge/NativeMessagingHosts")],
                Browser::Firefox => vec![PathBuf::from(
                    "/Library/Application Support/Mozilla/NativeMessagingHosts")],
            }
        }
        #[cfg(windows)]
        {
            let _ = self;
            vec![PathBuf::from(r"C:\ProgramData\DNEA\nm")]
        }
    }
}

#[derive(Serialize)]
struct HostManifest {
    name: String,
    description: String,
    path: PathBuf,
    #[serde(rename = "type")]
    kind: &'static str,
    allowed_origins: Vec<String>,
    #[serde(rename = "x-dnea-installed")]
    marker: XDneaInstalled,
}

#[derive(Serialize)]
struct XDneaInstalled {
    version: String,
    installed_at: String,
    content_digest: String,
}

pub fn register(
    browser: Browser,
    extension_ids: &[&str],
    bridge_path: &Path,
    runtime_version: &str,
) -> Result<Vec<PathBuf>> {
    if extension_ids.is_empty() {
        bail!("at least one extension ID is required");
    }
    let allowed: Vec<String> = extension_ids.iter()
        .map(|id| browser.allowed_origin(id))
        .collect();
    let mut manifest = HostManifest {
        name: browser.host_name().into(),
        description: "DNEA runtime bridge".into(),
        path: bridge_path.to_path_buf(),
        kind: "stdio",
        allowed_origins: allowed,
        marker: XDneaInstalled {
            version: runtime_version.into(),
            installed_at: time::OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_else(|_| "unknown".into()),
            content_digest: String::new(),
        },
    };

    let pre_digest = serde_json::to_vec(&serde_json::json!({
        "name": manifest.name,
        "description": manifest.description,
        "path": manifest.path,
        "type": manifest.kind,
        "allowed_origins": manifest.allowed_origins,
        "x-dnea-installed": {
            "version": manifest.marker.version,
            "installed_at": manifest.marker.installed_at,
        }
    }))?;
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(&pre_digest);
    manifest.marker.content_digest = format!(
        "sha-256:{}",
        base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD, h.finalize()),
    );

    let bytes = serde_json::to_vec_pretty(&manifest)?;
    let mut written = Vec::new();
    for dir in browser.manifest_dirs() {
        fs::create_dir_all(&dir).with_context(|| format!("create {}", dir.display()))?;
        let path = dir.join(format!("{}.json", browser.host_name()));
        if path.exists() {
            let existing = fs::read_to_string(&path).unwrap_or_default();
            if !existing.contains("x-dnea-installed") {
                let backup = path.with_extension(format!(
                    "json.bak-{}",
                    time::OffsetDateTime::now_utc()
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap_or_else(|_| "unknown".into())
                        .replace(':', "-"),
                ));
                fs::rename(&path, &backup)?;
            }
        }
        fs::write(&path, &bytes)?;
        written.push(path);
    }

    #[cfg(windows)]
    {
        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key_path = browser.registry_key();
        let (key, _) = hklm
            .create_subkey(key_path)
            .with_context(|| format!("create registry key {key_path}"))?;
        let manifest_path = written
            .first()
            .ok_or_else(|| anyhow::anyhow!("no manifest written"))?;
        key.set_value(browser.host_name(), &manifest_path.to_string_lossy().to_string())?;
    }

    Ok(written)
}

#[cfg(windows)]
impl Browser {
    fn registry_key(self) -> &'static str {
        match self {
            Browser::Chrome | Browser::Chromium | Browser::Brave
            | Browser::Vivaldi | Browser::Opera =>
                r"SOFTWARE\Google\Chrome\NativeMessagingHosts",
            Browser::Edge => r"SOFTWARE\Microsoft\Edge\NativeMessagingHosts",
            Browser::Firefox => r"SOFTWARE\Mozilla\NativeMessagingHosts",
        }
    }
}

use base64::Engine as _;