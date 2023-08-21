use std::sync::Arc;

use bitwarden::vault::{Cipher, CipherListView, CipherView};

use crate::{Client, Result};

#[derive(uniffi::Object)]
pub struct ClientCiphers(pub Arc<Client>);

#[uniffi::export]
impl ClientCiphers {
    /// Encrypt cipher
    pub async fn encrypt(&self, cipher_view: CipherView) -> Result<Cipher> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .ciphers()
            .encrypt(cipher_view)
            .await?)
    }

    /// Decrypt cipher
    pub async fn decrypt(&self, cipher: Cipher) -> Result<CipherView> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .ciphers()
            .decrypt(cipher)
            .await?)
    }

    /// Decrypt cipher list
    pub async fn decrypt_list(&self, ciphers: Vec<Cipher>) -> Result<Vec<CipherListView>> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .ciphers()
            .decrypt_list(ciphers)
            .await?)
    }
}
