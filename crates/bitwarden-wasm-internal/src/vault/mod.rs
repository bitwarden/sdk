pub mod folders;

use std::rc::Rc;

use bitwarden::Client;
use wasm_bindgen::prelude::*;

use crate::ClientFolders;

#[wasm_bindgen]
pub struct ClientVault(pub(crate) Rc<Client>);

#[wasm_bindgen]
impl ClientVault {
    pub fn folders(&self) -> ClientFolders {
        ClientFolders(self.0.clone())
    }
}
