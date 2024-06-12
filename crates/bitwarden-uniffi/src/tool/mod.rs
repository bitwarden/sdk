use std::sync::Arc;

use bitwarden::{
    generators::{PassphraseGeneratorRequest, PasswordGeneratorRequest, UsernameGeneratorRequest},
    tool::ExportFormat,
    vault::{Cipher, Collection, Folder},
};

use crate::{error::Result, Client};

mod sends;
pub use sends::ClientSends;

#[derive(uniffi::Object)]
pub struct ClientGenerators(pub(crate) Arc<Client>);

#[uniffi::export(async_runtime = "tokio")]
impl ClientGenerators {
    /// **API Draft:** Generate Password
    pub async fn password(&self, settings: PasswordGeneratorRequest) -> Result<String> {
        Ok(self.0 .0.generator().password(settings).await?)
    }

    /// **API Draft:** Generate Passphrase
    pub async fn passphrase(&self, settings: PassphraseGeneratorRequest) -> Result<String> {
        Ok(self.0 .0.generator().passphrase(settings).await?)
    }

    /// **API Draft:** Generate Username
    pub async fn username(&self, settings: UsernameGeneratorRequest) -> Result<String> {
        Ok(self.0 .0.generator().username(settings).await?)
    }
}

#[derive(uniffi::Object)]
pub struct ClientExporters(pub(crate) Arc<Client>);

#[uniffi::export(async_runtime = "tokio")]
impl ClientExporters {
    /// **API Draft:** Export user vault
    pub async fn export_vault(
        &self,
        folders: Vec<Folder>,
        ciphers: Vec<Cipher>,
        format: ExportFormat,
    ) -> Result<String> {
        Ok(self
            .0
             .0
            .exporters()
            .export_vault(folders, ciphers, format)
            .await?)
    }

    /// **API Draft:** Export organization vault
    pub async fn export_organization_vault(
        &self,
        collections: Vec<Collection>,
        ciphers: Vec<Cipher>,
        format: ExportFormat,
    ) -> Result<String> {
        Ok(self
            .0
             .0
            .exporters()
            .export_organization_vault(collections, ciphers, format)
            .await?)
    }
}
