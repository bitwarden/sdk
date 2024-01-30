use bitwarden_crypto::Decryptable;
use bitwarden_exporters::export;
use schemars::JsonSchema;

use crate::{
    error::{Error, Result},
    vault::{Cipher, CipherView, Collection, Folder, FolderView},
    Client,
};

mod client_exporter;
pub use client_exporter::ClientExporters;

#[derive(JsonSchema)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum ExportFormat {
    Csv,
    Json,
    EncryptedJson { password: String },
}

pub(super) fn export_vault(
    client: &Client,
    folders: Vec<Folder>,
    ciphers: Vec<Cipher>,
    format: ExportFormat,
) -> Result<String> {
    let enc = client.get_encryption_settings()?;

    let mut folders: Vec<FolderView> = folders.decrypt(enc, &None)?;
    let folders: Vec<bitwarden_exporters::Folder> =
        folders.into_iter().flat_map(|f| f.try_into()).collect();

    let mut ciphers: Vec<CipherView> = ciphers.decrypt(enc, &None)?;
    let ciphers: Vec<bitwarden_exporters::Cipher> =
        ciphers.into_iter().flat_map(|c| c.try_into()).collect();

    Ok(export(folders, ciphers, bitwarden_exporters::Format::Csv))
}

pub(super) fn export_organization_vault(
    _collections: Vec<Collection>,
    _ciphers: Vec<Cipher>,
    _format: ExportFormat,
) -> Result<String> {
    todo!();
}

impl TryFrom<FolderView> for bitwarden_exporters::Folder {
    type Error = Error;

    fn try_from(value: FolderView) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.ok_or(Error::MissingFields)?,
            name: value.name,
            revision_date: value.revision_date,
        })
    }
}

impl TryFrom<CipherView> for bitwarden_exporters::Cipher {
    type Error = Error;

    fn try_from(value: CipherView) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.ok_or(Error::MissingFields)?,
        })
    }
}
