use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

mod csv;
use csv::export_csv;
mod json;
use json::export_json;

pub enum Format {
    Csv,
    Json,
    EncryptedJson { password: String },
}

/// Export representation of a Bitwarden folder.
///
/// These are mostly duplicated from the `bitwarden` vault models to facilitate a stable export API
/// that is not tied to the internal vault models. We may revisit this in the future.
pub struct Folder {
    pub id: Uuid,
    pub name: String,
}

/// Export representation of a Bitwarden cipher.
///
/// These are mostly duplicated from the `bitwarden` vault models to facilitate a stable export API
/// that is not tied to the internal vault models. We may revisit this in the future.
pub struct Cipher {
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
pub struct Field {
    pub name: Option<String>,
    pub value: Option<String>,
    pub r#type: u8,
    pub linked_id: Option<u32>,
}

pub enum CipherType {
    Login(Box<Login>),
    SecureNote(Box<SecureNote>),
    Card(Box<Card>),
    Identity(Box<Identity>),
}

impl ToString for CipherType {
    fn to_string(&self) -> String {
        match self {
            CipherType::Login(_) => "login".to_string(),
            CipherType::SecureNote(_) => "note".to_string(),
            CipherType::Card(_) => "card".to_string(),
            CipherType::Identity(_) => "identity".to_string(),
        }
    }
}

pub struct Login {
    pub username: Option<String>,
    pub password: Option<String>,
    pub login_uris: Vec<LoginUri>,
    pub totp: Option<String>,
}

pub struct LoginUri {
    pub uri: Option<String>,
    pub r#match: Option<u8>,
}

pub struct Card {
    pub cardholder_name: Option<String>,
    pub exp_month: Option<String>,
    pub exp_year: Option<String>,
    pub code: Option<String>,
    pub brand: Option<String>,
    pub number: Option<String>,
}

pub struct SecureNote {
    pub r#type: SecureNoteType,
}

pub enum SecureNoteType {
    Generic = 0,
}

pub struct Identity {
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

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("CSV error: {0}")]
    Csv(#[from] csv::CsvError),
    #[error("JSON error: {0}")]
    Json(#[from] json::JsonError),
}

pub fn export(
    folders: Vec<Folder>,
    ciphers: Vec<Cipher>,
    format: Format,
) -> Result<String, ExportError> {
    match format {
        Format::Csv => Ok(export_csv(folders, ciphers)?),
        Format::Json => Ok(export_json(folders, ciphers)?),
        Format::EncryptedJson { password: _ } => todo!(),
    }
}
