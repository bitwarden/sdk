pub mod folders;

use std::rc::Rc;

use bitwarden_core::Client;
use wasm_bindgen::prelude::*;

use crate::ClientFolders;

#[wasm_bindgen]
pub struct ClientVault(Rc<Client>);

impl ClientVault {
    pub fn new(client: Rc<Client>) -> Self {
        Self(client)
    }
}

#[wasm_bindgen]
impl ClientVault {
    pub fn folders(&self) -> ClientFolders {
        ClientFolders::new(self.0.clone())
    }
}
