use crate::{crypto::Decryptable, error::Result, Client};

use super::{
    client_vault::ClientVault,
    collections::{
        CollectionDecryptListRequest, CollectionDecryptListResponse, CollectionDecryptRequest,
        CollectionDecryptResponse,
    },
};

pub struct ClientCollections<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientCollections<'a> {
    pub async fn decrypt(
        &self,
        req: CollectionDecryptRequest,
    ) -> Result<CollectionDecryptResponse> {
        let enc = self.client.get_encryption_settings()?;

        let collection = req.collection.decrypt(enc, &None)?;

        Ok(CollectionDecryptResponse { collection })
    }

    pub async fn decrypt_list(
        &self,
        req: CollectionDecryptListRequest,
    ) -> Result<CollectionDecryptListResponse> {
        let enc = self.client.get_encryption_settings()?;

        let collections = req.collections.decrypt(enc, &None)?;

        Ok(CollectionDecryptListResponse { collections })
    }
}

impl<'a> ClientVault<'a> {
    pub fn collections(&'a self) -> ClientCollections<'a> {
        ClientCollections {
            client: self.client,
        }
    }
}
