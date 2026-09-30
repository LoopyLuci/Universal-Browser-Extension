//! Semantic validation.

use std::collections::HashSet;

use jsonschema::{Draft, JSONSchema};
use serde_json::Value;
use thiserror::Error;

use crate::{
    capability::{is_valid_capability_id, CapabilityRegistry},
    manifest::{Manifest, ManifestError},
};

const SCHEMA_JSON: &str = include_str!("../../../schemas/module-manifest-v1.json");

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("schema validation failed: {0:?}")]
    Schema(Vec<String>),
    #[error("manifest parse: {0}")]
    Manifest(#[from] ManifestError),
    #[error("invalid capability ID: {0}")]
    InvalidCapabilityId(String),
    #[error("unknown capability: {0}")]
    UnknownCapability(String),
    #[error("migration chain broken: missing migration from version {0}")]
    BrokenMigrationChain(u32),
    #[error("UI surface {surface} declares origin {origin} not granted")]
    UiOriginNotGranted { surface: String, origin: String },
    #[error("module declares a conflict with itself")]
    SelfConflict,
    #[error("invalid entrypoint kind: {0}")]
    InvalidEntrypointKind(String),
    #[error("entrypoint path escapes module root: {0}")]
    PathTraversal(String),
    #[error("invalid runtime range: {0}")]
    InvalidRuntimeRange(String),
    #[error("invalid module ID: {0}")]
    InvalidModuleId(String),
    #[error("invalid version: {0}")]
    InvalidVersion(String),
    #[error("dynamicOriginGrants declared but no origin-scoped capability is requested")]
    DynamicOriginsWithoutOriginCapability,
}

#[derive(Debug, Default)]
pub struct ValidationReport {
    pub warnings: Vec<String>,
}

pub struct Validator {
    schema: JSONSchema,
    registry: CapabilityRegistry,
}

impl Validator {
    pub fn new() -> Result<Self, ValidationError> {
        let schema_value: Value = serde_json::from_str(SCHEMA_JSON)
            .map_err(|e| ValidationError::Manifest(ManifestError::Json(e)))?;
        let schema = JSONSchema::options()
            .with_draft(Draft::Draft202012)
            .compile(&schema_value)
            .map_err(|e| ValidationError::Schema(vec![format!("schema compile: {e}")]))?;
        Ok(Self { schema, registry: CapabilityRegistry::v1() })
    }

    pub fn validate(&self, value: &Value) -> Result<ValidationReport, ValidationError> {
        if let Err(errors) = self.schema.validate(value) {
            let msgs: Vec<String> = errors
                .map(|e| format!("{} at {}", e, e.instance_path))
                .collect();
            return Err(ValidationError::Schema(msgs));
        }
        let manifest: Manifest = serde_json::from_value(value.clone())
            .map_err(|e| ValidationError::Manifest(ManifestError::Json(e)))?;

        if dnea_core::ModuleId::parse(&manifest.id).is_err() {
            return Err(ValidationError::InvalidModuleId(manifest.id.clone()));
        }
        if semver_check(&manifest.version).is_err() {
            return Err(ValidationError::InvalidVersion(manifest.version.clone()));
        }
        if semver_range_check(&manifest.runtime).is_err() {
            return Err(ValidationError::InvalidRuntimeRange(manifest.runtime.clone()));
        }

        for (_, ep) in &manifest.entrypoints {
            if !matches!(ep.kind.as_str(), "wasm" | "js" | "native") {
                return Err(ValidationError::InvalidEntrypointKind(ep.kind.clone()));
            }
            if ep.path.starts_with('/') || ep.path.contains("..") {
                return Err(ValidationError::PathTraversal(ep.path.clone()));
            }
        }

        let mut has_origin_capability = false;
        for req in &manifest.capabilities.request {
            if !is_valid_capability_id(&req.id) {
                return Err(ValidationError::InvalidCapabilityId(req.id.clone()));
            }
            if !self.registry.is_known(&req.id) {
                return Err(ValidationError::UnknownCapability(req.id.clone()));
            }
            let c = &req.constraints;
            if c.get("origins").is_some() || c.get("hosts").is_some() {
                has_origin_capability = true;
            }
        }

        if manifest.dynamic_origin_grants.is_some() && !has_origin_capability {
            return Err(ValidationError::DynamicOriginsWithoutOriginCapability);
        }

        self.check_migration_chain(&manifest)?;

        let granted_origins: HashSet<&str> = manifest
            .capabilities
            .request
            .iter()
            .filter(|r| r.id == "browser:content:inject")
            .filter_map(|r| r.constraints.get("origins"))
            .filter_map(|v| v.as_array())
            .flatten()
            .filter_map(|v| v.as_str())
            .collect();
        for surface in &manifest.ui {
            if surface.kind == "content-overlay" {
                for origin in &surface.origins {
                    if !granted_origins.contains(origin.as_str()) {
                        return Err(ValidationError::UiOriginNotGranted {
                            surface: surface.id.clone(),
                            origin: origin.clone(),
                        });
                    }
                }
            }
        }

        for c in &manifest.conflicts {
            if c.id == manifest.id {
                return Err(ValidationError::SelfConflict);
            }
        }

        Ok(ValidationReport::default())
    }

    fn check_migration_chain(&self, manifest: &Manifest) -> Result<(), ValidationError> {
        let target = manifest.storage.storage_schema_version;
        if target <= 1 { return Ok(()); }
        let has: HashSet<u32> = manifest.migrations.iter().map(|m| m.from).collect();
        for v in 1..target {
            if !has.contains(&v) {
                return Err(ValidationError::BrokenMigrationChain(v));
            }
        }
        Ok(())
    }
}

fn semver_check(s: &str) -> Result<(), ()> {
    let bytes = s.as_bytes();
    if bytes.is_empty() { return Err(()); }
    let mut dot_count = 0;
    let mut segment_digits = 0;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'0'..=b'9' => segment_digits += 1,
            b'.' => {
                if segment_digits == 0 { return Err(()); }
                dot_count += 1;
                segment_digits = 0;
            }
            b'-' | b'+' => {
                if dot_count != 2 || segment_digits == 0 { return Err(()); }
                return if i == bytes.len() - 1 { Err(()) } else { Ok(()) };
            }
            _ => return Err(()),
        }
    }
    if dot_count == 2 && segment_digits > 0 { Ok(()) } else { Err(()) }
}

fn semver_range_check(s: &str) -> Result<(), ()> {
    if s.is_empty() { return Err(()); }
    if s == "*" { return Ok(()); }
    if s.chars().any(|c| c.is_ascii_digit()) { Ok(()) } else { Err(()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn minimal() -> Value {
        json!({
            "manifestVersion": 1, "id": "com.example.x", "version": "0.1.0",
            "runtime": ">=1.0.0 <2.0.0", "name": "X",
            "entrypoints": { "core": { "kind": "wasm", "path": "core.wasm", "digest": { "alg": "sha-256", "value": "AAAA" } } },
            "capabilities": { "request": [] }
        })
    }

    #[test]
    fn minimal_manifest_validates() {
        Validator::new().unwrap().validate(&minimal()).unwrap();
    }

    #[test]
    fn unknown_capability_rejected() {
        let mut m = minimal();
        m["capabilities"]["request"] = json!([{ "id": "nonsense:thing" }]);
        assert!(matches!(Validator::new().unwrap().validate(&m), Err(ValidationError::UnknownCapability(_))));
    }

    #[test]
    fn broken_migration_chain_rejected() {
        let mut m = minimal();
        m["storage"] = json!({ "storageSchemaVersion": 3 });
        m["migrations"] = json!([{ "from": 1, "to": 2, "entrypoint": "core", "export": "m1" }]);
        assert!(matches!(Validator::new().unwrap().validate(&m), Err(ValidationError::BrokenMigrationChain(2))));
    }

    #[test]
    fn self_conflict_rejected() {
        let mut m = minimal();
        m["conflicts"] = json!([{ "id": "com.example.x" }]);
        assert!(matches!(Validator::new().unwrap().validate(&m), Err(ValidationError::SelfConflict)));
    }
}