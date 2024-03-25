use super::export_vault_attachments;
use crate::{
    error::Result,
    tool::exporters::{export_organization_vault, export_vault, ExportFormat},
    vault::{Cipher, Collection, Folder},
    Client,
};

pub struct ClientExporters<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientExporters<'a> {
    /// **Draft:** Export the vault as a CSV, JSON, or encrypted JSON file.
    pub async fn export_vault(
        &self,
        folders: Vec<Folder>,
        ciphers: Vec<Cipher>,
        format: ExportFormat,
    ) -> Result<String> {
        export_vault(self.client, folders, ciphers, format)
    }

    pub async fn export_organization_vault(
        &self,
        collections: Vec<Collection>,
        ciphers: Vec<Cipher>,
        format: ExportFormat,
    ) -> Result<String> {
        export_organization_vault(collections, ciphers, format)
    }

    pub async fn export_vault_attachments(&mut self) -> Result<()> {
        export_vault_attachments(self.client).await
    }
}

impl<'a> Client {
    pub fn exporters(&'a mut self) -> ClientExporters<'a> {
        ClientExporters { client: self }
    }
}
