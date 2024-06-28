use bitwarden_core::{Client, Error};
use bitwarden_crypto::{CryptoError, KeyDecryptable, KeyEncryptable};

use crate::{ClientVault, Folder, FolderView};

pub struct ClientFolders<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientFolders<'a> {
    pub fn encrypt(&self, folder_view: FolderView) -> Result<Folder, Error> {
        let enc = self.client.internal.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;

        let folder = folder_view.encrypt_with_key(key)?;

        Ok(folder)
    }

    pub fn decrypt(&self, folder: Folder) -> Result<FolderView, Error> {
        let enc = self.client.internal.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;

        let folder_view = folder.decrypt_with_key(key)?;

        Ok(folder_view)
    }

    pub fn decrypt_list(&self, folders: Vec<Folder>) -> Result<Vec<FolderView>, Error> {
        let enc = self.client.internal.get_encryption_settings()?;
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
