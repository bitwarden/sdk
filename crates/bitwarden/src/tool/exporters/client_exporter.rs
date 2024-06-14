use bitwarden_vault::{Cipher, Collection, Folder};

use crate::{
    error::Result,
    tool::exporters::{export_organization_vault, export_vault, ExportFormat},
    Client,
};

pub struct ClientExporters<'a> {
    pub(crate) client: &'a crate::Client,
}

impl<'a> ClientExporters<'a> {
    /// **Draft:** Export the vault as a CSV, JSON, or encrypted JSON file.
    pub fn export_vault(
        &self,
        folders: Vec<Folder>,
        ciphers: Vec<Cipher>,
        format: ExportFormat,
    ) -> Result<String> {
        export_vault(self.client, folders, ciphers, format)
    }

    pub fn export_organization_vault(
        &self,
        collections: Vec<Collection>,
        ciphers: Vec<Cipher>,
        format: ExportFormat,
    ) -> Result<String> {
        export_organization_vault(collections, ciphers, format)
    }
}

impl<'a> Client {
    pub fn exporters(&'a self) -> ClientExporters<'a> {
        ClientExporters { client: self }
    }
}
