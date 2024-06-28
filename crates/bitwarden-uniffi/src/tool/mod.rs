use std::sync::Arc;

use bitwarden::{
    error::Error,
    exporters::{ClientExportersExt, ExportFormat},
    generators::{
        ClientGeneratorExt, PassphraseGeneratorRequest, PasswordGeneratorRequest,
        UsernameGeneratorRequest,
    },
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
    pub fn password(&self, settings: PasswordGeneratorRequest) -> Result<String> {
        Ok(self
            .0
             .0
            .generator()
            .password(settings)
            .map_err(Error::PasswordError)?)
    }

    /// **API Draft:** Generate Passphrase
    pub fn passphrase(&self, settings: PassphraseGeneratorRequest) -> Result<String> {
        Ok(self
            .0
             .0
            .generator()
            .passphrase(settings)
            .map_err(Error::PassphraseError)?)
    }

    /// **API Draft:** Generate Username
    pub async fn username(&self, settings: UsernameGeneratorRequest) -> Result<String> {
        Ok(self
            .0
             .0
            .generator()
            .username(settings)
            .await
            .map_err(Error::UsernameError)?)
    }
}

#[derive(uniffi::Object)]
pub struct ClientExporters(pub(crate) Arc<Client>);

#[uniffi::export]
impl ClientExporters {
    /// **API Draft:** Export user vault
    pub fn export_vault(
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
            .map_err(Error::ExportError)?)
    }

    /// **API Draft:** Export organization vault
    pub fn export_organization_vault(
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
            .map_err(Error::ExportError)?)
    }
}
