use schemars::JsonSchema;

use crate::{
    error::Result,
    vault::{Cipher, Collection, Folder},
};

mod client_exporter;
pub use client_exporter::ClientExporters;

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
    format: ExportFormat,
) -> Result<String> {
    Ok(match format {
        ExportFormat::Csv => "Csv".to_owned(),
        ExportFormat::Json => "Json".to_owned(),
        ExportFormat::AccountEncryptedJson => "AccountEncryptedJson".to_owned(),
        ExportFormat::EncryptedJson { .. } => "EncryptedJson".to_owned(),
    })
}

pub(super) fn export_organization_vault(
    _collections: Vec<Collection>,
    _ciphers: Vec<Cipher>,
    _format: ExportFormat,
) -> Result<String> {
    todo!();
}
