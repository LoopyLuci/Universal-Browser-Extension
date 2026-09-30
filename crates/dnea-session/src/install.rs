//! Module installation pipeline.

use std::{collections::BTreeMap, path::PathBuf, sync::Arc, time::{Duration, SystemTime}};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use dnea_core::ids::ModuleId;
use dnea_manifest::{
    capability::{CapabilityRegistry, ConsentTier},
    manifest::Manifest,
    signature::verify_manifest,
    trust::TrustStore,
    validate::Validator,
};
use dnea_module_host::{
    grant::{EffectiveGrant, GrantSet, GrantSource},
    host::HostState,
    instance::ModuleHost,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    consent::{ConsentRequest, RememberFor},
    hub::Session,
    registry::ModuleRecord,
};

#[derive(Clone, Debug)]
pub enum InstallSource {
    Local { manifest_path: PathBuf, artifact_dir: PathBuf },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InstallStatus { Installed, PendingConsent, Failed }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscriptStep {
    pub step: String,
    pub ok: bool,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallReport {
    pub install_id: Uuid,
    pub module_id: String,
    pub version: String,
    pub status: InstallStatus,
    pub transcript: Vec<TranscriptStep>,
    pub pending_consent: Vec<String>,
}

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("manifest parse: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("validation: {0}")]
    Validation(#[from] dnea_manifest::ValidationError),
    #[error("signature: {0}")]
    Signature(#[from] dnea_manifest::ManifestError),
    #[error("artifact {name} digest mismatch: expected {expected}, got {actual}")]
    ArtifactDigest { name: String, expected: String, actual: String },
    #[error("missing dependency: {module}")]
    MissingDependency { module: String },
    #[error("conflict with {module}: {reason}")]
    Conflict { module: String, reason: String },
    #[error("consent denied for {capability}")]
    ConsentDenied { capability: String },
    #[error("consent timeout for {capability}")]
    ConsentTimeout { capability: String },
    #[error("capability is forbidden: {capability}")]
    ForbiddenCapability { capability: String },
    #[error("capability {capability} requires administrator approval")]
    AdminRequired { capability: String },
    #[error("module host: {0}")]
    ModuleHost(String),
    #[error("module ID: {0}")]
    ModuleId(String),
    #[error("unknown pending install: {0}")]
    UnknownPending(Uuid),
    #[error("install path outside permitted roots: {0}")]
    PathOutsideRoots(PathBuf),
}

pub trait HostStateFactory: Send + Sync + 'static {
    fn build(&self, module_id: &str, version: &str, grants: Arc<GrantSet>) -> HostState;
}

struct PendingInstall {
    report: InstallReport,
    manifest: Manifest,
    artifact_dir: PathBuf,
    auto_grants: GrantSet,
    remaining_prompt: Vec<String>,
    created_at: SystemTime,
}

pub struct InstallPipeline {
    session: Session,
    host: Arc<ModuleHost>,
    validator: Arc<Validator>,
    trust: Arc<dyn TrustStore>,
    host_state_factory: Arc<dyn HostStateFactory>,
    consent_timeout: Duration,
    install_roots: Vec<PathBuf>,
    pending: RwLock<BTreeMap<Uuid, PendingInstall>>,
}

impl InstallPipeline {
    pub fn new(
        session: Session,
        host: Arc<ModuleHost>,
        validator: Arc<Validator>,
        trust: Arc<dyn TrustStore>,
        host_state_factory: Arc<dyn HostStateFactory>,
    ) -> Self {
        Self {
            session, host, validator, trust, host_state_factory,
            consent_timeout: Duration::from_secs(300),
            install_roots: Vec::new(),
            pending: RwLock::new(BTreeMap::new()),
        }
    }

    #[must_use]
    pub fn with_consent_timeout(mut self, ttl: Duration) -> Self {
        self.consent_timeout = ttl; self
    }

    #[must_use]
    pub fn with_install_roots(mut self, roots: Vec<PathBuf>) -> Self {
        self.install_roots = roots; self
    }

    pub async fn install(&self, source: InstallSource) -> Result<InstallReport, InstallError> {
        let install_id = Uuid::now_v7();
        let mut transcript = Vec::with_capacity(16);

        let (manifest_bytes, artifact_dir) = match source {
            InstallSource::Local { manifest_path, artifact_dir } => {
                self.check_path(&manifest_path)?;
                self.check_path(&artifact_dir)?;
                let bytes = tokio::fs::read(&manifest_path).await?;
                (bytes, artifact_dir)
            }
        };
        transcript.push(step("load-source", true, "manifest and artifacts located"));

        let manifest_value: serde_json::Value = serde_json::from_slice(&manifest_bytes)?;
        let manifest: Manifest = serde_json::from_value(manifest_value.clone())?;
        let module_id = ModuleId::parse(&manifest.id)
            .map_err(|e| InstallError::ModuleId(e.to_string()))?;
        transcript.push(step("parse-manifest", true,
            &format!("{} v{}", manifest.id, manifest.version)));

        self.validator.validate(&manifest_value)?;
        transcript.push(step("validate", true, "schema + semantic validation passed"));

        if !manifest.dev || manifest.signature.is_some() {
            let info = verify_manifest(&manifest_value, &*self.trust)?;
            transcript.push(step_with_details("verify-signature", true,
                "signature chain verified",
                serde_json::json!({
                    "signers": info.signers.iter().map(|s| serde_json::json!({
                        "keyId": s.key_id, "role": s.role })).collect::<Vec<_>>()
                })));
        } else {
            warn!(module = %manifest.id, "installing unsigned dev module");
            transcript.push(step("verify-signature", true, "unsigned dev module accepted"));
        }

        for (name, ep) in &manifest.entrypoints {
            let path = artifact_dir.join(&ep.path);
            let bytes = tokio::fs::read(&path).await?;
            let actual = format!("sha-256:{}",
                URL_SAFE_NO_PAD.encode(Sha256::digest(&bytes)));
            if actual != ep.digest.value {
                return Err(InstallError::ArtifactDigest {
                    name: name.clone(),
                    expected: ep.digest.value.clone(),
                    actual,
                });
            }
        }
        transcript.push(step("verify-artifacts", true, "all digests match"));

        for (dep_id, dep) in &manifest.dependencies {
            let dep_module = ModuleId::parse(dep_id)
                .map_err(|e| InstallError::ModuleId(e.to_string()))?;
            if self.session.module(&dep_module).is_none() && !dep.optional {
                return Err(InstallError::MissingDependency { module: dep_id.clone() });
            }
        }
        transcript.push(step("resolve-deps", true, "dependencies satisfied"));

        for c in &manifest.conflicts {
            let cid = ModuleId::parse(&c.id)
                .map_err(|e| InstallError::ModuleId(e.to_string()))?;
            if self.session.module(&cid).is_some() {
                return Err(InstallError::Conflict {
                    module: c.id.clone(),
                    reason: c.reason.clone().unwrap_or_else(|| "declared conflict".into()),
                });
            }
        }
        transcript.push(step("check-conflicts", true, "no conflicts"));

        let (auto_grants, prompt_caps, admin_caps) = self.compute_grants(&manifest)?;
        transcript.push(step("compute-grants", true,
            &format!("{} auto, {} prompt, {} admin",
                auto_grants.len(), prompt_caps.len(), admin_caps.len())));

        if !admin_caps.is_empty() {
            return Err(InstallError::AdminRequired { capability: admin_caps[0].clone() });
        }

        if prompt_caps.is_empty() {
            return self.finish_load(
                install_id, &manifest, &module_id, &artifact_dir, auto_grants, transcript,
            ).await;
        }

        let first = &prompt_caps[0];
        let request = self.build_consent_request(&manifest, first);
        let _rx = self.session.request_consent(request).await;

        let report = InstallReport {
            install_id,
            module_id: manifest.id.clone(),
            version: manifest.version.clone(),
            status: InstallStatus::PendingConsent,
            transcript: {
                let mut t = transcript;
                t.push(step("consent", true,
                    &format!("{} capabilities require consent", prompt_caps.len())));
                t
            },
            pending_consent: prompt_caps.clone(),
        };

        self.pending.write().insert(install_id, PendingInstall {
            report: report.clone(),
            manifest, artifact_dir, auto_grants,
            remaining_prompt: prompt_caps,
            created_at: SystemTime::now(),
        });

        Ok(report)
    }

    pub async fn resume(&self, install_id: Uuid) -> Result<InstallReport, InstallError> {
        let mut pending = {
            let mut map = self.pending.write();
            map.remove(&install_id).ok_or(InstallError::UnknownPending(install_id))?
        };
        if pending.created_at.elapsed().map(|d| d > self.consent_timeout).unwrap_or(true) {
            return Err(InstallError::ConsentTimeout {
                capability: pending.remaining_prompt.first().cloned().unwrap_or_default(),
            });
        }
        let module_id = ModuleId::parse(&pending.manifest.id)
            .map_err(|e| InstallError::ModuleId(e.to_string()))?;
        let current = self.session.grants(&module_id).unwrap_or_default();
        let mut still_needed = Vec::new();
        for cap in &pending.remaining_prompt {
            if current.get(cap).is_none() { still_needed.push(cap.clone()); }
        }
        if still_needed.is_empty() {
            let mut grants = pending.auto_grants;
            for (id, g) in current.iter() {
                grants.insert(EffectiveGrant {
                    id: id.to_string(),
                    constraints: g.constraints.clone(),
                    source: g.source,
                });
            }
            return self.finish_load(
                install_id, &pending.manifest, &module_id, &pending.artifact_dir,
                grants, pending.report.transcript,
            ).await;
        }

        let next = &still_needed[0];
        let request = self.build_consent_request(&pending.manifest, next);
        let _ = self.session.request_consent(request).await;
        pending.remaining_prompt = still_needed.clone();
        pending.report.pending_consent = still_needed;
        pending.report.status = InstallStatus::PendingConsent;
        let report = pending.report.clone();
        self.pending.write().insert(install_id, pending);
        Ok(report)
    }

    async fn finish_load(
        &self,
        install_id: Uuid,
        manifest: &Manifest,
        module_id: &ModuleId,
        artifact_dir: &PathBuf,
        grants: GrantSet,
        mut transcript: Vec<TranscriptStep>,
    ) -> Result<InstallReport, InstallError> {
        let component_path = self.primary_component_path(manifest, artifact_dir)?;
        let grants_arc = Arc::new(grants.clone());
        let factory = self.host_state_factory.clone();
        let mid = manifest.id.clone();
        let ver = manifest.version.clone();

        let handle = self.host.load(
            mid.clone(), ver.clone(), component_path, grants,
            move |g| factory.build(&mid, &ver, g),
        ).await.map_err(|e| InstallError::ModuleHost(e.to_string()))?;

        transcript.push(step("load", true, "Wasm component loaded and activated"));

        let manifest_arc = Arc::new(manifest.clone());
        self.session.register_module(Arc::new(ModuleRecord {
            id: module_id.clone(),
            version: manifest.version.clone(),
            handle: Arc::new(handle),
            manifest: Some(manifest_arc),
            loaded_at: SystemTime::now(),
        }));
        self.session.set_grants(module_id, grants_arc);

        transcript.push(step("register", true, "module registered"));
        info!(module = %manifest.id, version = %manifest.version, "module installed");

        Ok(InstallReport {
            install_id,
            module_id: manifest.id.clone(),
            version: manifest.version.clone(),
            status: InstallStatus::Installed,
            transcript,
            pending_consent: Vec::new(),
        })
    }

    fn check_path(&self, path: &PathBuf) -> Result<(), InstallError> {
        if self.install_roots.is_empty() { return Ok(()); }
        let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
        for root in &self.install_roots {
            let rc = root.canonicalize().unwrap_or_else(|_| root.clone());
            if canonical.starts_with(&rc) { return Ok(()); }
        }
        Err(InstallError::PathOutsideRoots(path.clone()))
    }

    fn compute_grants(
        &self, manifest: &Manifest,
    ) -> Result<(GrantSet, Vec<String>, Vec<String>), InstallError> {
        let registry = CapabilityRegistry::v1();
        let mut auto = GrantSet::new();
        let mut prompt = Vec::new();
        let mut admin = Vec::new();
        for req in &manifest.capabilities.request {
            let tier = req.consent.as_deref().and_then(ConsentTier::parse)
                .unwrap_or_else(|| registry.default_tier(&req.id));
            match tier {
                ConsentTier::Forbidden => return Err(InstallError::ForbiddenCapability {
                    capability: req.id.clone() }),
                ConsentTier::Admin => admin.push(req.id.clone()),
                ConsentTier::Prompt | ConsentTier::PerUse => prompt.push(req.id.clone()),
                ConsentTier::Auto => {
                    auto.insert(EffectiveGrant {
                        id: req.id.clone(),
                        constraints: if req.constraints.is_null() {
                            serde_json::json!({})
                        } else { req.constraints.clone() },
                        source: GrantSource::Auto,
                    });
                }
            }
        }
        for d in &manifest.capabilities.deny { auto.deny(d); }
        Ok((auto, prompt, admin))
    }

    fn build_consent_request(&self, manifest: &Manifest, capability: &str) -> ConsentRequest {
        let req = manifest.capabilities.request.iter().find(|r| r.id == capability);
        ConsentRequest {
            prompt_id: Uuid::now_v7(),
            module: manifest.id.clone(),
            module_name: manifest.name.clone(),
            capability: capability.to_string(),
            tier: req.and_then(|r| r.consent.clone()).unwrap_or_else(|| "prompt".into()),
            reason: req.and_then(|r| r.reason.clone())
                .unwrap_or_else(|| format!("Module requests {capability}")),
            constraints: req.map(|r| if r.constraints.is_null() { serde_json::json!({}) } else { r.constraints.clone() })
                .unwrap_or(serde_json::json!({})),
            target_description: None,
            expires_at: SystemTime::now() + self.consent_timeout,
            can_remember: vec![RememberFor::Session, RememberFor::ModuleVersion, RememberFor::Always],
        }
    }

    fn primary_component_path(
        &self, manifest: &Manifest, artifact_dir: &PathBuf,
    ) -> Result<PathBuf, InstallError> {
        let ep = manifest.entrypoints.get("core")
            .or_else(|| manifest.entrypoints.values().next())
            .ok_or_else(|| InstallError::ModuleHost("manifest has no entrypoints".into()))?;
        Ok(artifact_dir.join(&ep.path))
    }
}

fn step(name: &str, ok: bool, msg: &str) -> TranscriptStep {
    TranscriptStep { step: name.into(), ok, message: msg.into(), details: None }
}

fn step_with_details(name: &str, ok: bool, msg: &str, details: serde_json::Value) -> TranscriptStep {
    TranscriptStep { step: name.into(), ok, message: msg.into(), details: Some(details) }
}