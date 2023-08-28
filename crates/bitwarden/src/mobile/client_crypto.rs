#[cfg(feature = "internal")]
use super::crypto::{initialize_crypto, InitCryptoRequest};
#[cfg(feature = "internal")]
use crate::error::Result;
use crate::Client;

pub struct ClientCrypto<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientCrypto<'a> {
    #[cfg(feature = "internal")]
    pub async fn initialize_crypto(&mut self, req: InitCryptoRequest) -> Result<()> {
        initialize_crypto(self.client, req).await
    }
}

impl<'a> Client {
    pub fn crypto(&'a mut self) -> ClientCrypto<'a> {
        ClientCrypto { client: self }
    }
}
