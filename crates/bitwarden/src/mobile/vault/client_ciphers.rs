use bitwarden_crypto::{Decryptable, Encryptable, LocateKey};

use super::client_vault::ClientVault;
use crate::{
    error::{Error, Result},
    vault::{Cipher, CipherListView, CipherView},
    Client,
};

pub struct ClientCiphers<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientCiphers<'a> {
    pub async fn encrypt(&self, mut cipher_view: CipherView) -> Result<Cipher> {
        let enc = self.client.get_encryption_settings()?;

        if cipher_view.key.is_none() && self.client.get_flags().enable_cipher_key_encryption {
            let key = cipher_view
                .locate_key(enc, &None)
                .ok_or(Error::VaultLocked)?;
            cipher_view.generate_cipher_key(key)?;
        }

        let cipher = cipher_view.encrypt(enc, &None)?;

        Ok(cipher)
    }

    pub async fn decrypt(&self, cipher: Cipher) -> Result<CipherView> {
        let enc = self.client.get_encryption_settings()?;

        let cipher_view = cipher.decrypt(enc, &None)?;

        Ok(cipher_view)
    }

    pub async fn decrypt_list(&self, ciphers: Vec<Cipher>) -> Result<Vec<CipherListView>> {
        let enc = self.client.get_encryption_settings()?;

        let cipher_views = ciphers.decrypt(enc, &None)?;

        Ok(cipher_views)
    }
}

impl<'a> ClientVault<'a> {
    pub fn ciphers(&'a self) -> ClientCiphers<'a> {
        ClientCiphers {
            client: self.client,
        }
    }
}
