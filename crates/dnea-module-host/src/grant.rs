//! The grant set.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EffectiveGrant {
    pub id: String,
    #[serde(default)]
    pub constraints: serde_json::Value,
    pub source: GrantSource,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GrantSource { Auto, UserConsent, Policy, DynamicOrigin }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GrantSet {
    grants: BTreeMap<String, EffectiveGrant>,
    denied: BTreeSet<String>,
}

impl GrantSet {
    #[must_use]
    pub fn new() -> Self { Self::default() }

    pub fn insert(&mut self, grant: EffectiveGrant) {
        if self.denied.contains(&grant.id) { return; }
        self.grants.insert(grant.id.clone(), grant);
    }

    pub fn deny(&mut self, capability: impl Into<String>) {
        let cap = capability.into();
        self.grants.remove(&cap);
        self.denied.insert(cap);
    }

    #[must_use]
    pub fn get(&self, capability: &str) -> Option<&EffectiveGrant> {
        self.grants.get(capability)
    }

    #[must_use]
    pub fn any_of(&self, capabilities: &[&str]) -> bool {
        capabilities.iter().any(|c| self.grants.contains_key(*c))
    }

    #[must_use]
    pub fn all_of(&self, capabilities: &[&str]) -> bool {
        capabilities.iter().all(|c| self.grants.contains_key(*c))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &EffectiveGrant)> {
        self.grants.iter().map(|(k, v)| (k.as_str(), v))
    }

    #[must_use]
    pub fn len(&self) -> usize { self.grants.len() }

    #[must_use]
    pub fn is_empty(&self) -> bool { self.grants.is_empty() }

    #[must_use]
    pub fn is_denied(&self, capability: &str) -> bool {
        self.denied.contains(capability)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(id: &str) -> EffectiveGrant {
        EffectiveGrant { id: id.into(), constraints: serde_json::json!({}), source: GrantSource::Auto }
    }

    #[test]
    fn deny_overrides_grant() {
        let mut s = GrantSet::new();
        s.insert(g("net:connect"));
        assert!(s.get("net:connect").is_some());
        s.deny("net:connect");
        assert!(s.get("net:connect").is_none());
        assert!(s.is_denied("net:connect"));
        // Re-granting is a no-op.
        s.insert(g("net:connect"));
        assert!(s.get("net:connect").is_none());
    }
}