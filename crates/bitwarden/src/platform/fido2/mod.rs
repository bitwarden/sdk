pub mod client_create_credential;
mod credential_store;
mod crypto;
mod fido2;
mod guid;
mod transaction;
mod user_interface;
mod vault_item;

pub use client_create_credential::{
    Fido2ClientCreateCredentialRequest, Fido2CreatedPublicKeyCredential,
};

pub use credential_store::{Fido2CredentialStore, FindCredentialsParams, SaveCredentialParams};
pub use fido2::Fido2ClientGetAssertionRequest;
pub use passkey::types::{
    ctap2::make_credential::{PublicKeyCredentialRpEntity, PublicKeyCredentialUserEntity},
    webauthn::{PublicKeyCredentialCreationOptions, PublicKeyCredentialDescriptor},
};
pub use user_interface::{
    Fido2UserInterface, NewCredentialParams, NewCredentialResult, PickCredentialParams,
    PickCredentialResult,
};
pub use vault_item::{Fido2CredentialView, Fido2VaultItem};
