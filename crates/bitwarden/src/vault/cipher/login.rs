use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{Decryptable, EncString, Encryptable},
    error::Result,
};

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
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
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct LoginUri {
    pub uri: Option<EncString>,
    pub r#match: Option<UriMatchType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct LoginUriView {
    pub uri: Option<String>,
    pub r#match: Option<UriMatchType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Login {
    pub username: Option<EncString>,
    pub password: Option<EncString>,
    pub password_revision_date: Option<DateTime<Utc>>,

    pub uris: Option<Vec<LoginUri>>,
    pub totp: Option<EncString>,
    pub autofill_on_page_load: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct LoginView {
    pub username: Option<String>,
    pub password: Option<String>,
    pub password_revision_date: Option<DateTime<Utc>>,

    pub uris: Option<Vec<LoginUriView>>,
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

impl Decryptable<LoginUriView> for LoginUri {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<LoginUriView> {
        Ok(LoginUriView {
            uri: self.uri.decrypt(enc, org_id)?,
            r#match: self.r#match,
        })
    }
}

impl Decryptable<LoginView> for Login {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<LoginView> {
        Ok(LoginView {
            username: self.username.decrypt(enc, org_id)?,
            password: self.password.decrypt(enc, org_id)?,
            password_revision_date: self.password_revision_date,
            uris: self.uris.decrypt(enc, org_id)?,
            totp: self.totp.decrypt(enc, org_id)?,
            autofill_on_page_load: self.autofill_on_page_load,
        })
    }
}
