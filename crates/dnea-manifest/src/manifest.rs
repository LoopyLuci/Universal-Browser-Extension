//! The typed manifest.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::canonical::CanonicalError;

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("missing required field: {0}")]
    MissingField(String),
    #[error("invalid module ID: {0}")]
    InvalidId(String),
    #[error("manifest is not signed")]
    MissingSignature,
    #[error("payload digest mismatch: expected {expected}, actual {actual}")]
    DigestMismatch { expected: String, actual: String },
    #[error("unknown signing key: {0}")]
    UnknownKey(String),
    #[error("signing key revoked: {0}")]
    RevokedKey(String),
    #[error("signature verification failed for key {key_id}: {reason}")]
    BadSignature { key_id: String, reason: String },
    #[error("manifest has no author or publisher signature")]
    MissingTrustedSigner,
    #[error("unsupported signature algorithm: {0}")]
    UnsupportedAlgorithm(String),
    #[error("canonicalization: {0}")]
    Canonical(#[from] CanonicalError),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "manifestVersion")]
    pub manifest_version: u32,
    pub id: String,
    pub version: String,
    pub runtime: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub authors: Vec<Author>,
    #[serde(default)]
    pub dev: bool,
    #[serde(default)]
    pub signature: Option<Signature>,
    pub entrypoints: BTreeMap<String, Entrypoint>,
    pub capabilities: Capabilities,
    #[serde(default)]
    pub dependencies: BTreeMap<String, Dependency>,
    #[serde(default)]
    pub conflicts: Vec<Conflict>,
    #[serde(default)]
    pub lifecycle: Lifecycle,
    #[serde(default)]
    pub migrations: Vec<Migration>,
    #[serde(default)]
    pub storage: Storage,
    #[serde(default)]
    pub ui: Vec<UiSurface>,
    #[serde(default)]
    pub settings: Option<Value>,
    #[serde(default)]
    pub sandbox: Option<Value>,
    #[serde(default)]
    pub resources: Option<Value>,
    #[serde(default)]
    pub audit: Option<Value>,
    #[serde(default)]
    pub update: Option<Value>,
    #[serde(default)]
    pub i18n: Option<Value>,
    #[serde(default, rename = "dynamicOriginGrants")]
    pub dynamic_origin_grants: Option<DynamicOriginGrants>,
    #[serde(default, rename = "signaturePolicy")]
    pub signature_policy: Option<Value>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
    #[serde(rename = "payloadDigest")]
    pub payload_digest: PayloadDigest,
    pub signers: Vec<Signer>,
    #[serde(default)]
    pub transparency: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PayloadDigest {
    pub alg: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signer {
    #[serde(rename = "keyId")]
    pub key_id: String,
    pub alg: String,
    pub value: String,
    #[serde(default = "default_role")]
    pub role: String,
    #[serde(default, rename = "signedAt")]
    pub signed_at: Option<String>,
    #[serde(default, rename = "expiresAt")]
    pub expires_at: Option<String>,
}

fn default_role() -> String { "author".into() }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entrypoint {
    pub kind: String,
    pub path: String,
    pub digest: PayloadDigest,
    #[serde(default = "default_abi")]
    pub abi: String,
    #[serde(default)]
    pub exports: Vec<String>,
    #[serde(default, rename = "sandboxProfile")]
    pub sandbox_profile: Option<String>,
}

fn default_abi() -> String { "1.0.0".into() }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Capabilities {
    pub request: Vec<CapabilityRequest>,
    #[serde(default)]
    pub deny: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityRequest {
    pub id: String,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub consent: Option<String>,
    #[serde(default)]
    pub constraints: Value,
    #[serde(default = "default_true")]
    pub required: bool,
    #[serde(default)]
    pub audit: Option<String>,
    #[serde(default, rename = "policyKey")]
    pub policy_key: Option<String>,
}

fn default_true() -> bool { true }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub version: String,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Conflict {
    pub id: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Lifecycle {
    #[serde(default)] pub install: Option<HookRef>,
    #[serde(default)] pub activate: Option<HookRef>,
    #[serde(default)] pub deactivate: Option<HookRef>,
    #[serde(default)] pub upgrade: Option<HookRef>,
    #[serde(default)] pub downgrade: Option<HookRef>,
    #[serde(default)] pub uninstall: Option<HookRef>,
    #[serde(default, rename = "healthCheck")] pub health_check: Option<HookRef>,
    #[serde(default, rename = "onCapabilityChange")] pub on_capability_change: Option<HookRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HookRef {
    pub entrypoint: String,
    pub export: String,
    #[serde(default = "default_timeout", rename = "timeoutMs")]
    pub timeout_ms: u64,
    #[serde(default)]
    pub fatal: bool,
}

fn default_timeout() -> u64 { 10_000 }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Migration {
    pub from: u32,
    pub to: u32,
    pub entrypoint: String,
    pub export: String,
    #[serde(default)]
    pub reversible: bool,
    #[serde(default = "default_migration_timeout", rename = "timeoutMs")]
    pub timeout_ms: u64,
}

fn default_migration_timeout() -> u64 { 60_000 }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Storage {
    #[serde(default = "default_storage_schema", rename = "storageSchemaVersion")]
    pub storage_schema_version: u32,
    #[serde(default = "default_true")]
    pub encrypted: bool,
    #[serde(default = "default_quota", rename = "quotaBytes")]
    pub quota_bytes: u64,
    #[serde(default, rename = "sharedNamespaces")]
    pub shared_namespaces: Vec<String>,
    #[serde(default)]
    pub exportable: bool,
}

fn default_storage_schema() -> u32 { 1 }
fn default_quota() -> u64 { 10 * 1024 * 1024 }

impl Default for Storage {
    fn default() -> Self {
        Self {
            storage_schema_version: 1,
            encrypted: true,
            quota_bytes: default_quota(),
            shared_namespaces: Vec::new(),
            exportable: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiSurface {
    pub id: String,
    pub kind: String,
    pub entrypoint: String,
    #[serde(default)]
    pub surfaces: Vec<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub origins: Vec<String>,
    #[serde(default)]
    pub keybinding: Option<Value>,
    #[serde(default)]
    pub order: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DynamicOriginGrants {
    pub enabled: bool,
    #[serde(default = "default_max_origins", rename = "maxOrigins")]
    pub max_origins: u32,
    #[serde(default = "default_dynamic_consent")]
    pub consent: String,
    #[serde(default)]
    pub reason: Option<String>,
}

fn default_max_origins() -> u32 { 256 }
fn default_dynamic_consent() -> String { "per-use".into() }

impl Manifest {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ManifestError> {
        Ok(serde_json::from_slice(bytes)?)
    }

    #[must_use]
    pub fn entrypoint(&self, name: &str) -> Option<&Entrypoint> {
        self.entrypoints.get(name)
    }
}