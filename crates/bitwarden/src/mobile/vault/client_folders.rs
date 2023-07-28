use crate::{
    crypto::{Decryptable, Encryptable},
    error::{Error, Result},
    Client,
};

use super::{
    client_vault::ClientVault, FolderDecryptListRequest, FolderDecryptListResponse,
    FolderDecryptRequest, FolderDecryptResponse, FolderEncryptRequest, FolderEncryptResponse,
};

pub struct ClientFolders<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientFolders<'a> {
    pub async fn encrypt(&self, req: FolderEncryptRequest) -> Result<FolderEncryptResponse> {
        let enc = self
            .client
            .get_encryption_settings()
            .as_ref()
            .ok_or(Error::VaultLocked)?;

        let folder = req.folder.encrypt(enc, &None)?;

        Ok(FolderEncryptResponse { folder })
    }

    pub async fn decrypt(&self, req: FolderDecryptRequest) -> Result<FolderDecryptResponse> {
        let enc = self
            .client
            .get_encryption_settings()
            .as_ref()
            .ok_or(Error::VaultLocked)?;

        let folder = req.folder.decrypt(enc, &None)?;

        Ok(FolderDecryptResponse { folder })
    }

    pub async fn decrypt_list(
        &self,
        req: FolderDecryptListRequest,
    ) -> Result<FolderDecryptListResponse> {
        let enc = self
            .client
            .get_encryption_settings()
            .as_ref()
            .ok_or(Error::VaultLocked)?;

        let folders = req.folders.decrypt(enc, &None)?;

        Ok(FolderDecryptListResponse { folders })
    }
}

impl<'a> ClientVault<'a> {
    pub fn folders(&'a self) -> ClientFolders<'a> {
        ClientFolders {
            client: self.client,
        }
    }
}
