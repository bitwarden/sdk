mod passphrase;
pub use passphrase::{passphrase, PassphraseGeneratorRequest};
mod error;
mod util;
pub use error::GeneratorError;
mod password;
pub use password::{password, PasswordGeneratorRequest};
mod username;
pub use username::{username, UsernameGeneratorRequest};
mod username_forwarders;

#[cfg(feature = "mobile")]
uniffi::setup_scaffolding!();
