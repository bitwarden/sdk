use std::rc::Rc;

use bitwarden::{
    mobile::crypto::{InitOrgCryptoRequest, InitUserCryptoRequest},
    Client,
};
use wasm_bindgen::prelude::*;

// use crate::{error::Result, Client};

#[wasm_bindgen]
pub struct ClientCrypto(pub(crate) Rc<Client>);

#[wasm_bindgen]
impl ClientCrypto {
    /// Initialization method for the user crypto. Needs to be called before any other crypto
    /// operations.
    pub async fn initialize_user_crypto(&self, req: InitUserCryptoRequest) {
        // Ok(
        self.0.crypto().initialize_user_crypto(req).await.unwrap()
        // );
        // .map_err(Error::EncryptionSettings)?)
    }

    /// Initialization method for the organization crypto. Needs to be called after
    /// `initialize_user_crypto` but before any other crypto operations.
    pub async fn initialize_org_crypto(&self, req: InitOrgCryptoRequest) {
        // Ok(
        self.0.crypto().initialize_org_crypto(req).await.unwrap()
        // );
        // .map_err(Error::EncryptionSettings)?)
    }
}
