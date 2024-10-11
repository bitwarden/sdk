use std::rc::Rc;

use bitwarden::{
    mobile::crypto::{
        InitOrgCryptoRequest, InitUserCryptoRequest, MakeKeyPairResponse,
        VerifyAsymmetricKeysRequest, VerifyAsymmetricKeysResponse,
    },
    Client,
};
use wasm_bindgen::prelude::*;

use crate::error::Result;

#[wasm_bindgen]
pub struct ClientCrypto(pub(crate) Rc<Client>);

#[wasm_bindgen]
impl ClientCrypto {
    /// Initialization method for the user crypto. Needs to be called before any other crypto
    /// operations.
    pub async fn initialize_user_crypto(&self, req: InitUserCryptoRequest) -> Result<()> {
        Ok(self
            .0
            .crypto()
            .initialize_user_crypto(req)
            .await
            .map_err(bitwarden_core::Error::EncryptionSettings)?)
    }

    /// Initialization method for the organization crypto. Needs to be called after
    /// `initialize_user_crypto` but before any other crypto operations.
    pub async fn initialize_org_crypto(&self, req: InitOrgCryptoRequest) -> Result<()> {
        Ok(self
            .0
            .crypto()
            .initialize_org_crypto(req)
            .await
            .map_err(bitwarden_core::Error::EncryptionSettings)?)
    }

    pub fn make_key_pair(&self) -> Result<MakeKeyPairResponse> {
        Ok(self.0.crypto().make_key_pair()?)
    }

    pub fn verify_asymmetric_keys(
        &self,
        request: VerifyAsymmetricKeysRequest,
    ) -> Result<VerifyAsymmetricKeysResponse> {
        Ok(self.0.crypto().verify_asymmetric_keys(request)?)
    }
}
