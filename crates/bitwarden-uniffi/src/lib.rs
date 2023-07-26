uniffi::setup_scaffolding!();

use std::sync::Arc;

use async_lock::RwLock;
use bitwarden::{
    client::client_settings::ClientSettings,
    mobile::{crypto::InitCryptoRequest, kdf::PasswordHashRequest},
};

mod error;
mod vault;

use error::Result;

#[derive(uniffi::Object)]
pub struct Client(RwLock<bitwarden::Client>);

#[derive(uniffi::Object)]
pub struct ClientKdf(Arc<Client>);

#[derive(uniffi::Object)]
pub struct ClientCrypto(Arc<Client>);

#[uniffi::export]
impl Client {
    #[uniffi::constructor]
    pub fn new(settings: Option<ClientSettings>) -> Arc<Self> {
        Arc::new(Self(RwLock::new(bitwarden::Client::new(settings))))
    }

    pub fn kdf(self: Arc<Self>) -> Arc<ClientKdf> {
        Arc::new(ClientKdf(self))
    }
    pub fn crypto(self: Arc<Self>) -> Arc<ClientCrypto> {
        Arc::new(ClientCrypto(self))
    }

    pub fn echo(&self, msg: String) -> String {
        msg
    }
}

#[uniffi::export]
impl ClientKdf {
    pub async fn hash_password(&self, req: PasswordHashRequest) -> Result<String> {
        Ok(self.0 .0.read().await.kdf().hash_password(req).await?)
    }
}
#[uniffi::export]
impl ClientCrypto {
    pub async fn initialize_crypto(&self, req: InitCryptoRequest) -> Result<()> {
        Ok(self
            .0
             .0
            .write()
            .await
            .crypto()
            .initialize_crypto(req)
            .await?)
    }
}
