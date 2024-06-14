use std::sync::Arc;

use bitwarden::vault::{Cipher, CipherListView, CipherView, ClientVaultExt};
use uuid::Uuid;

use crate::{error::BitwardenError, Client, Result};

#[derive(uniffi::Object)]
pub struct ClientCiphers(pub Arc<Client>);

#[uniffi::export]
impl ClientCiphers {
    /// Encrypt cipher
    pub fn encrypt(&self, cipher_view: CipherView) -> Result<Cipher> {
        Ok(self.0 .0.vault().ciphers().encrypt(cipher_view)?)
    }

    /// Decrypt cipher
    pub fn decrypt(&self, cipher: Cipher) -> Result<CipherView> {
        Ok(self.0 .0.vault().ciphers().decrypt(cipher)?)
    }

    /// Decrypt cipher list
    pub fn decrypt_list(&self, ciphers: Vec<Cipher>) -> Result<Vec<CipherListView>> {
        Ok(self.0 .0.vault().ciphers().decrypt_list(ciphers)?)
    }

    pub fn decrypt_fido2_credentials(
        &self,
        cipher_view: CipherView,
    ) -> Result<Vec<Fido2CredentialView>> {
        Ok(self
            .0
             .0
            .vault()
            .ciphers()
            .decrypt_fido2_credentials(cipher_view)?)
    }

    /// Move a cipher to an organization, reencrypting the cipher key if necessary
    pub fn move_to_organization(
        &self,
        cipher: CipherView,
        organization_id: Uuid,
    ) -> Result<CipherView> {
        Ok(self
            .0
             .0
            .vault()
            .ciphers()
            .move_to_organization(cipher, organization_id)
            .await
            .map_err(|e| BitwardenError::E2(bitwarden::error::Error::Cipher(e)))?)
    }
}
