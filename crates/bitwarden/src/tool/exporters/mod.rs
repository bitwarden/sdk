use schemars::JsonSchema;

use crate::{
    error::Result,
    vault::{CipherView, CollectionView, FolderView},
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
    _folders: Vec<FolderView>,
    _ciphers: Vec<CipherView>,
    _format: ExportFormat,
) -> Result<String> {
    todo!();
}

pub(super) fn export_organization_vault(
    _collections: Vec<CollectionView>,
    _ciphers: Vec<CipherView>,
    _format: ExportFormat,
) -> Result<String> {
    todo!();
}
