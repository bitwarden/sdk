//! Bitwarden SDK Client

pub(crate) use client::*;
#[allow(clippy::module_inception)]
mod client;
pub mod client_settings;
pub mod encryption_settings;
pub use client::InternalClient;

#[cfg(feature = "internal")]
mod flags;

pub use client::Client;
