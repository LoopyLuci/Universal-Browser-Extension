//! The DNEA error taxonomy.

use std::fmt;

use thiserror::Error;

/// The crate's result alias.
pub type DneaResult<T, E = DneaError> = std::result::Result<T, E>;

/// A DNEA error.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DneaError {
    #[error("invalid module id: {0}")]
    InvalidModuleId(String),

    #[error("invalid envelope: {0}")]
    InvalidEnvelope(String),

    #[error("handshake rejected: {reason}")]
    HandshakeRejected { reason: String },

    #[error("unsupported protocol version {got}; supported: {supported}")]
    UnsupportedVersion { got: u32, supported: u32 },

    #[error("framing error: {0}")]
    Framing(#[from] FramingError),

    #[error("correlation error: unknown reply_to {0}")]
    UnknownCorrelation(String),

    #[error("capability denied: {capability} ({reason:?})")]
    CapabilityDenied { capability: String, reason: DenyReason },

    #[error("constraint violated for {capability}: {detail}")]
    ConstraintViolation { capability: String, detail: String },

    #[error("policy required: {policy_key}")]
    AdminRequired { policy_key: String },

    #[error("rate limited; retry after {retry_after_ms}ms")]
    RateLimited { retry_after_ms: u64 },

    #[error("module error: {module} {code}: {message}")]
    Module { module: String, code: String, message: String },

    #[error("module unavailable: {0}")]
    ModuleUnavailable(String),

    #[error("module quarantined: {0}")]
    ModuleQuarantined(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error("manifest error: {0}")]
    Manifest(String),

    #[error("signature verification failed: {0}")]
    Signature(String),

    #[error("wasm error: {0}")]
    Wasm(String),

    #[error("transport disconnected")]
    Transport,

    #[error("timeout after {0}ms")]
    Timeout(u64),

    #[error("unsupported feature: {0}")]
    UnsupportedFeature(String),

    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DenyReason {
    NotGranted,
    ModuleDenied,
    ConsentExpired,
    PolicyForbidden,
    PolicyAdminRequired,
}

#[derive(Debug, Clone, Error)]
pub enum FramingError {
    #[error("message too large ({actual} > {limit})")]
    TooLarge { actual: usize, limit: usize },
    #[error("chunk overflow for {cid} ({total})")]
    ChunkOverflow { cid: String, total: u32 },
    #[error("digest mismatch for {cid}")]
    DigestMismatch { cid: String },
    #[error("invalid JSON: {0}")]
    InvalidJson(String),
    #[error("missing envelope")]
    MissingEnvelope,
}

impl DneaError {
    /// The stable wire code.
    #[must_use]
    pub fn code(&self) -> &'static str {
        use DneaError::*;
        match self {
            Transport => "E_TRANSPORT_DISCONNECTED",
            UnsupportedVersion { .. } => "E_UNSUPPORTED_VERSION",
            HandshakeRejected { .. } => "E_HANDSHAKE_REJECTED",
            InvalidEnvelope(_) => "E_INVALID_ENVELOPE",
            UnknownCorrelation(_) => "E_UNKNOWN_CORRELATION",
            CapabilityDenied { reason: DenyReason::ModuleDenied, .. }
            | CapabilityDenied { reason: DenyReason::PolicyForbidden, .. } => "E_DENIED",
            CapabilityDenied { .. } => "E_NOT_GRANTED",
            ConstraintViolation { .. } => "E_CONSTRAINT_VIOLATION",
            AdminRequired { .. } => "E_ADMIN_REQUIRED",
            RateLimited { .. } => "E_RATE_LIMITED",
            Module { .. } => "E_MODULE_ERROR",
            ModuleUnavailable(_) => "E_MODULE_UNAVAILABLE",
            ModuleQuarantined(_) => "E_MODULE_QUARANTINED",
            Timeout(_) => "E_TIMEOUT",
            UnsupportedFeature(_) => "E_UNSUPPORTED_FEATURE",
            Framing(FramingError::TooLarge { .. }) => "E_FRAMING_TOO_LARGE",
            Framing(FramingError::ChunkOverflow { .. }) => "E_FRAMING_CHUNK_OVERFLOW",
            Framing(FramingError::DigestMismatch { .. }) => "E_FRAMING_DIGEST_MISMATCH",
            Framing(FramingError::InvalidJson(_)) => "E_FRAMING_INVALID_JSON",
            Framing(FramingError::MissingEnvelope) => "E_INVALID_ENVELOPE",
            Storage(_) | Manifest(_) | Signature(_) | Wasm(_) | Internal(_) => "E_INTERNAL",
            InvalidModuleId(_) => "E_INVALID_ENVELOPE",
        }
    }

    #[must_use]
    pub fn retryable(&self) -> bool {
        use DneaError::*;
        matches!(
            self,
            Transport | Timeout(_) | RateLimited { .. } | ModuleUnavailable(_)
        )
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_variants_have_a_code() {
        let variants: &[DneaError] = &[
            DneaError::InvalidModuleId("x".into()),
            DneaError::InvalidEnvelope("x".into()),
            DneaError::HandshakeRejected { reason: "x".into() },
            DneaError::UnsupportedVersion { got: 2, supported: 1 },
            DneaError::Framing(FramingError::MissingEnvelope),
            DneaError::UnknownCorrelation("x".into()),
            DneaError::CapabilityDenied { capability: "x".into(), reason: DenyReason::NotGranted },
            DneaError::ConstraintViolation { capability: "x".into(), detail: "y".into() },
            DneaError::AdminRequired { policy_key: "x".into() },
            DneaError::RateLimited { retry_after_ms: 100 },
            DneaError::Module { module: "x".into(), code: "y".into(), message: "z".into() },
            DneaError::ModuleUnavailable("x".into()),
            DneaError::ModuleQuarantined("x".into()),
            DneaError::Storage("x".into()),
            DneaError::Manifest("x".into()),
            DneaError::Signature("x".into()),
            DneaError::Wasm("x".into()),
            DneaError::Transport,
            DneaError::Timeout(100),
            DneaError::UnsupportedFeature("x".into()),
            DneaError::Internal("x".into()),
        ];
        for v in variants {
            assert!(v.code().starts_with("E_"), "{v:?} has no code");
        }
    }

    #[test]
    fn code_denied_variants_map_correctly() {
        assert_eq!(
            DneaError::CapabilityDenied { capability: "x".into(), reason: DenyReason::NotGranted }.code(),
            "E_NOT_GRANTED",
        );
        assert_eq!(
            DneaError::CapabilityDenied { capability: "x".into(), reason: DenyReason::ModuleDenied }.code(),
            "E_DENIED",
        );
    }

    #[test]
    fn retryable_is_conservative() {
        assert!(DneaError::Transport.retryable());
        assert!(DneaError::RateLimited { retry_after_ms: 100 }.retryable());
        assert!(!DneaError::CapabilityDenied { capability: "x".into(), reason: DenyReason::NotGranted }.retryable());
        assert!(!DneaError::Internal("x".into()).retryable());
    }
}