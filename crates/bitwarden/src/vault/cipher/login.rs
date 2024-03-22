use base64::{engine::general_purpose::STANDARD, Engine};
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
    pub uri_checksum: Option<EncString>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct LoginUriView {
    pub uri: Option<String>,
    pub r#match: Option<UriMatchType>,
    pub uri_checksum: Option<String>,
}

impl LoginUriView {
    pub(crate) fn is_checksum_valid(&self) -> bool {
        let Some(uri) = &self.uri else {
            return false;
        };
        let Some(cs) = &self.uri_checksum else {
            return false;
        };
        let Ok(cs) = STANDARD.decode(cs) else {
            return false;
        };

        use sha2::Digest;
        let uri_hash = sha2::Sha256::new().chain_update(uri.as_bytes()).finalize();

        uri_hash.as_slice() == cs
    }

    pub(crate) fn generate_checksum(&mut self) {
        if let Some(uri) = &self.uri {
            use sha2::Digest;
            let uri_hash = sha2::Sha256::new().chain_update(uri.as_bytes()).finalize();
            let uri_hash = STANDARD.encode(uri_hash.as_slice());
            self.uri_checksum = Some(uri_hash);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Fido2Credential {
    pub credential_id: EncString,
    pub key_type: EncString,
    pub key_algorithm: EncString,
    pub key_curve: EncString,
    pub key_value: EncString,
    pub rp_id: EncString,
    pub user_handle: Option<EncString>,
    pub user_name: Option<EncString>,
    pub counter: EncString,
    pub rp_name: Option<EncString>,
    pub user_display_name: Option<EncString>,
    pub discoverable: EncString,
    pub creation_date: DateTime<Utc>,
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

    pub fido2_credentials: Option<Vec<Fido2Credential>>,
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

    // TODO: Remove this once the SDK supports state
    pub fido2_credentials: Option<Vec<Fido2Credential>>,
}

impl KeyEncryptable<SymmetricCryptoKey, LoginUri> for LoginUriView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<LoginUri, CryptoError> {
        Ok(LoginUri {
            uri: self.uri.encrypt_with_key(key)?,
            r#match: self.r#match,
            uri_checksum: self.uri_checksum.encrypt_with_key(key)?,
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
            fido2_credentials: self.fido2_credentials,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, LoginUriView> for LoginUri {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<LoginUriView, CryptoError> {
        Ok(LoginUriView {
            uri: self.uri.decrypt_with_key(key)?,
            r#match: self.r#match,
            uri_checksum: self.uri_checksum.decrypt_with_key(key)?,
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
            fido2_credentials: self.fido2_credentials.clone(),
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
            fido2_credentials: login
                .fido2_credentials
                .map(|v| v.into_iter().map(|c| c.try_into()).collect())
                .transpose()?,
        })
    }
}

impl TryFrom<CipherLoginUriModel> for LoginUri {
    type Error = Error;

    fn try_from(uri: CipherLoginUriModel) -> Result<Self> {
        Ok(Self {
            uri: EncString::try_from_optional(uri.uri)?,
            r#match: uri.r#match.map(|m| m.into()),
            uri_checksum: EncString::try_from_optional(uri.uri_checksum)?,
        })
    }
}

impl From<bitwarden_api_api::models::UriMatchType> for UriMatchType {
    fn from(value: bitwarden_api_api::models::UriMatchType) -> Self {
        match value {
            bitwarden_api_api::models::UriMatchType::Domain => Self::Domain,
            bitwarden_api_api::models::UriMatchType::Host => Self::Host,
            bitwarden_api_api::models::UriMatchType::StartsWith => Self::StartsWith,
            bitwarden_api_api::models::UriMatchType::Exact => Self::Exact,
            bitwarden_api_api::models::UriMatchType::RegularExpression => Self::RegularExpression,
            bitwarden_api_api::models::UriMatchType::Never => Self::Never,
        }
    }
}

impl TryFrom<bitwarden_api_api::models::CipherFido2CredentialModel> for Fido2Credential {
    type Error = Error;

    fn try_from(value: bitwarden_api_api::models::CipherFido2CredentialModel) -> Result<Self> {
        Ok(Self {
            credential_id: value.credential_id.ok_or(Error::MissingFields)?.parse()?,
            key_type: value.key_type.ok_or(Error::MissingFields)?.parse()?,
            key_algorithm: value.key_algorithm.ok_or(Error::MissingFields)?.parse()?,
            key_curve: value.key_curve.ok_or(Error::MissingFields)?.parse()?,
            key_value: value.key_value.ok_or(Error::MissingFields)?.parse()?,
            rp_id: value.rp_id.ok_or(Error::MissingFields)?.parse()?,
            user_handle: EncString::try_from_optional(value.user_handle)
                .ok()
                .flatten(),
            user_name: EncString::try_from_optional(value.user_name).ok().flatten(),
            counter: value.counter.ok_or(Error::MissingFields)?.parse()?,
            rp_name: EncString::try_from_optional(value.rp_name).ok().flatten(),
            user_display_name: EncString::try_from_optional(value.user_display_name)
                .ok()
                .flatten(),
            discoverable: value.discoverable.ok_or(Error::MissingFields)?.parse()?,
            creation_date: value.creation_date.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_checksum() {
        let uri = super::LoginUriView {
            uri: Some("https://example.com".to_string()),
            r#match: Some(super::UriMatchType::Domain),
            uri_checksum: Some("EAaArVRs5qV39C9S3zO0z9ynVoWeZkuNfeMpsVDQnOk=".to_string()),
        };
        assert!(uri.is_checksum_valid());
    }

    #[test]
    fn test_invalid_checksum() {
        let uri = super::LoginUriView {
            uri: Some("https://example.com".to_string()),
            r#match: Some(super::UriMatchType::Domain),
            uri_checksum: Some("UtSgIv8LYfEdOu7yqjF7qXWhmouYGYC8RSr7/ryZg5Q=".to_string()),
        };
        assert!(!uri.is_checksum_valid());
    }

    #[test]
    fn test_missing_checksum() {
        let uri = super::LoginUriView {
            uri: Some("https://example.com".to_string()),
            r#match: Some(super::UriMatchType::Domain),
            uri_checksum: None,
        };
        assert!(!uri.is_checksum_valid());
    }

    #[test]
    fn test_generate_checksum() {
        let mut uri = super::LoginUriView {
            uri: Some("https://test.com".to_string()),
            r#match: Some(super::UriMatchType::Domain),
            uri_checksum: None,
        };

        uri.generate_checksum();

        assert_eq!(
            uri.uri_checksum.unwrap().as_str(),
            "OWk2vQvwYD1nhLZdA+ltrpBWbDa2JmHyjUEWxRZSS8w="
        );
    }
}
