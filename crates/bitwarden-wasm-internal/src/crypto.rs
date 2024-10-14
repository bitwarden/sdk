use std::rc::Rc;

use bitwarden_core::{
    mobile::crypto::{InitOrgCryptoRequest, InitUserCryptoRequest},
    Client,
};
use wasm_bindgen::prelude::*;

use crate::error::Result;

#[wasm_bindgen]
pub struct ClientCrypto(Rc<Client>);

impl ClientCrypto {
    pub fn new(client: Rc<Client>) -> Self {
        Self(client)
    }
}

#[wasm_bindgen]
impl ClientCrypto {
    /// Initialization method for the user crypto. Needs to be called before any other crypto
    /// operations.
    pub async fn initialize_user_crypto(&self, req: InitUserCryptoRequest) -> Result<()> {
        Ok(self.0.crypto().initialize_user_crypto(req).await?)
    }

    /// Initialization method for the organization crypto. Needs to be called after
    /// `initialize_user_crypto` but before any other crypto operations.
    pub async fn initialize_org_crypto(&self, req: InitOrgCryptoRequest) -> Result<()> {
        Ok(self.0.crypto().initialize_org_crypto(req).await?)
    }
}
