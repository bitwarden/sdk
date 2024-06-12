use std::sync::Arc;

use bitwarden::vault::{Collection, CollectionView};

use crate::{Client, Result};

#[derive(uniffi::Object)]
pub struct ClientCollections(pub Arc<Client>);

#[uniffi::export(async_runtime = "tokio")]
impl ClientCollections {
    /// Decrypt collection
    pub async fn decrypt(&self, collection: Collection) -> Result<CollectionView> {
        Ok(self.0 .0.vault().collections().decrypt(collection).await?)
    }

    /// Decrypt collection list
    pub async fn decrypt_list(&self, collections: Vec<Collection>) -> Result<Vec<CollectionView>> {
        Ok(self
            .0
             .0
            .vault()
            .collections()
            .decrypt_list(collections)
            .await?)
    }
}
