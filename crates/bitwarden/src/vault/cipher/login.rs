use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{CipherString, Encryptable},
    error::Result,
};

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum UriMatchType {
    Domain = 0,
    Host = 1,
    StartsWith = 2,
    Exact = 3,
    RegularExpression = 4,
    Never = 5,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LoginUri {
    pub uri: CipherString,
    pub r#match: Option<UriMatchType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LoginUriView {
    pub uri: String,
    pub r#match: Option<UriMatchType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Login {
    pub username: CipherString,
    pub password: CipherString,
    pub password_revision_date: Option<DateTime<Utc>>,

    pub uris: Vec<LoginUri>,
    pub totp: Option<CipherString>,
    pub autofill_on_page_load: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LoginView {
    pub username: String,
    pub password: String,
    pub password_revision_date: Option<DateTime<Utc>>,

    pub uris: Vec<LoginUriView>,
    pub totp: Option<String>,
    pub autofill_on_page_load: Option<bool>,
}

impl Encryptable<LoginUri> for LoginUriView {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<LoginUri> {
        Ok(LoginUri {
            uri: self.uri.encrypt(enc, org_id)?,
            r#match: self.r#match,
        })
    }
}

impl Encryptable<Login> for LoginView {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Login> {
        Ok(Login {
            username: self.username.encrypt(enc, org_id)?,
            password: self.password.encrypt(enc, org_id)?,
            password_revision_date: self.password_revision_date,
            uris: self.uris.encrypt(enc, org_id)?,
            totp: self.totp.encrypt(enc, org_id)?,
            autofill_on_page_load: self.autofill_on_page_load,
        })
    }
}
