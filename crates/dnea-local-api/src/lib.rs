//! The DNEA Local API and embedded companion UI.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

mod auth;
mod module_ui;
mod routes;
mod state;
mod static_assets;
mod ws;

pub use auth::{mint_token, verify_token, Claims, Scope};
pub use state::{ApiConfig, ApiState, LocalApi, LocalApiError};