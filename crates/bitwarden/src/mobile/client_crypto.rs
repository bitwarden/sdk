use crate::Client;
#[cfg(feature = "internal")]
use crate::{
    error::Result,
    mobile::crypto::{
        derive_pin_key, get_user_encryption_key, initialize_org_crypto, initialize_user_crypto,
        DerivePinKeyResponse, InitOrgCryptoRequest, InitUserCryptoRequest,
    },
};

pub struct ClientCrypto<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientCrypto<'a> {
    #[cfg(feature = "internal")]
    pub async fn initialize_user_crypto(&mut self, req: InitUserCryptoRequest) -> Result<()> {
        initialize_user_crypto(self.client, req).await
    }

    #[cfg(feature = "internal")]
    pub async fn initialize_org_crypto(&mut self, req: InitOrgCryptoRequest) -> Result<()> {
        initialize_org_crypto(self.client, req).await
    }

    #[cfg(feature = "internal")]
    pub async fn get_user_encryption_key(&mut self) -> Result<String> {
        get_user_encryption_key(self.client).await
    }

    #[cfg(feature = "internal")]
    pub async fn derive_pin_key(&mut self, pin: String) -> Result<DerivePinKeyResponse> {
        derive_pin_key(self.client, pin)
    }
}

impl<'a> Client {
    pub fn crypto(&'a mut self) -> ClientCrypto<'a> {
        ClientCrypto { client: self }
    }
}
