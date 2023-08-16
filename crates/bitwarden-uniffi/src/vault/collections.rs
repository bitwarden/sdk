use std::sync::Arc;

use bitwarden::mobile::vault::{
    CollectionDecryptListRequest, CollectionDecryptListResponse, CollectionDecryptRequest,
    CollectionDecryptResponse,
};

use crate::{Client, Result};

#[derive(uniffi::Object)]
pub struct ClientCollections(pub Arc<Client>);

#[uniffi::export]
impl ClientCollections {
    /// Decrypt collection
    pub async fn decrypt(
        &self,
        req: CollectionDecryptRequest,
    ) -> Result<CollectionDecryptResponse> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .collections()
            .decrypt(req)
            .await?)
    }

    /// Decrypt collection list
    pub async fn decrypt_list(
        &self,
        req: CollectionDecryptListRequest,
    ) -> Result<CollectionDecryptListResponse> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .collections()
            .decrypt_list(req)
            .await?)
    }
}
