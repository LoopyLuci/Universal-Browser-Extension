//! The Wasm module host.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

pub mod engine;
pub mod grant;
pub mod host;
pub mod instance;
pub mod link;

pub use engine::{build_engine, EngineConfig};
pub use grant::{EffectiveGrant, GrantSet, GrantSource};
pub use instance::{InstanceHandle, InstanceMessage, ModuleHost, ModuleHostError};