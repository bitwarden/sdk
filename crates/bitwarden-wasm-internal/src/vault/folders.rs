use std::rc::Rc;

use bitwarden_core::Client;
use bitwarden_vault::{ClientVaultExt, Folder, FolderView};
use wasm_bindgen::prelude::*;

use crate::error::Result;

#[wasm_bindgen]
pub struct ClientFolders(pub(crate) Rc<Client>);

#[wasm_bindgen]
impl ClientFolders {
    /// Decrypt folder
    pub fn decrypt(&self, folder: Folder) -> Result<FolderView> {
        Ok(self.0.vault().folders().decrypt(folder)?)
    }
}
