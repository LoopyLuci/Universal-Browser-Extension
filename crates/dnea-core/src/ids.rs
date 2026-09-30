//! Strongly-typed identifiers.

use std::{fmt, str::FromStr};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use crate::{DneaError, DneaResult};

macro_rules! uuid_newtype {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self { Self(Uuid::now_v7()) }

            #[must_use]
            pub fn as_uuid(&self) -> Uuid { self.0 }

            #[must_use]
            pub fn is_nil(&self) -> bool { self.0.is_nil() }
        }

        impl Default for $name {
            fn default() -> Self { Self::new() }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self(Uuid::parse_str(s)?)) }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(&self.0.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let s = String::deserialize(d)?;
                Uuid::parse_str(&s).map(Self).map_err(de::Error::custom)
            }
        }
    };
}

uuid_newtype!(EnvelopeId, "A protocol envelope ID (UUIDv7).");
uuid_newtype!(SessionId,  "A runtime session ID (UUIDv7).");
uuid_newtype!(PortId,     "A content-script port ID (UUIDv7).");

/// A module identifier, e.g. `com.example.focus-mode`.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModuleId(String);

impl ModuleId {
    /// Parses a module ID, enforcing the reverse-DNS grammar.
    pub fn parse(s: impl Into<String>) -> DneaResult<Self> {
        static RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
            regex::Regex::new(
                r"^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?(\.[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?){1,}$",
            )
            .expect("module ID regex is a compile-time constant")
        });
        let s = s.into();
        if s.len() > 255 || !RE.is_match(&s) {
            return Err(DneaError::InvalidModuleId(s));
        }
        Ok(Self(s))
    }

    #[must_use]
    pub fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

impl fmt::Debug for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ModuleId({})", self.0)
    }
}

impl Serialize for ModuleId {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ModuleId {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Self::parse(s).map_err(de::Error::custom)
    }
}

impl FromStr for ModuleId {
    type Err = DneaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Self::parse(s) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_id_accepts_valid() {
        for s in ["com.example.focus-mode", "org.dnea.standard-ui", "a.b"] {
            assert!(ModuleId::parse(s).is_ok(), "should accept {s}");
        }
    }

    #[test]
    fn module_id_rejects_invalid() {
        for s in ["", "nodot", "Com.Example", "com..example", "com.example.", "-x.y", "x-.y"] {
            assert!(ModuleId::parse(s).is_err(), "should reject {s}");
        }
        assert!(ModuleId::parse("a".repeat(300)).is_err());
    }

    #[test]
    fn uuid_v7_is_time_ordered() {
        let a = EnvelopeId::new();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let b = EnvelopeId::new();
        assert!(a < b, "UUIDv7 must be time-sortable");
    }

    #[test]
    fn module_id_serde_round_trip() {
        let id = ModuleId::parse("com.example.focus-mode").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"com.example.focus-mode\"");
        let back: ModuleId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, id);
    }
}