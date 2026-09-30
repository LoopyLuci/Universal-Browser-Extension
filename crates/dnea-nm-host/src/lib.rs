//! DNEA Native Messaging host.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

pub mod registration;
pub mod session;
pub mod supervisor;

#[cfg(windows)]
pub mod supervisor_windows;

pub use session::{ClientId, ClientInfo, SessionSink};
pub use supervisor::{NmHostConfig, NmHostError, NmSupervisor};