//! Bitwarden SDK Client

#[allow(clippy::module_inception)]
mod client;
pub mod client_settings;
pub mod encryption_settings;
pub mod internal;
pub use internal::ApiConfigurations;
pub mod login_method;
#[cfg(feature = "secrets")]
pub(crate) use login_method::ServiceAccountLoginMethod;
pub(crate) use login_method::{LoginMethod, UserLoginMethod};
#[cfg(feature = "internal")]
mod flags;

pub use client::Client;
pub use client_settings::{ClientSettings, DeviceType};

#[cfg(feature = "internal")]
pub mod test_accounts;
