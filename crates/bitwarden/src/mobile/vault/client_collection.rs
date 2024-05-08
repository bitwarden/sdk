use bitwarden_crypto::{CryptoError, KeyDecryptable};

use super::client_vault::ClientVault;
use crate::{
    error::Result,
    vault::{Collection, CollectionView},
    Client,
};

pub struct ClientCollections<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientCollections<'a> {
    pub async fn decrypt(&self, collection: Collection) -> Result<CollectionView> {
        let enc = self.client.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;

        let view = collection.decrypt_with_key(key)?;

        Ok(view)
    }

    pub async fn decrypt_list(&self, collections: Vec<Collection>) -> Result<Vec<CollectionView>> {
        let enc = self.client.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;

        let views = collections.decrypt_with_key(key)?;

        Ok(views)
    }
}

impl<'a> ClientVault<'a> {
    pub fn collections(&'a self) -> ClientCollections<'a> {
        ClientCollections {
            client: self.client,
        }
    }
}
