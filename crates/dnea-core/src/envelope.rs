//! The wire envelope.

use std::fmt;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use time::OffsetDateTime;

use crate::{
    error::DneaError,
    ids::{EnvelopeId, ModuleId, PortId},
    DneaResult,
};

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Party {
    Runtime,
    Client,
    ContentScript(PortId),
    Module(ModuleId),
}

impl fmt::Debug for Party {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Party::Runtime => f.write_str("Runtime"),
            Party::Client => f.write_str("Client"),
            Party::ContentScript(id) => write!(f, "ContentScript({id})"),
            Party::Module(id) => write!(f, "Module({id})"),
        }
    }
}

impl Serialize for Party {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Party::Runtime => s.serialize_str("runtime"),
            Party::Client => s.serialize_str("client"),
            Party::ContentScript(id) => s.serialize_str(&format!("cs:{id}")),
            Party::Module(id) => s.serialize_str(&format!("mod:{id}")),
        }
    }
}

impl<'de> Deserialize<'de> for Party {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        if s == "runtime" { return Ok(Party::Runtime); }
        if s == "client" { return Ok(Party::Client); }
        if let Some(rest) = s.strip_prefix("cs:") {
            let id: PortId = rest.parse().map_err(de::Error::custom)?;
            return Ok(Party::ContentScript(id));
        }
        if let Some(rest) = s.strip_prefix("mod:") {
            let id = ModuleId::parse(rest).map_err(de::Error::custom)?;
            return Ok(Party::Module(id));
        }
        Err(de::Error::custom(format!("unknown party: {s}")))
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MessageType {
    Hello, Welcome, Ping, Pong, Goodbye, Error,
    Invoke, Result, Event, Subscribe, Unsubscribe, Chunk,
    CsRegister, CsUnregister, CsForward,
    CapabilityList, CapabilityListResult, CapabilityChanged,
    ConsentRequest, ConsentResponse,
    UiPanel, UiPopup, UiNotify, UiInject, UiClear,
    Module(String),
}

impl MessageType {
    #[must_use]
    pub fn as_str(&self) -> &str {
        use MessageType::*;
        match self {
            Hello => "dnea/hello",
            Welcome => "dnea/welcome",
            Ping => "dnea/ping",
            Pong => "dnea/pong",
            Goodbye => "dnea/goodbye",
            Error => "dnea/error",
            Invoke => "dnea/invoke",
            Result => "dnea/result",
            Event => "dnea/event",
            Subscribe => "dnea/subscribe",
            Unsubscribe => "dnea/unsubscribe",
            Chunk => "dnea/chunk",
            CsRegister => "dnea/cs/register",
            CsUnregister => "dnea/cs/unregister",
            CsForward => "dnea/cs/forward",
            CapabilityList => "dnea/capability/list",
            CapabilityListResult => "dnea/capability/list/result",
            CapabilityChanged => "dnea/capability/changed",
            ConsentRequest => "dnea/consent/request",
            ConsentResponse => "dnea/consent/response",
            UiPanel => "dnea/ui/panel",
            UiPopup => "dnea/ui/popup",
            UiNotify => "dnea/ui/notify",
            UiInject => "dnea/ui/inject",
            UiClear => "dnea/ui/clear",
            Module(s) => s,
        }
    }

    pub fn parse(s: &str) -> DneaResult<Self> {
        use MessageType::*;
        Ok(match s {
            "dnea/hello" => Hello,
            "dnea/welcome" => Welcome,
            "dnea/ping" => Ping,
            "dnea/pong" => Pong,
            "dnea/goodbye" => Goodbye,
            "dnea/error" => Error,
            "dnea/invoke" => Invoke,
            "dnea/result" => Result,
            "dnea/event" => Event,
            "dnea/subscribe" => Subscribe,
            "dnea/unsubscribe" => Unsubscribe,
            "dnea/chunk" => Chunk,
            "dnea/cs/register" => CsRegister,
            "dnea/cs/unregister" => CsUnregister,
            "dnea/cs/forward" => CsForward,
            "dnea/capability/list" => CapabilityList,
            "dnea/capability/list/result" => CapabilityListResult,
            "dnea/capability/changed" => CapabilityChanged,
            "dnea/consent/request" => ConsentRequest,
            "dnea/consent/response" => ConsentResponse,
            "dnea/ui/panel" => UiPanel,
            "dnea/ui/popup" => UiPopup,
            "dnea/ui/notify" => UiNotify,
            "dnea/ui/inject" => UiInject,
            "dnea/ui/clear" => UiClear,
            other if other.starts_with("mod/") => Module(other.to_string()),
            other => return Err(DneaError::InvalidEnvelope(format!("unknown message type: {other}"))),
        })
    }
}

impl fmt::Debug for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MessageType({})", self.as_str())
    }
}

impl Serialize for MessageType {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        MessageType::parse(&s).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Encoding { #[default] Identity, Zstd, Br }

impl Serialize for Encoding {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(match self { Encoding::Identity => "identity", Encoding::Zstd => "zstd", Encoding::Br => "br" })
    }
}

impl<'de> Deserialize<'de> for Encoding {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        match s.as_str() {
            "identity" => Ok(Encoding::Identity),
            "zstd" => Ok(Encoding::Zstd),
            "br" => Ok(Encoding::Br),
            other => Err(de::Error::custom(format!("unknown encoding: {other}"))),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvelopeError {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub retryable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Envelope {
    pub v: u32,
    pub id: EnvelopeId,
    #[serde(rename = "type")]
    pub kind: MessageType,
    pub ts: OffsetDateTime,
    pub src: Party,
    pub dst: Party,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<EnvelopeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<ModuleId>,
    #[serde(default, skip_serializing_if = "is_identity")]
    pub enc: Encoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<EnvelopeError>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub meta: serde_json::Map<String, Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportDirection { Client, Runtime }

fn is_identity(e: &Encoding) -> bool { matches!(e, Encoding::Identity) }
fn is_false(b: &bool) -> bool { !*b }

impl Envelope {
    #[must_use]
    pub fn new(kind: MessageType, src: Party, dst: Party) -> Self {
        Self {
            v: PROTOCOL_VERSION,
            id: EnvelopeId::new(),
            kind,
            ts: OffsetDateTime::now_utc(),
            src,
            dst,
            reply_to: None,
            cap: None,
            module: None,
            enc: Encoding::Identity,
            ttl_ms: None,
            payload: None,
            err: None,
            meta: Default::default(),
        }
    }

    #[must_use]
    pub fn reply(&self, kind: MessageType) -> Self {
        Self {
            v: self.v,
            id: EnvelopeId::new(),
            kind,
            ts: OffsetDateTime::now_utc(),
            src: self.dst.clone(),
            dst: self.src.clone(),
            reply_to: Some(self.id),
            cap: None,
            module: None,
            enc: Encoding::Identity,
            ttl_ms: None,
            payload: None,
            err: None,
            meta: Default::default(),
        }
    }

    #[must_use]
    pub fn from_error(req: &Envelope, err: &DneaError) -> Self {
        let mut out = req.reply(MessageType::Error);
        out.err = Some(EnvelopeError {
            code: err.code().into(),
            message: err.to_string(),
            retryable: err.retryable(),
            retry_after_ms: match err {
                DneaError::RateLimited { retry_after_ms } => Some(*retry_after_ms),
                _ => None,
            },
            details: None,
            cause: None,
        });
        out
    }

    pub fn validate_direction(&self, direction: TransportDirection) -> DneaResult<()> {
        let ok = matches!(
            (direction, &self.src),
            (TransportDirection::Client, Party::Client)
                | (TransportDirection::Client, Party::ContentScript(_))
                | (TransportDirection::Runtime, Party::Runtime)
                | (TransportDirection::Runtime, Party::Module(_))
        );
        if ok { Ok(()) } else {
            Err(DneaError::InvalidEnvelope(format!(
                "envelope src={:?} is inconsistent with {direction:?} transport", self.src
            )))
        }
    }
}

impl Default for Envelope {
    fn default() -> Self { Self::new(MessageType::Ping, Party::Runtime, Party::Client) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::PortId;

    #[test]
    fn message_type_round_trips() {
        for t in [
            MessageType::Hello, MessageType::Welcome, MessageType::Ping,
            MessageType::Pong, MessageType::Goodbye, MessageType::Error,
            MessageType::ConsentRequest, MessageType::UiInject,
            MessageType::Module("mod/com.example.focus-mode/getState".into()),
        ] {
            assert_eq!(MessageType::parse(t.as_str()).unwrap(), t);
        }
    }

    #[test]
    fn party_round_trips() {
        let parties = [
            Party::Runtime, Party::Client,
            Party::ContentScript(PortId::new()),
            Party::Module(ModuleId::parse("com.example.focus-mode").unwrap()),
        ];
        for p in parties {
            let json = serde_json::to_string(&p).unwrap();
            let back: Party = serde_json::from_str(&json).unwrap();
            assert_eq!(back, p);
        }
    }

    #[test]
    fn reply_swaps_src_and_dst() {
        let req = Envelope::new(MessageType::Ping, Party::Client, Party::Runtime);
        let rep = req.reply(MessageType::Pong);
        assert_eq!(rep.src, Party::Runtime);
        assert_eq!(rep.dst, Party::Client);
        assert_eq!(rep.reply_to, Some(req.id));
    }

    #[test]
    fn direction_validation_enforced() {
        let client_env = Envelope::new(MessageType::Ping, Party::Client, Party::Runtime);
        assert!(client_env.validate_direction(TransportDirection::Client).is_ok());
        assert!(client_env.validate_direction(TransportDirection::Runtime).is_err());
    }
}