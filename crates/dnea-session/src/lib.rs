//! The DNEA runtime hub.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

mod consent;
mod grants;
mod hub;
pub mod install;
mod registry;

pub use consent::{ConsentDecision, ConsentRequest, ConsentState, RememberFor};
pub use grants::{GrantChange, GrantStore, GrantUpdate, OriginGrant};
pub use hub::{Session, SessionError, SessionEvent, SessionHandle};
pub use install::{
    HostStateFactory, InstallError, InstallPipeline, InstallReport, InstallSource, InstallStatus,
    TranscriptStep,
};
pub use registry::{ClientRecord, ClientRegistry, ModuleRecord, ModuleRegistry};