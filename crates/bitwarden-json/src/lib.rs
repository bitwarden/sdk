pub mod client;
pub mod command;
pub mod response;

pub use bitwarden::{
    error::Result,
    platform::fido2::{
        Fido2ClientCreateCredentialRequest, Fido2ClientGetAssertionRequest,
        Fido2GetAssertionUserInterface, Fido2MakeCredentialUserInterface, NewCredentialParams,
        NewCredentialResult, VaultItem,
    },
};
