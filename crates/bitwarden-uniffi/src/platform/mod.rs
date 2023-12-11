use std::sync::Arc;

use bitwarden::platform::FingerprintRequest;

use crate::{error::Result, Client};

#[derive(uniffi::Object)]
pub struct ClientPlatform(pub(crate) Arc<Client>);

#[uniffi::export(async_runtime = "tokio")]
impl ClientPlatform {
    /// Fingerprint
    pub async fn fingerprint(&self, req: FingerprintRequest) -> Result<String> {
        Ok(self.0 .0.read().await.fingerprint(&req)?.fingerprint)
    }
}
