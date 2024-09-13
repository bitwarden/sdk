mod client_generator;
mod username_forwarders;
pub use client_generator::{ClientGenerator, ClientGeneratorExt};
pub(crate) mod passphrase;
pub use passphrase::{PassphraseError, PassphraseGeneratorRequest};
pub(crate) mod password;
pub use password::{PasswordError, PasswordGeneratorRequest};
pub(crate) mod username;
pub use username::{ForwarderServiceType, UsernameError, UsernameGeneratorRequest};
mod util;

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
