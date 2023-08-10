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
    /// Folder operations
    pub fn folders(self: Arc<Self>) -> Arc<ClientFolders> {
        Arc::new(ClientFolders(self.0.clone()))
    }
}

#[uniffi::export]
impl Client {
    /// Vault operations
    pub fn vault(self: Arc<Self>) -> Arc<ClientVault> {
        Arc::new(ClientVault(self))
    }
}

#[uniffi::export]
impl ClientFolders {
    /// Encrypt folder
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

    /// Decrypt folder
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

    /// Decrypt folder list
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
