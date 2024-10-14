use bitwarden_core::Client;
use bitwarden_crypto::KeyDecryptable;
use bitwarden_vault::{Cipher, CipherView, Collection, Folder, FolderView};

use crate::{
    csv::export_csv, encrypted_json::export_encrypted_json, json::export_json, ExportError,
    ExportFormat,
};

pub(crate) fn export_vault(
    client: &Client,
    folders: Vec<Folder>,
    ciphers: Vec<Cipher>,
    format: ExportFormat,
) -> Result<String, ExportError> {
    let enc = client.internal.get_encryption_settings()?;
    let key = enc.get_key(&None)?;

    let folders: Vec<FolderView> = folders.decrypt_with_key(key)?;
    let folders: Vec<crate::Folder> = folders.into_iter().flat_map(|f| f.try_into()).collect();

    let ciphers: Vec<CipherView> = ciphers.decrypt_with_key(key)?;
    let ciphers: Vec<crate::Cipher> = ciphers.into_iter().flat_map(|c| c.try_into()).collect();

    match format {
        ExportFormat::Csv => Ok(export_csv(folders, ciphers)?),
        ExportFormat::Json => Ok(export_json(folders, ciphers)?),
        ExportFormat::EncryptedJson { password } => Ok(export_encrypted_json(
            folders,
            ciphers,
            password,
            client.internal.get_kdf()?,
        )?),
    }
}

pub(crate) fn export_organization_vault(
    _collections: Vec<Collection>,
    _ciphers: Vec<Cipher>,
    _format: ExportFormat,
) -> Result<String, ExportError> {
    todo!();
}
