pub mod client_create_credential;
mod fido2;
mod user_interface;

pub use client_create_credential::Fido2ClientCreateCredentialRequest;
pub(crate) use fido2::client_get_assertion;

pub use fido2::Fido2ClientGetAssertionRequest;
pub use user_interface::{
    Fido2GetAssertionUserInterface, Fido2MakeCredentialUserInterface, NewCredentialParams,
    NewCredentialResult, VaultItem,
};
