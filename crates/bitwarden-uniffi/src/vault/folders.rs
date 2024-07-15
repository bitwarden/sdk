use std::sync::Arc;

use bitwarden::vault::{ClientVaultExt, Folder, FolderView};

use crate::{Client, Result};

#[derive(uniffi::Object)]
pub struct ClientFolders(pub Arc<Client>);

#[uniffi::export]
impl ClientFolders {
    /// Encrypt folder
    pub fn encrypt(&self, folder: FolderView) -> Result<Folder> {
        Ok(self.0 .0.vault().folders().encrypt(folder)?)
    }

    /// Decrypt folder
    pub fn decrypt(&self, folder: Folder) -> Result<FolderView> {
        Ok(self.0 .0.vault().folders().decrypt(folder)?)
    }

    /// Decrypt folder list
    pub fn decrypt_list(&self, folders: Vec<Folder>) -> Result<Vec<FolderView>> {
        Ok(self.0 .0.vault().folders().decrypt_list(folders)?)
    }
}
