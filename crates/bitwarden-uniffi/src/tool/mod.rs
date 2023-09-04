use std::sync::Arc;

use bitwarden::{
    tool::{ExportFormat, PassphraseGeneratorRequest, PasswordGeneratorRequest},
    vault::{CipherView, CollectionView, FolderView},
};

use crate::{error::Result, Client};

#[derive(uniffi::Object)]
pub struct ClientGenerators(pub(crate) Arc<Client>);

#[uniffi::export]
impl ClientGenerators {
    /// **API Draft:** Generate Password
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

    /// **API Draft:** Generate Passphrase
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

#[derive(uniffi::Object)]
pub struct ClientExporters(pub(crate) Arc<Client>);

#[uniffi::export]
impl ClientExporters {
    /// **API Draft:** Export user vault
    pub async fn export_vault(
        &self,
        folders: Vec<FolderView>,
        ciphers: Vec<CipherView>,
        format: ExportFormat,
    ) -> Result<String> {
        Ok(self
            .0
             .0
            .read()
            .await
            .exporters()
            .export_vault(folders, ciphers, format)
            .await?)
    }

    /// **API Draft:** Export organization vault
    pub async fn export_organization_vault(
        &self,
        collections: Vec<CollectionView>,
        ciphers: Vec<CipherView>,
        format: ExportFormat,
    ) -> Result<String> {
        Ok(self
            .0
             .0
            .read()
            .await
            .exporters()
            .export_organization_vault(collections, ciphers, format)
            .await?)
    }
}
