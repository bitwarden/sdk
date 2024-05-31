use bitwarden_crypto::{CryptoError, KeyDecryptable, LocateKey};

use crate::{
    error::Result,
    vault::{ClientVault, Collection, CollectionView},
    Client,
};

pub struct ClientCollections<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientCollections<'a> {
    pub async fn decrypt(&self, collection: Collection) -> Result<CollectionView> {
        let enc = self.client.get_encryption_settings()?;
        let key = collection
            .locate_key(enc, &None)
            .ok_or(CryptoError::MissingKey)?;

        let view = collection.decrypt_with_key(key)?;

        Ok(view)
    }

    pub async fn decrypt_list(&self, collections: Vec<Collection>) -> Result<Vec<CollectionView>> {
        let enc = self.client.get_encryption_settings()?;

        let views: Result<Vec<CollectionView>> = collections
            .iter()
            .map(|c| -> Result<CollectionView> {
                let key = c.locate_key(enc, &None).ok_or(CryptoError::MissingKey)?;
                Ok(c.decrypt_with_key(key)?)
            })
            .collect();

        views
    }
}

impl<'a> ClientVault<'a> {
    pub fn collections(&'a self) -> ClientCollections<'a> {
        ClientCollections {
            client: self.client,
        }
    }
}
