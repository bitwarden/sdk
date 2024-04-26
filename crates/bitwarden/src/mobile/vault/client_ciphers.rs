use bitwarden_crypto::{CryptoError, KeyDecryptable, KeyEncryptable, LocateKey};
use uuid::Uuid;

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

        // TODO: Once this flag is removed, the key generation logic should
        // be moved directly into the KeyEncryptable implementation
        if cipher_view.key.is_none() && self.client.get_flags().enable_cipher_key_encryption {
            let key = cipher_view
                .locate_key(enc, &None)
                .ok_or(Error::VaultLocked)?;
            cipher_view.generate_cipher_key(key)?;
        }

        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;
        let cipher = cipher_view.encrypt_with_key(key)?;

        Ok(cipher)
    }

    pub async fn decrypt(&self, cipher: Cipher) -> Result<CipherView> {
        let enc = self.client.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;

        let cipher_view = cipher.decrypt_with_key(key)?;

        Ok(cipher_view)
    }

    pub async fn decrypt_list(&self, ciphers: Vec<Cipher>) -> Result<Vec<CipherListView>> {
        let enc = self.client.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;

        let cipher_views = ciphers.decrypt_with_key(key)?;

        Ok(cipher_views)
    }

    pub async fn move_to_organization(
        &self,
        mut cipher_view: CipherView,
        organization_id: Uuid,
    ) -> Result<CipherView> {
        let enc = self.client.get_encryption_settings()?;
        cipher_view.move_to_organization(enc, organization_id)?;
        Ok(cipher_view)
    }
}

impl<'a> ClientVault<'a> {
    pub fn ciphers(&'a self) -> ClientCiphers<'a> {
        ClientCiphers {
            client: self.client,
        }
    }
}
