//! The capability vocabulary.

use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

static CAP_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z][a-z0-9]*(:[a-z0-9*_-]+)+$").expect("capability regex is a constant")
});

#[must_use]
pub fn is_valid_capability_id(s: &str) -> bool {
    !s.is_empty() && s.len() <= 128 && CAP_RE.is_match(s)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConsentTier { Auto, Prompt, PerUse, Admin, Forbidden }

impl ConsentTier {
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "auto" => Self::Auto,
            "prompt" => Self::Prompt,
            "per-use" => Self::PerUse,
            "admin" => Self::Admin,
            "forbidden" => Self::Forbidden,
            _ => return None,
        })
    }
}

pub struct CapabilityRegistry {
    tiers: HashMap<&'static str, ConsentTier>,
}

impl CapabilityRegistry {
    #[must_use]
    pub fn v1() -> Self {
        use ConsentTier::*;
        let entries: &[(&str, ConsentTier)] = &[
            ("fs:read", Prompt), ("fs:write", Prompt), ("fs:append", Prompt),
            ("fs:list", Auto), ("fs:watch", Prompt), ("fs:exec", Admin), ("fs:mount", Admin),
            ("net:connect", Prompt), ("net:listen", Admin), ("net:proxy", Admin),
            ("net:dns:resolve", Auto), ("net:websocket", Prompt), ("net:udp", Admin),
            ("browser:identity", Auto),
            ("browser:tabs:query", Prompt), ("browser:tabs:create", Prompt),
            ("browser:tabs:update", Prompt), ("browser:tabs:close", Prompt),
            ("browser:windows:query", Prompt),
            ("browser:history:read", Prompt), ("browser:history:delete", PerUse),
            ("browser:bookmarks:read", Auto), ("browser:bookmarks:write", Prompt),
            ("browser:cookies:read", Prompt), ("browser:cookies:write", PerUse),
            ("browser:downloads:read", Auto), ("browser:downloads:manage", Prompt),
            ("browser:content:read", Prompt), ("browser:content:inject", Prompt),
            ("browser:content:mutate", PerUse),
            ("browser:forms:read", Prompt), ("browser:forms:values", PerUse),
            ("browser:forms:fill", PerUse),
            ("browser:navigation:intercept", Admin),
            ("browser:clipboard:read", PerUse), ("browser:clipboard:write", Prompt),
            ("browser:a11y:tree:read", Admin), ("browser:a11y:input:inject", Admin),
            ("ui:companion:panel", Auto), ("ui:companion:notify", Prompt),
            ("ui:webext:popup", Auto), ("ui:webext:content", Prompt),
            ("ui:overlay:fullscreen", PerUse),
            ("storage:module", Auto), ("storage:shared", Prompt), ("storage:secure", Admin),
            ("runtime:log", Auto), ("runtime:modules:list", Auto),
            ("runtime:modules:control", Admin),
            ("runtime:config:read", Auto), ("runtime:config:write", Admin),
            ("runtime:identity:read", Prompt), ("runtime:spawn", Admin), ("runtime:schedule", Auto),
            ("notifications:send", Prompt),
            ("identity:crypto:sign", Admin), ("identity:crypto:verify", Auto),
            ("system:autostart", Admin), ("system:tray", Auto), ("system:power", Admin),
            ("media:capture", Admin),
            ("policy:read", Auto), ("audit:read", Admin), ("telemetry:emit", Prompt),
        ];
        Self { tiers: entries.iter().copied().collect() }
    }

    #[must_use]
    pub fn is_known(&self, id: &str) -> bool { self.tiers.contains_key(id) }

    #[must_use]
    pub fn default_tier(&self, id: &str) -> ConsentTier {
        self.tiers.get(id).copied().unwrap_or(ConsentTier::Prompt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_grammar() {
        assert!(is_valid_capability_id("fs:read"));
        assert!(is_valid_capability_id("browser:tabs:query"));
        assert!(!is_valid_capability_id(""));
        assert!(!is_valid_capability_id("nocolon"));
        assert!(!is_valid_capability_id("Upper:Case"));
    }
}