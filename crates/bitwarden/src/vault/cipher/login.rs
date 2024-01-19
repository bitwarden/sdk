use bitwarden_api_api::models::{CipherLoginModel, CipherLoginUriModel};
use bitwarden_crypto::{
    CryptoError, EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::error::{Error, Result};

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

impl KeyEncryptable<SymmetricCryptoKey, LoginUri> for LoginUriView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<LoginUri, CryptoError> {
        Ok(LoginUri {
            uri: self.uri.encrypt_with_key(key)?,
            r#match: self.r#match,
        })
    }
}

impl KeyEncryptable<SymmetricCryptoKey, Login> for LoginView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Login, CryptoError> {
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

impl KeyDecryptable<SymmetricCryptoKey, LoginUriView> for LoginUri {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<LoginUriView, CryptoError> {
        Ok(LoginUriView {
            uri: self.uri.decrypt_with_key(key)?,
            r#match: self.r#match,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, LoginView> for Login {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<LoginView, CryptoError> {
        Ok(LoginView {
            username: self.username.decrypt_with_key(key).ok().flatten(),
            password: self.password.decrypt_with_key(key).ok().flatten(),
            password_revision_date: self.password_revision_date,
            uris: self.uris.decrypt_with_key(key).ok().flatten(),
            totp: self.totp.decrypt_with_key(key).ok().flatten(),
            autofill_on_page_load: self.autofill_on_page_load,
        })
    }
}

impl TryFrom<CipherLoginModel> for Login {
    type Error = Error;

    fn try_from(login: CipherLoginModel) -> Result<Self> {
        Ok(Self {
            username: EncString::try_from_optional(login.username)?,
            password: EncString::try_from_optional(login.password)?,
            password_revision_date: login
                .password_revision_date
                .map(|d| d.parse())
                .transpose()?,
            uris: login
                .uris
                .map(|v| v.into_iter().map(|u| u.try_into()).collect())
                .transpose()?,
            totp: EncString::try_from_optional(login.totp)?,
            autofill_on_page_load: login.autofill_on_page_load,
        })
    }
}

impl TryFrom<CipherLoginUriModel> for LoginUri {
    type Error = Error;

    fn try_from(uri: CipherLoginUriModel) -> Result<Self> {
        Ok(Self {
            uri: EncString::try_from_optional(uri.uri)?,
            r#match: uri.r#match.map(|m| m.into()),
        })
    }
}

impl From<bitwarden_api_api::models::UriMatchType> for UriMatchType {
    fn from(value: bitwarden_api_api::models::UriMatchType) -> Self {
        match value {
            bitwarden_api_api::models::UriMatchType::Variant0 => Self::Domain,
            bitwarden_api_api::models::UriMatchType::Variant1 => Self::Host,
            bitwarden_api_api::models::UriMatchType::Variant2 => Self::StartsWith,
            bitwarden_api_api::models::UriMatchType::Variant3 => Self::Exact,
            bitwarden_api_api::models::UriMatchType::Variant4 => Self::RegularExpression,
            bitwarden_api_api::models::UriMatchType::Variant5 => Self::Never,
        }
    }
}
