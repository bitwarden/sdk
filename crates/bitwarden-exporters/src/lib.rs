use chrono::{DateTime, Utc};
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

pub struct Folder {
    pub id: Uuid,
    pub name: String,
}

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
    name: Option<String>,
    value: Option<String>,
    r#type: u8,
    linked_id: Option<u8>,
}

pub enum CipherType {
    Login(Login),
    Identity(),
    Card(Card),
    SecureNote(SecureNote),
}

impl ToString for CipherType {
    fn to_string(&self) -> String {
        match self {
            CipherType::Login(_) => "login".to_string(),
            CipherType::SecureNote(_) => "note".to_string(),
            CipherType::Card(_) => "card".to_string(),
            CipherType::Identity() => "identity".to_string(),
        }
    }
}

pub struct Login {
    pub username: String,
    pub password: String,
    pub login_uris: Vec<LoginUri>,
    pub totp: Option<String>,
}

pub struct LoginUri {
    pub uri: Option<String>,
    pub r#match: Option<UriMatchType>,
}

pub enum UriMatchType {
    Domain = 0,
    Host = 1,
    StartsWith = 2,
    Exact = 3,
    RegularExpression = 4,
    Never = 5,
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

pub fn export(folders: Vec<Folder>, ciphers: Vec<Cipher>, format: Format) -> String {
    match format {
        Format::Csv => export_csv(folders, ciphers).unwrap(),
        Format::Json => export_json(folders, ciphers).unwrap(),
        // Format::EncryptedJson { password } => export_encrypted_json(folders, ciphers, password),
        _ => todo!(),
    }
}
