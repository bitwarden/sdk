use bitwarden_crypto::{CryptoError, KeyDecryptable, KeyEncryptable};
use bitwarden_vault::{Folder, FolderView};

use crate::{error::Result, vault::ClientVault, Client};

pub struct ClientFolders<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientFolders<'a> {
    pub async fn encrypt(&self, folder_view: FolderView) -> Result<Folder> {
        let enc = self.client.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;

        let folder = folder_view.encrypt_with_key(key)?;

        Ok(folder)
    }

    pub async fn decrypt(&self, folder: Folder) -> Result<FolderView> {
        let enc = self.client.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;

        let folder_view = folder.decrypt_with_key(key)?;

        Ok(folder_view)
    }

    pub async fn decrypt_list(&self, folders: Vec<Folder>) -> Result<Vec<FolderView>> {
        let enc = self.client.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;

        let views = folders.decrypt_with_key(key)?;

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
