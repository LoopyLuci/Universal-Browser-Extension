//! DNEA core types and the wire codec.
//!
//! This crate has no I/O. It defines the envelope, the chunking codec,
//! and the error taxonomy. Every transport (Native Messaging, Local
//! API, IPC) reuses this crate verbatim.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

pub mod codec;
pub mod envelope;
pub mod error;
pub mod ids;

pub use codec::{Codec, Decoded, CHUNK_TIMEOUT, DEFAULT_MTU, MAX_CHUNKS, OVERHEAD_BUDGET};
pub use envelope::{
    Envelope, EnvelopeError, Encoding, MessageType, Party, TransportDirection, PROTOCOL_VERSION,
};
pub use error::{DneaError, DneaResult, DenyReason, FramingError};
pub use ids::{EnvelopeId, ModuleId, PortId, SessionId};