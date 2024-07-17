use bitwarden_core::{MissingFieldError, VaultLocked};
use bitwarden_crypto::{CryptoError, EncString};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use thiserror::Error;

use super::{attachment, card, field, identity, login, secure_note};
use crate::password_history;

#[derive(Debug, Error)]
pub enum CipherError {
    #[error(transparent)]
    MissingFieldError(#[from] MissingFieldError),
    #[error(transparent)]
    VaultLocked(#[from] VaultLocked),
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error("This cipher contains attachments without keys. Those attachments will need to be reuploaded to complete the operation")]
    AttachmentsWithoutKeys,
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum CipherType {
    Login = 1,
    SecureNote = 2,
    Card = 3,
    Identity = 4,
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum CipherRepromptType {
    None = 0,
    Password = 1,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CipherData {
    pub name: EncString,
    pub notes: Option<EncString>,

    pub r#type: CipherType,
    pub login: Option<login::Login>,
    pub identity: Option<identity::Identity>,
    pub card: Option<card::Card>,
    pub secure_note: Option<secure_note::SecureNote>,

    pub favorite: bool,
    pub reprompt: CipherRepromptType,
    pub organization_use_totp: bool,
    pub edit: bool,
    pub view_password: bool,

    pub attachments: Option<Vec<attachment::Attachment>>,
    pub fields: Option<Vec<field::Field>>,
    pub password_history: Option<Vec<password_history::PasswordHistory>>,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}

impl From<bitwarden_api_api::models::CipherType> for CipherType {
    fn from(t: bitwarden_api_api::models::CipherType) -> Self {
        match t {
            bitwarden_api_api::models::CipherType::Login => CipherType::Login,
            bitwarden_api_api::models::CipherType::SecureNote => CipherType::SecureNote,
            bitwarden_api_api::models::CipherType::Card => CipherType::Card,
            bitwarden_api_api::models::CipherType::Identity => CipherType::Identity,
        }
    }
}

impl From<bitwarden_api_api::models::CipherRepromptType> for CipherRepromptType {
    fn from(t: bitwarden_api_api::models::CipherRepromptType) -> Self {
        match t {
            bitwarden_api_api::models::CipherRepromptType::None => CipherRepromptType::None,
            bitwarden_api_api::models::CipherRepromptType::Password => CipherRepromptType::Password,
        }
    }
}

#[cfg(test)]
mod tests {}
