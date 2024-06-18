use std::sync::Arc;

use bitwarden_fido::{
    Fido2Authenticator, Fido2Client, Fido2CredentialAutofillView, Fido2CredentialStore,
    Fido2UserInterface, FidoEncryptionSettingStore,
};
use bitwarden_vault::CipherView;
use thiserror::Error;

use crate::Client;

pub struct ClientFido2<'a> {
    #[allow(dead_code)]
    pub(crate) client: &'a Client,
}

#[derive(Debug, Error)]
pub enum DecryptFido2AutofillCredentialsError {
    #[error(transparent)]
    VaultLocked(#[from] bitwarden_core::VaultLocked),
    #[error(transparent)]
    Fido2CredentialAutofillViewError(#[from] bitwarden_fido::Fido2CredentialAutofillViewError),
}

impl FidoEncryptionSettingStore for Client {
    fn get_encryption_settings(
        &self,
    ) -> Result<Arc<dyn bitwarden_crypto::KeyContainer>, bitwarden_core::VaultLocked> {
        Ok(self.get_encryption_settings()?)
    }
}

impl<'a> ClientFido2<'a> {
    pub fn create_authenticator(
        &'a self,
        user_interface: &'a dyn Fido2UserInterface,
        credential_store: &'a dyn Fido2CredentialStore,
    ) -> Fido2Authenticator<'a> {
        Fido2Authenticator::new(self.client, user_interface, credential_store)
    }

    pub fn create_client(
        &'a self,
        user_interface: &'a dyn Fido2UserInterface,
        credential_store: &'a dyn Fido2CredentialStore,
    ) -> Fido2Client<'a> {
        Fido2Client {
            authenticator: self.create_authenticator(user_interface, credential_store),
        }
    }

    pub fn decrypt_fido2_autofill_credentials(
        &'a self,
        cipher_view: CipherView,
    ) -> Result<Vec<Fido2CredentialAutofillView>, DecryptFido2AutofillCredentialsError> {
        let enc = self.client.get_encryption_settings()?;

        Ok(Fido2CredentialAutofillView::from_cipher_view(
            &cipher_view,
            &*enc,
        )?)
    }
}
