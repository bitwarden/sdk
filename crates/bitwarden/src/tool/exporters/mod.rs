use bitwarden_crypto::Decryptable;
use bitwarden_exporters::export;
use schemars::JsonSchema;

use crate::{
    error::{Error, Result},
    vault::{
        login::LoginUriView, Cipher, CipherType, CipherView, Collection, FieldView, Folder,
        FolderView, SecureNoteType,
    },
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

    let folders: Vec<FolderView> = folders.decrypt(enc, &None)?;
    let folders: Vec<bitwarden_exporters::Folder> =
        folders.into_iter().flat_map(|f| f.try_into()).collect();

    let ciphers: Vec<CipherView> = ciphers.decrypt(enc, &None)?;
    let ciphers: Vec<bitwarden_exporters::Cipher> =
        ciphers.into_iter().flat_map(|c| c.try_into()).collect();

    Ok(export(folders, ciphers, format.into())?)
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
        })
    }
}

impl TryFrom<CipherView> for bitwarden_exporters::Cipher {
    type Error = Error;

    fn try_from(value: CipherView) -> Result<Self, Self::Error> {
        let r = match value.r#type {
            CipherType::Login => {
                let l = value.login.ok_or(Error::MissingFields)?;
                bitwarden_exporters::CipherType::Login(Box::new(bitwarden_exporters::Login {
                    username: l.username,
                    password: l.password,
                    login_uris: l
                        .uris
                        .unwrap_or_default()
                        .into_iter()
                        .map(|u| u.into())
                        .collect(),
                    totp: l.totp,
                }))
            }
            CipherType::SecureNote => bitwarden_exporters::CipherType::SecureNote(Box::new(
                bitwarden_exporters::SecureNote {
                    r#type: value
                        .secure_note
                        .map(|t| t.r#type)
                        .unwrap_or(SecureNoteType::Generic)
                        .into(),
                },
            )),
            CipherType::Card => {
                let c = value.card.ok_or(Error::MissingFields)?;
                bitwarden_exporters::CipherType::Card(Box::new(bitwarden_exporters::Card {
                    cardholder_name: c.cardholder_name,
                    exp_month: c.exp_month,
                    exp_year: c.exp_year,
                    code: c.code,
                    brand: c.brand,
                    number: c.number,
                }))
            }
            CipherType::Identity => {
                let i = value.identity.ok_or(Error::MissingFields)?;
                bitwarden_exporters::CipherType::Identity(Box::new(bitwarden_exporters::Identity {
                    title: i.title,
                    first_name: i.first_name,
                    middle_name: i.middle_name,
                    last_name: i.last_name,
                    address1: i.address1,
                    address2: i.address2,
                    address3: i.address3,
                    city: i.city,
                    state: i.state,
                    postal_code: i.postal_code,
                    country: i.country,
                    company: i.company,
                    email: i.email,
                    phone: i.phone,
                    ssn: i.ssn,
                    username: i.username,
                    passport_number: i.passport_number,
                    license_number: i.license_number,
                }))
            }
        };

        Ok(Self {
            id: value.id.ok_or(Error::MissingFields)?,
            folder_id: value.folder_id,
            name: value.name,
            notes: value.notes,
            r#type: r,
            favorite: value.favorite,
            reprompt: value.reprompt as u8,
            fields: value
                .fields
                .unwrap_or_default()
                .into_iter()
                .map(|f| f.into())
                .collect(),
            revision_date: value.revision_date,
            creation_date: value.creation_date,
            deleted_date: value.deleted_date,
        })
    }
}

impl From<FieldView> for bitwarden_exporters::Field {
    fn from(value: FieldView) -> Self {
        Self {
            name: value.name,
            value: value.value,
            r#type: value.r#type as u8,
            linked_id: value.linked_id.map(|id| id.into()),
        }
    }
}

impl From<LoginUriView> for bitwarden_exporters::LoginUri {
    fn from(value: LoginUriView) -> Self {
        Self {
            r#match: value.r#match.map(|v| v as u8),
            uri: value.uri,
        }
    }
}

impl From<SecureNoteType> for bitwarden_exporters::SecureNoteType {
    fn from(value: SecureNoteType) -> Self {
        match value {
            SecureNoteType::Generic => bitwarden_exporters::SecureNoteType::Generic,
        }
    }
}

impl From<ExportFormat> for bitwarden_exporters::Format {
    fn from(value: ExportFormat) -> Self {
        match value {
            ExportFormat::Csv => Self::Csv,
            ExportFormat::Json => Self::Json,
            ExportFormat::EncryptedJson { password } => Self::EncryptedJson { password },
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};

    use super::*;
    use crate::vault::{login::LoginView, CipherRepromptType};

    #[test]
    fn test_try_from_folder_view() {
        let view = FolderView {
            id: Some("fd411a1a-fec8-4070-985d-0e6560860e69".parse().unwrap()),
            name: "test_name".to_string(),
            revision_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
        };

        let f: bitwarden_exporters::Folder = view.try_into().unwrap();

        assert_eq!(
            f.id,
            "fd411a1a-fec8-4070-985d-0e6560860e69".parse().unwrap()
        );
        assert_eq!(f.name, "test_name".to_string());
    }

    #[test]
    fn test_try_from_cipher_view_login() {
        let cipher_view = CipherView {
            r#type: CipherType::Login,
            login: Some(LoginView {
                username: Some("test_username".to_string()),
                password: Some("test_password".to_string()),
                password_revision_date: None,
                uris: None,
                totp: None,
                autofill_on_page_load: None,
            }),
            id: "fd411a1a-fec8-4070-985d-0e6560860e69".parse().ok(),
            organization_id: None,
            folder_id: None,
            collection_ids: vec![],
            key: None,
            name: "My login".to_string(),
            notes: None,
            identity: None,
            card: None,
            secure_note: None,
            favorite: false,
            reprompt: CipherRepromptType::None,
            organization_use_totp: true,
            edit: true,
            view_password: true,
            local_data: None,
            attachments: None,
            fields: None,
            password_history: None,
            creation_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
            deleted_date: None,
            revision_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
        };

        let cipher: bitwarden_exporters::Cipher = cipher_view.try_into().unwrap();

        assert_eq!(
            cipher.id,
            "fd411a1a-fec8-4070-985d-0e6560860e69".parse().unwrap()
        );
        assert_eq!(cipher.folder_id, None);
        assert_eq!(cipher.name, "My login".to_string());
        assert_eq!(cipher.notes, None);
        assert!(!cipher.favorite);
        assert_eq!(cipher.reprompt, 0);
        assert!(cipher.fields.is_empty());
        assert_eq!(
            cipher.revision_date,
            "2024-01-30T17:55:36.150Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(
            cipher.creation_date,
            "2024-01-30T17:55:36.150Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(cipher.deleted_date, None);

        if let bitwarden_exporters::CipherType::Login(l) = cipher.r#type {
            assert_eq!(l.username, Some("test_username".to_string()));
            assert_eq!(l.password, Some("test_password".to_string()));
            assert!(l.login_uris.is_empty());
            assert_eq!(l.totp, None);
        } else {
            panic!("Expected login type");
        }
    }

    #[test]
    fn test_from_export_format() {
        assert!(matches!(
            bitwarden_exporters::Format::from(ExportFormat::Csv),
            bitwarden_exporters::Format::Csv
        ));
        assert!(matches!(
            bitwarden_exporters::Format::from(ExportFormat::Json),
            bitwarden_exporters::Format::Json
        ));
        assert!(matches!(
            bitwarden_exporters::Format::from(ExportFormat::EncryptedJson {
                password: "password".to_string()
            }),
            bitwarden_exporters::Format::EncryptedJson { .. }
        ));
    }
}
