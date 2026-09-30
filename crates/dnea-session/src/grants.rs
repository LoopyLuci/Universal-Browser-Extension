//! The grant store.

use std::collections::HashMap;
use std::time::SystemTime;

use dnea_core::ids::ModuleId;
use dnea_module_host::grant::{EffectiveGrant, GrantSet, GrantSource};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GrantUpdate {
    pub module: String,
    pub capability: String,
    pub change: GrantChange,
    pub effective_constraints: serde_json::Value,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GrantChange { Granted, Revoked, Widened, Narrowed }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OriginGrant {
    pub module: String,
    pub capability: String,
    pub origin: String,
    pub granted_at: SystemTime,
    pub expires_at: Option<SystemTime>,
}

#[derive(Default)]
pub struct GrantStore {
    inner: RwLock<HashMap<ModuleId, GrantSet>>,
    origins: RwLock<HashMap<(String, String, String), OriginGrant>>,
}

impl GrantStore {
    pub fn set(&self, module: ModuleId, grants: GrantSet) {
        self.inner.write().insert(module, grants);
    }

    pub fn get(&self, module: &ModuleId) -> Option<GrantSet> {
        self.inner.read().get(module).cloned()
    }

    pub fn grant(
        &self,
        module: &ModuleId,
        capability: &str,
        constraints: serde_json::Value,
        source: GrantSource,
    ) -> Option<GrantUpdate> {
        let mut guard = self.inner.write();
        let set = guard.entry(module.clone()).or_default();
        let existed = set.get(capability).is_some();
        set.insert(EffectiveGrant {
            id: capability.to_string(),
            constraints: constraints.clone(),
            source,
        });
        if existed { None } else {
            Some(GrantUpdate {
                module: module.to_string(),
                capability: capability.to_string(),
                change: GrantChange::Granted,
                effective_constraints: constraints,
            })
        }
    }

    pub fn revoke(&self, module: &ModuleId, capability: &str) -> Option<GrantUpdate> {
        let mut guard = self.inner.write();
        let set = guard.get_mut(module)?;
        if set.get(capability).is_none() { return None; }
        let mut new_set = GrantSet::new();
        for (id, g) in set.iter() {
            if id != capability { new_set.insert(g.clone()); }
        }
        *set = new_set;
        Some(GrantUpdate {
            module: module.to_string(),
            capability: capability.to_string(),
            change: GrantChange::Revoked,
            effective_constraints: serde_json::Value::Null,
        })
    }

    pub fn modules(&self) -> Vec<ModuleId> {
        self.inner.read().keys().cloned().collect()
    }

    pub fn add_origin(
        &self,
        module: &ModuleId,
        capability: &str,
        origin: &str,
        expires_at: Option<SystemTime>,
    ) -> OriginGrant {
        let rec = OriginGrant {
            module: module.to_string(),
            capability: capability.to_string(),
            origin: origin.to_string(),
            granted_at: SystemTime::now(),
            expires_at,
        };
        self.origins.write().insert(
            (module.to_string(), capability.to_string(), origin.to_string()),
            rec.clone(),
        );
        rec
    }

    pub fn remove_origin(&self, module: &ModuleId, capability: &str, origin: &str) -> bool {
        self.origins
            .write()
            .remove(&(module.to_string(), capability.to_string(), origin.to_string()))
            .is_some()
    }

    pub fn origins_for(&self, module: &ModuleId, capability: &str) -> Vec<OriginGrant> {
        self.origins
            .read()
            .iter()
            .filter(|((m, c, _), _)| m == module.as_str() && c == capability)
            .map(|(_, v)| v.clone())
            .collect()
    }

    #[must_use]
    pub fn has_origin(&self, module: &ModuleId, capability: &str, origin: &str) -> bool {
        let key = (module.to_string(), capability.to_string(), origin.to_string());
        let guard = self.origins.read();
        match guard.get(&key) {
            Some(g) => g.expires_at.map(|e| e > SystemTime::now()).unwrap_or(true),
            None => false,
        }
    }
}