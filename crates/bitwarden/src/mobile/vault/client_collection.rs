use bitwarden_crypto::Decryptable;

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

        let view = collection.decrypt(enc, &None)?;

        Ok(view)
    }

    pub async fn decrypt_list(&self, collections: Vec<Collection>) -> Result<Vec<CollectionView>> {
        let enc = self.client.get_encryption_settings()?;

        let views = collections.decrypt(enc, &None)?;

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
