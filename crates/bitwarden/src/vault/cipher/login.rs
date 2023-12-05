use bitwarden_crypto::symmetric_crypto_key::SymmetricCryptoKey;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    crypto::{EncString, KeyDecryptable, KeyEncryptable},
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

impl KeyEncryptable<LoginUri> for LoginUriView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<LoginUri> {
        Ok(LoginUri {
            uri: self.uri.encrypt_with_key(key)?,
            r#match: self.r#match,
        })
    }
}

impl KeyEncryptable<Login> for LoginView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Login> {
        Ok(Login {
            username: self.username.encrypt_with_key(key)?,
            password: self.password.encrypt_with_key(key)?,
            password_revision_date: self.password_revision_date,
            uris: self.uris.encrypt_with_key(key)?,
            totp: self.totp.encrypt_with_key(key)?,
            autofill_on_page_load: self.autofill_on_page_load,
        })
    }
}

impl KeyDecryptable<LoginUriView> for LoginUri {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<LoginUriView> {
        Ok(LoginUriView {
            uri: self.uri.decrypt_with_key(key)?,
            r#match: self.r#match,
        })
    }
}

impl KeyDecryptable<LoginView> for Login {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<LoginView> {
        Ok(LoginView {
            username: self.username.decrypt_with_key(key)?,
            password: self.password.decrypt_with_key(key)?,
            password_revision_date: self.password_revision_date,
            uris: self.uris.decrypt_with_key(key)?,
            totp: self.totp.decrypt_with_key(key)?,
            autofill_on_page_load: self.autofill_on_page_load,
        })
    }
}
