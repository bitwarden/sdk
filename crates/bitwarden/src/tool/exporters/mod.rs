use schemars::JsonSchema;

use crate::{
    error::Result,
    vault::{Cipher, Collection, Folder},
};

mod client_exporter;

#[derive(JsonSchema)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum ExportFormat {
    Csv,
    Json,
    AccountEncryptedJson, // TODO: Should we deprecate this option completely?
    EncryptedJson { password: String },
}

pub(super) fn export_vault(
    _folders: Vec<Folder>,
    _ciphers: Vec<Cipher>,
    _format: ExportFormat,
) -> Result<String> {
    todo!();
}

pub(super) fn export_organization_vault(
    _collections: Vec<Collection>,
    _ciphers: Vec<Cipher>,
    _format: ExportFormat,
) -> Result<String> {
    todo!();
}
