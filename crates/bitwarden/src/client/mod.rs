//! Bitwarden SDK Client

pub(crate) use client::*;
pub(crate) mod access_token;
#[cfg(any(feature = "internal", feature = "mobile"))]
pub mod auth_settings;
#[allow(clippy::module_inception)]
mod client;
pub mod client_settings;
pub(crate) mod encryption_settings;

pub use access_token::AccessToken;
pub use client::Client;
