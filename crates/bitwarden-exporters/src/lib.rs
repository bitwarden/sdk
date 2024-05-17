use std::fmt;

use bitwarden_crypto::{DecryptedString, Kdf};
use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

mod csv;
use crate::csv::export_csv;
mod json;
use json::export_json;
mod encrypted_json;

use encrypted_json::export_encrypted_json;

pub enum Format {
    Csv,
    Json,
    EncryptedJson { password: String, kdf: Kdf },
}

/// Export representation of a Bitwarden folder.
///
/// These are mostly duplicated from the `bitwarden` vault models to facilitate a stable export API
/// that is not tied to the internal vault models. We may revisit this in the future.
pub struct Folder {
    pub id: Uuid,
    pub name: DecryptedString,
}

/// Export representation of a Bitwarden cipher.
///
/// These are mostly duplicated from the `bitwarden` vault models to facilitate a stable export API
/// that is not tied to the internal vault models. We may revisit this in the future.
pub struct Cipher {
    pub id: Uuid,
    pub folder_id: Option<Uuid>,

    pub name: DecryptedString,
    pub notes: Option<DecryptedString>,

    pub r#type: CipherType,

    pub favorite: bool,
    pub reprompt: u8,

    pub fields: Vec<Field>,

    pub revision_date: DateTime<Utc>,
    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub struct Field {
    pub name: Option<DecryptedString>,
    pub value: Option<DecryptedString>,
    pub r#type: u8,
    pub linked_id: Option<u32>,
}

pub enum CipherType {
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

pub struct Login {
    pub username: Option<DecryptedString>,
    pub password: Option<DecryptedString>,
    pub login_uris: Vec<LoginUri>,
    pub totp: Option<DecryptedString>,
}

pub struct LoginUri {
    pub uri: Option<DecryptedString>,
    pub r#match: Option<u8>,
}

pub struct Card {
    pub cardholder_name: Option<DecryptedString>,
    pub exp_month: Option<DecryptedString>,
    pub exp_year: Option<DecryptedString>,
    pub code: Option<DecryptedString>,
    pub brand: Option<DecryptedString>,
    pub number: Option<DecryptedString>,
}

pub struct SecureNote {
    pub r#type: SecureNoteType,
}

pub enum SecureNoteType {
    Generic = 0,
}

pub struct Identity {
    pub title: Option<DecryptedString>,
    pub first_name: Option<DecryptedString>,
    pub middle_name: Option<DecryptedString>,
    pub last_name: Option<DecryptedString>,
    pub address1: Option<DecryptedString>,
    pub address2: Option<DecryptedString>,
    pub address3: Option<DecryptedString>,
    pub city: Option<DecryptedString>,
    pub state: Option<DecryptedString>,
    pub postal_code: Option<DecryptedString>,
    pub country: Option<DecryptedString>,
    pub company: Option<DecryptedString>,
    pub email: Option<DecryptedString>,
    pub phone: Option<DecryptedString>,
    pub ssn: Option<DecryptedString>,
    pub username: Option<DecryptedString>,
    pub passport_number: Option<DecryptedString>,
    pub license_number: Option<DecryptedString>,
}

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("CSV error: {0}")]
    Csv(#[from] csv::CsvError),
    #[error("JSON error: {0}")]
    Json(#[from] json::JsonError),
    #[error("Encrypted JSON error: {0}")]
    EncryptedJsonError(#[from] encrypted_json::EncryptedJsonError),
}

pub fn export(
    folders: Vec<Folder>,
    ciphers: Vec<Cipher>,
    format: Format,
) -> Result<String, ExportError> {
    match format {
        Format::Csv => Ok(export_csv(folders, ciphers)?),
        Format::Json => Ok(export_json(folders, ciphers)?),
        Format::EncryptedJson { password, kdf } => {
            Ok(export_encrypted_json(folders, ciphers, password, kdf)?)
        }
    }
}
