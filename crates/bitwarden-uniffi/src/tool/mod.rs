use std::sync::Arc;

use bitwarden::tool::{PassphraseGeneratorRequest, PasswordGeneratorRequest};

use crate::{error::Result, Client};

#[derive(uniffi::Object)]
pub struct ClientGenerators(pub(crate) Arc<Client>);

#[uniffi::export]
impl ClientGenerators {
    /// Generate Password
    pub async fn password(&self, settings: PasswordGeneratorRequest) -> Result<String> {
        Ok(self
            .0
             .0
            .read()
            .await
            .generator()
            .password(settings)
            .await?)
    }

    /// Generate Passphrase
    pub async fn passphrase(&self, settings: PassphraseGeneratorRequest) -> Result<String> {
        Ok(self
            .0
             .0
            .read()
            .await
            .generator()
            .passphrase(settings)
            .await?)
    }
}
