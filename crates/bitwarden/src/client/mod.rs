//! Bitwarden SDK Client

pub(crate) use client::*;
#[allow(clippy::module_inception)]
mod client;
pub mod client_settings;
pub(crate) mod encryption_settings;

#[cfg(feature = "internal")]
mod flags;

pub use client::Client;
pub use client_settings::{ClientSettings, DeviceType};

#[cfg(feature = "internal")]
#[cfg(test)]
pub(crate) mod test_accounts;
