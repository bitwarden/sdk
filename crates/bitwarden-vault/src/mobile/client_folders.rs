use bitwarden_core::{Client, Error};

use crate::{ClientVault, Folder, FolderView};

pub struct ClientFolders<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientFolders<'a> {
    pub fn encrypt(&self, folder_view: FolderView) -> Result<Folder, Error> {
        let crypto = self.client.internal.get_crypto_service();

        let folder = crypto.encrypt(folder_view)?;

        Ok(folder)
    }

    pub fn decrypt(&self, folder: Folder) -> Result<FolderView, Error> {
        let crypto = self.client.internal.get_crypto_service();

        let folder_view = crypto.decrypt(&folder)?;

        Ok(folder_view)
    }

    pub fn decrypt_list(&self, folders: Vec<Folder>) -> Result<Vec<FolderView>, Error> {
        let crypto = self.client.internal.get_crypto_service();

        let views = crypto.decrypt_list(&folders)?;

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
