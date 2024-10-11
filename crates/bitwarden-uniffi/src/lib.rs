uniffi::setup_scaffolding!();

use std::sync::Arc;

use auth::ClientAuth;
use bitwarden_core::ClientSettings;

pub mod auth;
pub mod crypto;
mod error;
pub mod platform;
pub mod tool;
mod uniffi_support;
pub mod vault;

#[cfg(target_os = "android")]
mod android_support;

use crypto::ClientCrypto;
use error::Result;
use platform::ClientPlatform;
use tool::{ClientExporters, ClientGenerators, ClientSends};
use vault::ClientVault;

#[derive(uniffi::Object)]
pub struct Client(bitwarden_core::Client);

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    /// Initialize a new instance of the SDK client
    #[uniffi::constructor]
    pub fn new(settings: Option<ClientSettings>) -> Arc<Self> {
        init_logger();

        #[cfg(target_os = "android")]
        android_support::init();

        Arc::new(Self(bitwarden_core::Client::new(settings)))
    }

    /// Crypto operations
    pub fn crypto(self: Arc<Self>) -> Arc<ClientCrypto> {
        Arc::new(ClientCrypto(self))
    }

    /// Vault item operations
    pub fn vault(self: Arc<Self>) -> Arc<ClientVault> {
        Arc::new(ClientVault(self))
    }

    pub fn platform(self: Arc<Self>) -> Arc<ClientPlatform> {
        Arc::new(ClientPlatform(self))
    }

    /// Generator operations
    pub fn generators(self: Arc<Self>) -> Arc<ClientGenerators> {
        Arc::new(ClientGenerators(self))
    }

    /// Exporters
    pub fn exporters(self: Arc<Self>) -> Arc<ClientExporters> {
        Arc::new(ClientExporters(self))
    }

    /// Sends operations
    pub fn sends(self: Arc<Self>) -> Arc<ClientSends> {
        Arc::new(ClientSends(self))
    }

    /// Auth operations
    pub fn auth(self: Arc<Self>) -> Arc<ClientAuth> {
        Arc::new(ClientAuth(self))
    }

    /// Test method, echoes back the input
    pub fn echo(&self, msg: String) -> String {
        msg
    }

    /// Test method, calls http endpoint
    pub async fn http_get(&self, url: String) -> Result<String> {
        let client = self.0.internal.get_http_client();
        let res = client
            .get(&url)
            .send()
            .await
            .map_err(bitwarden_core::Error::Reqwest)?;

        Ok(res.text().await.map_err(bitwarden_core::Error::Reqwest)?)
    }
}

fn init_logger() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .try_init();

    #[cfg(target_os = "ios")]
    let _ = oslog::OsLogger::new("com.8bit.bitwarden")
        .level_filter(log::LevelFilter::Info)
        .init();

    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default().with_max_level(uniffi::deps::log::LevelFilter::Info),
    );
}
