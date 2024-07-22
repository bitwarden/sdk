#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
#[cfg(feature = "uniffi")]
mod uniffi_support;

#[cfg(feature = "internal")]
pub mod admin_console;
pub mod auth;
pub mod client;
mod error;
pub use error::Error;
#[cfg(feature = "internal")]
pub mod mobile;
pub use error::{MissingFieldError, VaultLocked};
mod database;
pub use database::{Database, DatabaseError};
#[cfg(feature = "internal")]
pub mod platform;
#[cfg(feature = "secrets")]
pub mod secrets_manager;
mod util;

pub use bitwarden_crypto::ZeroizingAllocator;
pub use client::{Client, ClientSettings, DeviceType};
