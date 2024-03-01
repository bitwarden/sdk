pub mod client;
pub mod command;
pub mod response;

pub use bitwarden::{
    error::Result,
    platform::{Fido2ClientGetAssertionRequest, Fido2GetAssertionUserInterface, VaultItem},
};
