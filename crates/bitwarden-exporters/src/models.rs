use std::fmt;

use bitwarden_core::require;
use bitwarden_vault::{CipherView, FieldView, FolderView, LoginUriView};

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::ExportError;

/// Export representation of a Bitwarden folder.
///
/// These are mostly duplicated from the `bitwarden` vault models to facilitate a stable export API
/// that is not tied to the internal vault models. We may revisit this in the future.
pub(crate) struct Folder {
    pub id: Uuid,
    pub name: String,
}

/// Export representation of a Bitwarden cipher.
///
/// These are mostly duplicated from the `bitwarden` vault models to facilitate a stable export API
/// that is not tied to the internal vault models. We may revisit this in the future.
pub(crate) struct Cipher {
    pub id: Uuid,
    pub folder_id: Option<Uuid>,

    pub name: String,
    pub notes: Option<String>,

    pub r#type: CipherType,

    pub favorite: bool,
    pub reprompt: u8,

    pub fields: Vec<Field>,

    pub revision_date: DateTime<Utc>,
    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub(crate) struct Field {
    pub name: Option<String>,
    pub value: Option<String>,
    pub r#type: u8,
    pub linked_id: Option<u32>,
}

pub(crate) enum CipherType {
    Login(Box<Login>),
    SecureNote(Box<SecureNote>),
    Card(Box<Card>),
    Identity(Box<Identity>),
}

impl fmt::Display for CipherType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CipherType::Login(_) => write!(f, "login"),
            CipherType::SecureNote(_) => write!(f, "note"),
            CipherType::Card(_) => write!(f, "card"),
            CipherType::Identity(_) => write!(f, "identity"),
        }
    }
}

pub(crate) struct Login {
    pub username: Option<String>,
    pub password: Option<String>,
    pub login_uris: Vec<LoginUri>,
    pub totp: Option<String>,
}

pub(crate) struct LoginUri {
    pub uri: Option<String>,
    pub r#match: Option<u8>,
}

pub(crate) struct Card {
    pub cardholder_name: Option<String>,
    pub exp_month: Option<String>,
    pub exp_year: Option<String>,
    pub code: Option<String>,
    pub brand: Option<String>,
    pub number: Option<String>,
}

pub(crate) struct SecureNote {
    pub r#type: SecureNoteType,
}

pub(crate) enum SecureNoteType {
    Generic = 0,
}

pub(crate) struct Identity {
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ssn: Option<String>,
    pub username: Option<String>,
    pub passport_number: Option<String>,
    pub license_number: Option<String>,
}

impl TryFrom<FolderView> for Folder {
    type Error = ExportError;

    fn try_from(value: FolderView) -> Result<Self, Self::Error> {
        Ok(Self {
            id: require!(value.id),
            name: value.name,
        })
    }
}

impl TryFrom<CipherView> for Cipher {
    type Error = ExportError;

    fn try_from(value: CipherView) -> Result<Self, Self::Error> {
        let r = match value.r#type {
            bitwarden_vault::CipherType::Login => {
                let l = require!(value.login);
                CipherType::Login(Box::new(Login {
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
            bitwarden_vault::CipherType::SecureNote => {
                CipherType::SecureNote(Box::new(SecureNote {
                    r#type: value
                        .secure_note
                        .map(|t| t.r#type)
                        .unwrap_or(bitwarden_vault::SecureNoteType::Generic)
                        .into(),
                }))
            }
            bitwarden_vault::CipherType::Card => {
                let c = require!(value.card);
                CipherType::Card(Box::new(Card {
                    cardholder_name: c.cardholder_name,
                    exp_month: c.exp_month,
                    exp_year: c.exp_year,
                    code: c.code,
                    brand: c.brand,
                    number: c.number,
                }))
            }
            bitwarden_vault::CipherType::Identity => {
                let i = require!(value.identity);
                CipherType::Identity(Box::new(Identity {
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
            id: require!(value.id),
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

impl From<FieldView> for Field {
    fn from(value: FieldView) -> Self {
        Self {
            name: value.name,
            value: value.value,
            r#type: value.r#type as u8,
            linked_id: value.linked_id.map(|id| id.into()),
        }
    }
}

impl From<LoginUriView> for LoginUri {
    fn from(value: LoginUriView) -> Self {
        Self {
            r#match: value.r#match.map(|v| v as u8),
            uri: value.uri,
        }
    }
}

impl From<bitwarden_vault::SecureNoteType> for SecureNoteType {
    fn from(value: bitwarden_vault::SecureNoteType) -> Self {
        match value {
            bitwarden_vault::SecureNoteType::Generic => SecureNoteType::Generic,
        }
    }
}

#[cfg(test)]
mod tests {
    use bitwarden_vault::{CipherRepromptType, LoginView};
    use chrono::{DateTime, Utc};

    use super::*;

    #[test]
    fn test_try_from_folder_view() {
        let view = FolderView {
            id: Some("fd411a1a-fec8-4070-985d-0e6560860e69".parse().unwrap()),
            name: "test_name".to_string(),
            revision_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
        };

        let f: Folder = view.try_into().unwrap();

        assert_eq!(
            f.id,
            "fd411a1a-fec8-4070-985d-0e6560860e69".parse().unwrap()
        );
        assert_eq!(f.name, "test_name".to_string());
    }

    #[test]
    fn test_try_from_cipher_view_login() {
        let cipher_view = CipherView {
            r#type: bitwarden_vault::CipherType::Login,
            login: Some(LoginView {
                username: Some("test_username".to_string()),
                password: Some("test_password".to_string()),
                password_revision_date: None,
                uris: None,
                totp: None,
                autofill_on_page_load: None,
                fido2_credentials: None,
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

        let cipher: Cipher = cipher_view.try_into().unwrap();

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

        if let CipherType::Login(l) = cipher.r#type {
            assert_eq!(l.username, Some("test_username".to_string()));
            assert_eq!(l.password, Some("test_password".to_string()));
            assert!(l.login_uris.is_empty());
            assert_eq!(l.totp, None);
        } else {
            panic!("Expected login type");
        }
    }
}
