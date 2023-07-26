use std::sync::Arc;

use bitwarden::mobile::vault::{
    FolderDecryptListRequest, FolderDecryptListResponse, FolderDecryptRequest,
    FolderDecryptResponse, FolderEncryptRequest, FolderEncryptResponse,
};

use crate::{Client, Result};

#[derive(uniffi::Object)]
pub struct ClientVault(Arc<Client>);

#[derive(uniffi::Object)]
pub struct ClientFolders(Arc<Client>);

#[uniffi::export]
impl ClientVault {
    pub fn folders(self: Arc<Self>) -> Arc<ClientFolders> {
        Arc::new(ClientFolders(self.0.clone()))
    }
}

#[uniffi::export]
impl Client {
    pub fn vault(self: Arc<Self>) -> Arc<ClientVault> {
        Arc::new(ClientVault(self))
    }
}

#[uniffi::export]
impl ClientFolders {
    pub async fn encrypt(&self, req: FolderEncryptRequest) -> Result<FolderEncryptResponse> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .folders()
            .encrypt(req)
            .await?)
    }

    pub async fn decrypt(&self, req: FolderDecryptRequest) -> Result<FolderDecryptResponse> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .folders()
            .decrypt(req)
            .await?)
    }

    pub async fn decrypt_list(
        &self,
        req: FolderDecryptListRequest,
    ) -> Result<FolderDecryptListResponse> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .folders()
            .decrypt_list(req)
            .await?)
    }
}
