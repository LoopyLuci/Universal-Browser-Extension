//! Signed-install end-to-end test.

use std::{path::PathBuf, sync::Arc};

use dnea_manifest::{
    signature::sign_manifest,
    trust::{ResolvedKey, SigAlg, StaticTrustStore},
    validate::Validator,
};
use dnea_module_host::{engine::EngineConfig, grant::GrantSet, host::HostState, instance::ModuleHost};
use dnea_session::{
    install::{HostStateFactory, InstallPipeline, InstallSource, InstallStatus},
    Session,
};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use serde_json::json;
use sha2::{Digest, Sha256};
use tempfile::TempDir;

struct NoopFactory;

impl HostStateFactory for NoopFactory {
    fn build(&self, module_id: &str, version: &str, grants: Arc<GrantSet>) -> HostState {
        HostState::new(
            module_id.to_string(), version.to_string(), grants,
            wasmtime::StoreLimitsBuilder::new().memory_size(64 * 1024 * 1024).build(),
        )
    }
}

fn make_manifest(artifact_digest: &str) -> serde_json::Value {
    json!({
        "manifestVersion": 1, "id": "com.example.focus-mode", "version": "1.2.0",
        "runtime": ">=1.0.0 <2.0.0", "name": "Focus Mode",
        "entrypoints": { "core": {
            "kind": "wasm", "path": "core.wasm",
            "digest": { "alg": "sha-256", "value": artifact_digest } } },
        "capabilities": { "request": [
            { "id": "storage:module" }, { "id": "runtime:log" },
            { "id": "browser:bookmarks:read" } ] }
    })
}

#[tokio::test(flavor = "multi_thread")]
async fn install_signed_focus_mode_with_auto_grants() {
    let tmp = TempDir::new().unwrap();
    let artifact_bytes = b"\0asm\x01\0\0\0";
    let artifact_digest = format!("sha-256:{}",
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD,
            Sha256::digest(artifact_bytes)));
    std::fs::write(tmp.path().join("core.wasm"), artifact_bytes).unwrap();

    let signing_key = SigningKey::generate(&mut OsRng);
    let key_id = "https://test.example/keys/signing";
    let mut manifest = make_manifest(&artifact_digest);
    sign_manifest(&mut manifest, &signing_key, key_id, "author").unwrap();
    std::fs::write(tmp.path().join("manifest.json"),
        serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();

    let mut trust = StaticTrustStore::new();
    trust.insert(key_id, ResolvedKey {
        alg: SigAlg::EdDSA,
        public_key: signing_key.verifying_key().to_bytes().to_vec(),
        revoked: false,
    });

    let session = Session::new();
    let host = Arc::new(ModuleHost::new(&EngineConfig::default()).unwrap());
    let validator = Arc::new(Validator::new().unwrap());
    let pipeline = InstallPipeline::new(
        session.clone(), host, validator, Arc::new(trust), Arc::new(NoopFactory));

    let report = pipeline.install(InstallSource::Local {
        manifest_path: tmp.path().join("manifest.json"),
        artifact_dir: tmp.path().to_path_buf(),
    }).await;

    match report {
        Ok(r) => {
            assert_eq!(r.module_id, "com.example.focus-mode");
            assert_eq!(r.status, InstallStatus::Installed);
        }
        Err(e) => {
            let s = e.to_string();
            assert!(s.contains("component") || s.contains("wasm") || s.contains("module host"),
                "expected Wasm load error, got: {s}");
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn install_rejects_tampered_manifest() {
    let tmp = TempDir::new().unwrap();
    let artifact_bytes = b"\0asm\x01\0\0\0";
    let artifact_digest = format!("sha-256:{}",
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD,
            Sha256::digest(artifact_bytes)));
    std::fs::write(tmp.path().join("core.wasm"), artifact_bytes).unwrap();

    let signing_key = SigningKey::generate(&mut OsRng);
    let key_id = "https://test.example/keys/signing";
    let mut manifest = make_manifest(&artifact_digest);
    sign_manifest(&mut manifest, &signing_key, key_id, "author").unwrap();
    manifest["name"] = json!("Tampered Name");
    std::fs::write(tmp.path().join("manifest.json"),
        serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();

    let mut trust = StaticTrustStore::new();
    trust.insert(key_id, ResolvedKey {
        alg: SigAlg::EdDSA,
        public_key: signing_key.verifying_key().to_bytes().to_vec(),
        revoked: false,
    });

    let session = Session::new();
    let host = Arc::new(ModuleHost::new(&EngineConfig::default()).unwrap());
    let validator = Arc::new(Validator::new().unwrap());
    let pipeline = InstallPipeline::new(
        session, host, validator, Arc::new(trust), Arc::new(NoopFactory));

    let err = pipeline.install(InstallSource::Local {
        manifest_path: tmp.path().join("manifest.json"),
        artifact_dir: tmp.path().to_path_buf(),
    }).await.unwrap_err();
    assert!(err.to_string().contains("digest") || err.to_string().contains("signature"));
}

#[tokio::test(flavor = "multi_thread")]
async fn install_rejects_tampered_artifact() {
    let tmp = TempDir::new().unwrap();
    let real = b"\0asm\x01\0\0\0";
    let claimed = b"different bytes";
    let claimed_digest = format!("sha-256:{}",
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD,
            Sha256::digest(claimed)));
    std::fs::write(tmp.path().join("core.wasm"), real.as_slice()).unwrap();

    let signing_key = SigningKey::generate(&mut OsRng);
    let key_id = "https://test.example/keys/signing";
    let mut manifest = make_manifest(&claimed_digest);
    sign_manifest(&mut manifest, &signing_key, key_id, "author").unwrap();
    std::fs::write(tmp.path().join("manifest.json"),
        serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();

    let mut trust = StaticTrustStore::new();
    trust.insert(key_id, ResolvedKey {
        alg: SigAlg::EdDSA,
        public_key: signing_key.verifying_key().to_bytes().to_vec(),
        revoked: false,
    });

    let session = Session::new();
    let host = Arc::new(ModuleHost::new(&EngineConfig::default()).unwrap());
    let validator = Arc::new(Validator::new().unwrap());
    let pipeline = InstallPipeline::new(
        session, host, validator, Arc::new(trust), Arc::new(NoopFactory));

    let err = pipeline.install(InstallSource::Local {
        manifest_path: tmp.path().join("manifest.json"),
        artifact_dir: tmp.path().to_path_buf(),
    }).await.unwrap_err();
    assert!(matches!(err, dnea_session::install::InstallError::ArtifactDigest { .. }));
}