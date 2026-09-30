//! Signing and verification.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature as Ed25519Signature, Signer as _, SigningKey, Verifier, VerifyingKey};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{
    canonical::canonicalize,
    manifest::{ManifestError, Signature},
    trust::{SigAlg, TrustStore},
};

#[derive(Clone, Debug)]
pub struct VerifiedSigner {
    pub key_id: String,
    pub role: String,
    pub alg: SigAlg,
}

#[derive(Clone, Debug)]
pub struct SignatureInfo {
    pub signers: Vec<VerifiedSigner>,
}

pub fn sign_manifest(
    manifest: &mut Value,
    signing_key: &SigningKey,
    key_id: &str,
    role: &str,
) -> Result<(), ManifestError> {
    let obj = manifest
        .as_object_mut()
        .ok_or_else(|| ManifestError::MissingField("manifest must be an object".into()))?;
    obj.remove("signature");
    let canonical = canonicalize(manifest)?;
    let digest_bytes = Sha256::digest(canonical.as_bytes());
    let payload_digest = format!("sha-256:{}", URL_SAFE_NO_PAD.encode(digest_bytes));
    let signature = signing_key.sign(&digest_bytes);
    let sig_value = serde_json::json!({
        "payloadDigest": { "alg": "sha-256", "value": payload_digest },
        "signers": [{
            "keyId": key_id,
            "alg": "EdDSA",
            "role": role,
            "value": URL_SAFE_NO_PAD.encode(signature.to_bytes()),
        }],
    });
    manifest
        .as_object_mut()
        .unwrap()
        .insert("signature".to_string(), sig_value);
    Ok(())
}

pub fn verify_manifest(
    manifest: &Value,
    trust: &dyn TrustStore,
) -> Result<SignatureInfo, ManifestError> {
    let sig_value = manifest
        .get("signature")
        .ok_or(ManifestError::MissingSignature)?;
    let sig: Signature = serde_json::from_value(sig_value.clone())?;

    let mut without_sig = manifest.clone();
    without_sig
        .as_object_mut()
        .ok_or_else(|| ManifestError::MissingField("manifest must be an object".into()))?
        .remove("signature");

    let canonical = canonicalize(&without_sig)?;
    let digest_bytes = Sha256::digest(canonical.as_bytes());
    let actual = format!("sha-256:{}", URL_SAFE_NO_PAD.encode(digest_bytes));
    if actual != sig.payload_digest.value {
        return Err(ManifestError::DigestMismatch {
            expected: sig.payload_digest.value.clone(),
            actual,
        });
    }

    let mut verified = Vec::with_capacity(sig.signers.len());
    for signer in &sig.signers {
        let alg = SigAlg::parse(&signer.alg)
            .ok_or_else(|| ManifestError::UnsupportedAlgorithm(signer.alg.clone()))?;
        let key = trust
            .resolve(&signer.key_id)
            .ok_or_else(|| ManifestError::UnknownKey(signer.key_id.clone()))?;
        if key.revoked {
            return Err(ManifestError::RevokedKey(signer.key_id.clone()));
        }
        if key.alg != alg {
            return Err(ManifestError::BadSignature {
                key_id: signer.key_id.clone(),
                reason: format!("key algorithm mismatch: manifest={alg:?} trust={:?}", key.alg),
            });
        }
        let sig_bytes = URL_SAFE_NO_PAD
            .decode(&signer.value)
            .map_err(|e| ManifestError::BadSignature {
                key_id: signer.key_id.clone(),
                reason: format!("bad base64: {e}"),
            })?;
        verify_signature(alg, &key.public_key, &digest_bytes, &sig_bytes).map_err(|reason| {
            ManifestError::BadSignature {
                key_id: signer.key_id.clone(),
                reason,
            }
        })?;
        verified.push(VerifiedSigner {
            key_id: signer.key_id.clone(),
            role: signer.role.clone(),
            alg,
        });
    }

    let has_trusted = verified
        .iter()
        .any(|s| matches!(s.role.as_str(), "author" | "publisher"));
    if !has_trusted {
        return Err(ManifestError::MissingTrustedSigner);
    }

    Ok(SignatureInfo { signers: verified })
}

fn verify_signature(
    alg: SigAlg,
    public_key: &[u8],
    digest: &[u8],
    signature: &[u8],
) -> Result<(), String> {
    match alg {
        SigAlg::EdDSA => {
            let vk = VerifyingKey::from_bytes(
                public_key.try_into().map_err(|_| "Ed25519 public key must be 32 bytes".to_string())?,
            )
            .map_err(|e| format!("invalid Ed25519 key: {e}"))?;
            let sig = Ed25519Signature::from_slice(signature)
                .map_err(|e| format!("invalid Ed25519 signature: {e}"))?;
            vk.verify(digest, &sig).map_err(|e| format!("Ed25519 verify: {e}"))
        }
        SigAlg::Es256 | SigAlg::Es384 | SigAlg::Ps256 => {
            Err("only Ed25519 is implemented in this build".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trust::StaticTrustStore;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    use serde_json::json;

    fn key_pair() -> (SigningKey, StaticTrustStore) {
        let signing = SigningKey::generate(&mut OsRng);
        let mut trust = StaticTrustStore::new();
        trust.insert("test-key", crate::trust::ResolvedKey {
            alg: SigAlg::EdDSA,
            public_key: signing.verifying_key().to_bytes().to_vec(),
            revoked: false,
        });
        (signing, trust)
    }

    fn base_manifest() -> Value {
        json!({
            "manifestVersion": 1, "id": "com.example.x", "version": "0.1.0",
            "runtime": "*", "name": "X",
            "entrypoints": { "core": { "kind": "wasm", "path": "x.wasm", "digest": { "alg": "sha-256", "value": "AAAA" } } },
            "capabilities": { "request": [] }
        })
    }

    #[test]
    fn sign_then_verify() {
        let (signing, trust) = key_pair();
        let mut m = base_manifest();
        sign_manifest(&mut m, &signing, "test-key", "author").unwrap();
        let info = verify_manifest(&m, &trust).unwrap();
        assert_eq!(info.signers[0].key_id, "test-key");
    }

    #[test]
    fn tampering_invalidates() {
        let (signing, trust) = key_pair();
        let mut m = base_manifest();
        sign_manifest(&mut m, &signing, "test-key", "author").unwrap();
        m["name"] = json!("Tampered");
        assert!(matches!(verify_manifest(&m, &trust), Err(ManifestError::DigestMismatch { .. })));
    }

    #[test]
    fn unknown_key_rejected() {
        let (signing, _) = key_pair();
        let mut m = base_manifest();
        sign_manifest(&mut m, &signing, "test-key", "author").unwrap();
        let trust = StaticTrustStore::new();
        assert!(matches!(verify_manifest(&m, &trust), Err(ManifestError::UnknownKey(_))));
    }

    #[test]
    fn only_reviewer_role_rejected() {
        let (signing, trust) = key_pair();
        let mut m = base_manifest();
        sign_manifest(&mut m, &signing, "test-key", "reviewer").unwrap();
        assert!(matches!(verify_manifest(&m, &trust), Err(ManifestError::MissingTrustedSigner)));
    }
}