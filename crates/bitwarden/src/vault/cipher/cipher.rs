use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::crypto::CipherString;

use super::{attachment, card, field, identity, login, password_history, secure_note};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, JsonSchema)]
pub enum CipherType {
    Login = 1,
    SecureNote = 2,
    Card = 3,
    Identity = 4,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, JsonSchema)]
pub enum CipherRepromptType {
    None = 0,
    Password = 1,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Cipher {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,

    pub name: CipherString,
    pub notes: CipherString,

    pub r#type: CipherType,
    pub login: Option<login::Login>,
    pub identity: Option<identity::Identity>,
    pub card: Option<card::Card>,
    pub secure_note: Option<secure_note::SecureNote>,

    pub collection_ids: Vec<Uuid>,
    pub favorite: bool,
    pub reprompt: CipherRepromptType,

    pub attachments: Vec<attachment::Attachment>,
    pub fields: Vec<field::Field>,
    pub password_history: Vec<password_history::PasswordHistory>,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherView {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,

    pub name: String,
    pub notes: String,

    pub r#type: CipherType,
    pub login: Option<login::LoginView>,
    pub identity: Option<identity::IdentityView>,
    pub card: Option<card::CardView>,
    pub secure_note: Option<secure_note::SecureNote>,

    pub collection_ids: Vec<Uuid>,
    pub favorite: bool,
    pub reprompt: CipherRepromptType,

    pub attachments: Vec<attachment::AttachmentView>,
    pub fields: Vec<field::FieldView>,
    pub password_history: Vec<password_history::PasswordHistoryView>,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherListView {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,

    pub name: String,
    pub notes: String,

    pub r#type: CipherType,
    pub login: Option<login::LoginView>,
    pub identity: Option<identity::IdentityView>,
    pub card: Option<card::CardView>,
    pub secure_note: Option<secure_note::SecureNote>,

    pub collection_ids: Vec<Uuid>,
    pub favorite: bool,
    pub reprompt: CipherRepromptType,

    pub attachments: Vec<attachment::AttachmentView>,
    pub fields: Vec<field::FieldView>,
    pub password_history: Vec<password_history::PasswordHistoryView>,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}
