use std::sync::Arc;

use bitwarden_fido::{
    Fido2Authenticator, Fido2Client, Fido2CredentialStore, Fido2UserInterface,
    FidoEncryptionSettingStore,
};

use crate::Client;

pub struct ClientFido2<'a> {
    #[allow(dead_code)]
    pub(crate) client: &'a Client,
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
}
