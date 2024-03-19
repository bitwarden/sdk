pub mod client;
pub mod command;
pub mod response;

pub use bitwarden::{
    error::Result,
    platform::fido2::{
        Fido2ClientCreateCredentialRequest, Fido2ClientGetAssertionRequest, Fido2CredentialStore,
        Fido2UserInterface, NewCredentialParams, NewCredentialResult, VaultItem,
    },
};
