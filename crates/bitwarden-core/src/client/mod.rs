//! Bitwarden SDK Client

pub(crate) use client::*;
#[allow(clippy::module_inception)]
mod client;
pub mod client_settings;
pub mod encryption_settings;
pub mod internal;
pub use internal::ApiConfigurations;
pub mod login_method;
#[cfg(feature = "internal")]
pub(crate) use login_method::UserLoginMethod;
pub(crate) use login_method::{LoginMethod, ServiceAccountLoginMethod};

#[cfg(feature = "internal")]
mod flags;

pub use client::Client;
pub use client_settings::{ClientSettings, DeviceType};

#[cfg(feature = "internal")]
pub mod test_accounts;
