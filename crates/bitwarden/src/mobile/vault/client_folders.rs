use bitwarden_crypto::{Decryptable, Encryptable};

use super::client_vault::ClientVault;
use crate::{
    error::Result,
    vault::{Folder, FolderView},
    Client,
};

pub struct ClientFolders<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientFolders<'a> {
    pub async fn encrypt(&self, folder_view: FolderView) -> Result<Folder> {
        let enc = self.client.get_encryption_settings()?;

        let folder = folder_view.encrypt(enc, &None)?;

        Ok(folder)
    }

    pub async fn decrypt(&self, folder: Folder) -> Result<FolderView> {
        let enc = self.client.get_encryption_settings()?;

        let folder_view = folder.decrypt(enc, &None)?;

        Ok(folder_view)
    }

    pub async fn decrypt_list(&self, folders: Vec<Folder>) -> Result<Vec<FolderView>> {
        let enc = self.client.get_encryption_settings()?;

        let views = folders.decrypt(enc, &None)?;

        Ok(views)
    }
}

impl<'a> ClientVault<'a> {
    pub fn folders(&'a self) -> ClientFolders<'a> {
        ClientFolders {
            client: self.client,
        }
    }
}
