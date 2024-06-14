use std::sync::Arc;

use bitwarden::vault::{Cipher, CipherListView, CipherView};
use uuid::Uuid;

use crate::{Client, Result};

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

    /// Move a cipher to an organization, reencrypting the cipher key if necessary
    ///
    /// Warning!
    ///
    /// This function is potentially dangerous when used on legacy attachments. It does not validate
    /// that attachments can actually be decrypted with the new key. Care MUST be taken to validate
    /// attachments before calling this function and to migrate them yourself.
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
            .move_to_organization(cipher, organization_id)?)
    }
}
