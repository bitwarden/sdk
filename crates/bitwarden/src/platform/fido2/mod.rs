use crate::{
    error::Result,
    vault::{login::Fido2Credential, CipherView},
    Client,
};

mod authenticator;
mod client;
mod traits;

pub use authenticator::{
    Fido2Authenticator, GetAssertionRequest, GetAssertionResult, MakeCredentialRequest,
    MakeCredentialResult,
};
pub use client::{
    AuthenticatorAssertionResponse, AuthenticatorAttestationResponse, ClientData, Fido2Client,
    PublicKeyCredentialAuthenticatorAssertionResponse,
    PublicKeyCredentialAuthenticatorAttestationResponse,
};
use passkey::types::Passkey;
pub use traits::{CheckUserOptions, CheckUserResult, CredentialStore, UserInterface, Verification};

pub struct ClientFido2<'a> {
    #[allow(dead_code)]
    pub(crate) client: &'a mut Client,
}

impl<'a> ClientFido2<'a> {
    pub fn create_authenticator(
        &'a mut self,

        user_interface: &'a dyn UserInterface,
        credential_store: &'a dyn CredentialStore,
    ) -> Result<Fido2Authenticator<'a>> {
        Ok(Fido2Authenticator {
            client: self.client,
            user_interface,
            credential_store,
        })
    }

    pub fn create_client(
        &'a mut self,

        user_interface: &'a dyn UserInterface,
        credential_store: &'a dyn CredentialStore,
    ) -> Result<Fido2Client<'a>> {
        Ok(Fido2Client {
            authenticator: self.create_authenticator(user_interface, credential_store)?,
        })
    }
}

#[allow(dead_code)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SelectedCredential {
    cipher: CipherView,
    credential: Fido2Credential,
}

impl TryFrom<CipherView> for Passkey {
    type Error = crate::error::Error;

    fn try_from(value: CipherView) -> std::prelude::v1::Result<Self, Self::Error> {
        let _creds = value
            .login
            .and_then(|l| l.fido2_credentials)
            .ok_or("No Fido2Credential")?;

        todo!("We have more than one credential, we need to pick one?")
    }
}
