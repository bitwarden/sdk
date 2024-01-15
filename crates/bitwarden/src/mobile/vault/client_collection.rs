use super::client_vault::ClientVault;
use crate::{
    crypto::Decryptable,
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

        let view = collection.decrypt(enc)?;

        Ok(view)
    }

    pub async fn decrypt_list(&self, collections: Vec<Collection>) -> Result<Vec<CollectionView>> {
        let enc = self.client.get_encryption_settings()?;

        let views: Result<_> = collections.into_iter().map(|c| c.decrypt(enc)).collect();

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
