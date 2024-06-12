use std::sync::Arc;

use bitwarden::vault::{Folder, FolderView};

use crate::{Client, Result};

#[derive(uniffi::Object)]
pub struct ClientFolders(pub Arc<Client>);

#[uniffi::export(async_runtime = "tokio")]
impl ClientFolders {
    /// Encrypt folder
    pub async fn encrypt(&self, folder: FolderView) -> Result<Folder> {
        Ok(self.0 .0.vault().folders().encrypt(folder).await?)
    }

    /// Decrypt folder
    pub async fn decrypt(&self, folder: Folder) -> Result<FolderView> {
        Ok(self.0 .0.vault().folders().decrypt(folder).await?)
    }

    /// Decrypt folder list
    pub async fn decrypt_list(&self, folders: Vec<Folder>) -> Result<Vec<FolderView>> {
        Ok(self.0 .0.vault().folders().decrypt_list(folders).await?)
    }
}
