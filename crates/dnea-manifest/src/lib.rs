//! DNEA module manifests.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

pub mod canonical;
pub mod capability;
pub mod manifest;
pub mod signature;
pub mod trust;
pub mod validate;

pub use canonical::{canonicalize, CanonicalError};
pub use capability::{is_valid_capability_id, CapabilityRegistry, ConsentTier};
pub use manifest::{
    Author, Capabilities, CapabilityRequest, Conflict, Dependency, DynamicOriginGrants,
    Entrypoint, HookRef, Lifecycle, Manifest, ManifestError, Migration, PayloadDigest,
    Signature, Signer, Storage, UiSurface,
};
pub use signature::{sign_manifest, verify_manifest, SignatureInfo, VerifiedSigner};
pub use trust::{ResolvedKey, SigAlg, StaticTrustStore, TrustStore};
pub use validate::{ValidationError, ValidationReport, Validator};