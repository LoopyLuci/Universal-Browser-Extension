//! The trust store.

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SigAlg { EdDSA, Es256, Es384, Ps256 }

impl SigAlg {
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "EdDSA" => Self::EdDSA,
            "ES256" => Self::Es256,
            "ES384" => Self::Es384,
            "PS256" => Self::Ps256,
            _ => return None,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ResolvedKey {
    pub alg: SigAlg,
    pub public_key: Vec<u8>,
    pub revoked: bool,
}

pub trait TrustStore: Send + Sync + 'static {
    fn resolve(&self, key_id: &str) -> Option<ResolvedKey>;
}

pub struct StaticTrustStore {
    keys: HashMap<String, ResolvedKey>,
}

impl StaticTrustStore {
    #[must_use]
    pub fn new() -> Self { Self { keys: HashMap::new() } }

    pub fn insert(&mut self, key_id: impl Into<String>, key: ResolvedKey) {
        self.keys.insert(key_id.into(), key);
    }
}

impl Default for StaticTrustStore {
    fn default() -> Self { Self::new() }
}

impl TrustStore for StaticTrustStore {
    fn resolve(&self, key_id: &str) -> Option<ResolvedKey> {
        self.keys.get(key_id).cloned()
    }
}