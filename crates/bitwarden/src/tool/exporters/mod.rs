use bitwarden_core::VaultLocked;
use bitwarden_crypto::KeyDecryptable;
use bitwarden_exporters::export;
use bitwarden_vault::{Cipher, CipherView, Collection, Folder, FolderView};
use schemars::JsonSchema;

use crate::{
    client::{LoginMethod, UserLoginMethod},
    error::{Error, Result},
    Client,
};

mod client_exporter;
pub use client_exporter::ClientExporters;

#[derive(JsonSchema)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
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
    let key = enc.get_key(&None).ok_or(VaultLocked)?;

    let folders: Vec<FolderView> = folders.decrypt_with_key(key)?;
    let folders: Vec<bitwarden_exporters::Folder> =
        folders.into_iter().flat_map(|f| f.try_into()).collect();

    let ciphers: Vec<CipherView> = ciphers.decrypt_with_key(key)?;
    let ciphers: Vec<bitwarden_exporters::Cipher> =
        ciphers.into_iter().flat_map(|c| c.try_into()).collect();

    let format = convert_format(client, format)?;

    Ok(export(folders, ciphers, format)?)
}

fn convert_format(
    client: &Client,
    format: ExportFormat,
) -> Result<bitwarden_exporters::Format, Error> {
    let login_method = client
        .login_method
        .as_ref()
        .ok_or(Error::NotAuthenticated)?;

    let kdf = match login_method {
        LoginMethod::User(
            UserLoginMethod::Username { kdf, .. } | UserLoginMethod::ApiKey { kdf, .. },
        ) => kdf,
        _ => return Err(Error::NotAuthenticated),
    };

    Ok(match format {
        ExportFormat::Csv => bitwarden_exporters::Format::Csv,
        ExportFormat::Json => bitwarden_exporters::Format::Json,
        ExportFormat::EncryptedJson { password } => bitwarden_exporters::Format::EncryptedJson {
            password,
            kdf: kdf.clone(),
        },
    })
}

pub(super) fn export_organization_vault(
    _collections: Vec<Collection>,
    _ciphers: Vec<Cipher>,
    _format: ExportFormat,
) -> Result<String> {
    todo!();
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use bitwarden_crypto::Kdf;

    use super::*;

    #[test]
    fn test_convert_format() {
        let client = Client::new(None);
        client.set_login_method(LoginMethod::User(UserLoginMethod::Username {
            client_id: "7b821276-e27c-400b-9853-606393c87f18".to_owned(),
            email: "test@bitwarden.com".to_owned(),
            kdf: Kdf::PBKDF2 {
                iterations: NonZeroU32::new(600_000).unwrap(),
            },
        }));

        assert!(matches!(
            convert_format(&client, ExportFormat::Csv).unwrap(),
            bitwarden_exporters::Format::Csv
        ));
        assert!(matches!(
            convert_format(&client, ExportFormat::Json).unwrap(),
            bitwarden_exporters::Format::Json
        ));
        assert!(matches!(
            convert_format(
                &client,
                ExportFormat::EncryptedJson {
                    password: "password".to_string()
                }
            )
            .unwrap(),
            bitwarden_exporters::Format::EncryptedJson { .. }
        ));
    }
}
