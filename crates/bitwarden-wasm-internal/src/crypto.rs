use std::rc::Rc;

use bitwarden_core::{
    mobile::crypto::{
        InitOrgCryptoRequest, InitUserCryptoRequest, MakeKeyPairResponse,
        VerifyAsymmetricKeysRequest, VerifyAsymmetricKeysResponse,
    },
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

    /// Generates a new key pair and encrypts the private key with the initialized user key.
    /// Needs to be called after `initialize_user_crypto`.
    pub fn make_key_pair(&self) -> Result<MakeKeyPairResponse> {
        Ok(self.0.crypto().make_key_pair()?)
    }

    /// Verifies a user's asymmetric keys by decrypting the private key with the initialized user
    /// key. Returns if the private key is decryptable and if it is a valid matching key.
    /// Needs to be called after `initialize_user_crypto`.
    pub fn verify_asymmetric_keys(
        &self,
        request: VerifyAsymmetricKeysRequest,
    ) -> Result<VerifyAsymmetricKeysResponse> {
        Ok(self.0.crypto().verify_asymmetric_keys(request)?)
    }
}
