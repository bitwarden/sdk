mod fido2;
mod user_interface;

pub(crate) use fido2::{client_create_credential, client_get_assertion};
pub use fido2::{Fido2ClientCreateCredentialRequest, Fido2ClientGetAssertionRequest};
pub use user_interface::{
    Fido2GetAssertionUserInterface, Fido2MakeCredentialUserInterface, NewCredentialParams,
    NewCredentialResult, VaultItem,
};
