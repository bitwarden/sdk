use base64::engine::general_purpose::STANDARD;
use bitwarden_api_api::models::{CipherLoginModel, CipherLoginUriModel};
use bitwarden_crypto::{
    CryptoError, DecryptedString, DecryptedVec, EncString, KeyDecryptable, KeyEncryptable,
    Sensitive, SensitiveVec, SymmetricCryptoKey,
};
use chrono::{DateTime, Utc};
use hmac::digest::generic_array::GenericArray;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sha2::Digest;

use crate::error::{require, Error, Result};

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum UriMatchType {
    Domain = 0,
    Host = 1,
    StartsWith = 2,
    Exact = 3,
    RegularExpression = 4,
    Never = 5,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct LoginUri {
    pub uri: Option<EncString>,
    pub r#match: Option<UriMatchType>,
    pub uri_checksum: Option<EncString>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct LoginUriView {
    pub uri: Option<DecryptedString>,
    pub r#match: Option<UriMatchType>,
    pub uri_checksum: Option<DecryptedString>,
}

impl LoginUriView {
    pub(crate) fn is_checksum_valid(&self) -> bool {
        let Some(uri) = &self.uri else {
            return false;
        };
        let Some(cs) = &self.uri_checksum else {
            return false;
        };
        let Ok(cs) = cs.clone().decode_base64(STANDARD) else {
            return false;
        };

        let uri_hash: Sensitive<GenericArray<u8, _>> = Sensitive::new(Box::new(
            sha2::Sha256::new()
                .chain_update(uri.expose().as_bytes())
                .finalize(),
        ));

        cs == uri_hash.expose().as_slice()
    }

    pub(crate) fn generate_checksum(&mut self) {
        if let Some(uri) = &self.uri {
            let uri_hash: SensitiveVec = Sensitive::new(Box::new(
                sha2::Sha256::new()
                    .chain_update(uri.expose().as_bytes())
                    .finalize(),
            ))
            .into();
            self.uri_checksum = Some(uri_hash.encode_base64(STANDARD))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Fido2CredentialView {
    pub credential_id: DecryptedString,
    pub key_type: DecryptedString,
    pub key_algorithm: DecryptedString,
    pub key_curve: DecryptedString,
    // This value doesn't need to be returned to the client
    // so we keep it encrypted until we need it
    pub key_value: EncString,
    pub rp_id: DecryptedString,
    pub user_handle: Option<DecryptedVec>,
    pub user_name: Option<DecryptedString>,
    pub counter: DecryptedString,
    pub rp_name: Option<DecryptedString>,
    pub user_display_name: Option<DecryptedString>,
    pub discoverable: DecryptedString,
    pub creation_date: DateTime<Utc>,
}

// This is mostly a copy of the Fido2CredentialView, but with the key exposed
// Only meant to be used internally and not exposed to the outside world
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Fido2CredentialFullView {
    pub credential_id: DecryptedString,
    pub key_type: DecryptedString,
    pub key_algorithm: DecryptedString,
    pub key_curve: DecryptedString,
    pub key_value: DecryptedVec,
    pub rp_id: DecryptedString,
    pub user_handle: Option<DecryptedVec>,
    pub user_name: Option<DecryptedString>,
    pub counter: DecryptedString,
    pub rp_name: Option<DecryptedString>,
    pub user_display_name: Option<DecryptedString>,
    pub discoverable: DecryptedString,
    pub creation_date: DateTime<Utc>,
}

// This is mostly a copy of the Fido2CredentialView, meant to be exposed to the clients
// to let them select where to store the new credential. Note that it doesn't contain
// the encrypted key as that is only filled when the cipher is selected
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Fido2CredentialNewView {
    pub credential_id: DecryptedString,
    pub key_type: DecryptedString,
    pub key_algorithm: DecryptedString,
    pub key_curve: DecryptedString,
    pub rp_id: DecryptedString,
    pub user_handle: Option<DecryptedVec>,
    pub user_name: Option<DecryptedString>,
    pub counter: DecryptedString,
    pub rp_name: Option<DecryptedString>,
    pub user_display_name: Option<DecryptedString>,
    pub discoverable: DecryptedString,
    pub creation_date: DateTime<Utc>,
}

impl From<Fido2CredentialFullView> for Fido2CredentialNewView {
    fn from(value: Fido2CredentialFullView) -> Self {
        Fido2CredentialNewView {
            credential_id: value.credential_id,
            key_type: value.key_type,
            key_algorithm: value.key_algorithm,
            key_curve: value.key_curve,
            rp_id: value.rp_id,
            user_handle: value.user_handle,
            user_name: value.user_name,
            counter: value.counter,
            rp_name: value.rp_name,
            user_display_name: value.user_display_name,
            discoverable: value.discoverable,
            creation_date: value.creation_date,
        }
    }
}

impl KeyEncryptable<SymmetricCryptoKey, Fido2CredentialView> for Fido2CredentialFullView {
    fn encrypt_with_key(
        self,
        key: &SymmetricCryptoKey,
    ) -> Result<Fido2CredentialView, CryptoError> {
        Ok(Fido2CredentialView {
            credential_id: self.credential_id,
            key_type: self.key_type,
            key_algorithm: self.key_algorithm,
            key_curve: self.key_curve,
            key_value: self.key_value.expose().encrypt_with_key(key)?,
            rp_id: self.rp_id,
            user_handle: self.user_handle,
            user_name: self.user_name,
            counter: self.counter,
            rp_name: self.rp_name,
            user_display_name: self.user_display_name,
            discoverable: self.discoverable,
            creation_date: self.creation_date,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, Fido2CredentialFullView> for Fido2Credential {
    fn decrypt_with_key(
        &self,
        key: &SymmetricCryptoKey,
    ) -> Result<Fido2CredentialFullView, CryptoError> {
        Ok(Fido2CredentialFullView {
            credential_id: self.credential_id.decrypt_with_key(key)?,
            key_type: self.key_type.decrypt_with_key(key)?,
            key_algorithm: self.key_algorithm.decrypt_with_key(key)?,
            key_curve: self.key_curve.decrypt_with_key(key)?,
            key_value: self.key_value.decrypt_with_key(key)?,
            rp_id: self.rp_id.decrypt_with_key(key)?,
            user_handle: self.user_handle.decrypt_with_key(key)?,
            user_name: self.user_name.decrypt_with_key(key)?,
            counter: self.counter.decrypt_with_key(key)?,
            rp_name: self.rp_name.decrypt_with_key(key)?,
            user_display_name: self.user_display_name.decrypt_with_key(key)?,
            discoverable: self.discoverable.decrypt_with_key(key)?,
            creation_date: self.creation_date,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, Fido2CredentialFullView> for Fido2CredentialView {
    fn decrypt_with_key(
        &self,
        key: &SymmetricCryptoKey,
    ) -> Result<Fido2CredentialFullView, CryptoError> {
        Ok(Fido2CredentialFullView {
            credential_id: self.credential_id.clone(),
            key_type: self.key_type.clone(),
            key_algorithm: self.key_algorithm.clone(),
            key_curve: self.key_curve.clone(),
            key_value: self.key_value.decrypt_with_key(key)?,
            rp_id: self.rp_id.clone(),
            user_handle: self.user_handle.clone(),
            user_name: self.user_name.clone(),
            counter: self.counter.clone(),
            rp_name: self.rp_name.clone(),
            user_display_name: self.user_display_name.clone(),
            discoverable: self.discoverable.clone(),
            creation_date: self.creation_date,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Login {
    pub username: Option<EncString>,
    pub password: Option<EncString>,
    pub password_revision_date: Option<DateTime<Utc>>,

    pub uris: Option<Vec<LoginUri>>,
    pub totp: Option<EncString>,
    pub autofill_on_page_load: Option<bool>,

    pub fido2_credentials: Option<Vec<Fido2Credential>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct LoginView {
    pub username: Option<DecryptedString>,
    pub password: Option<DecryptedString>,
    pub password_revision_date: Option<DateTime<Utc>>,

    pub uris: Option<Vec<LoginUriView>>,
    pub totp: Option<DecryptedString>,
    pub autofill_on_page_load: Option<bool>,

    // TODO: Remove this once the SDK supports state
    pub fido2_credentials: Option<Vec<Fido2CredentialView>>,
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
            fido2_credentials: self.fido2_credentials.encrypt_with_key(key)?,
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
            fido2_credentials: self.fido2_credentials.decrypt_with_key(key).ok().flatten(),
        })
    }
}

impl KeyEncryptable<SymmetricCryptoKey, Fido2Credential> for Fido2CredentialView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Fido2Credential, CryptoError> {
        Ok(Fido2Credential {
            credential_id: self.credential_id.encrypt_with_key(key)?,
            key_type: self.key_type.encrypt_with_key(key)?,
            key_algorithm: self.key_algorithm.encrypt_with_key(key)?,
            key_curve: self.key_curve.encrypt_with_key(key)?,
            key_value: self.key_value,
            rp_id: self.rp_id.encrypt_with_key(key)?,
            user_handle: self
                .user_handle
                .map(|h| h.expose().encrypt_with_key(key))
                .transpose()?,
            user_name: self
                .user_name
                .map(|n| n.encrypt_with_key(key))
                .transpose()?,
            counter: self.counter.encrypt_with_key(key)?,
            rp_name: self.rp_name.encrypt_with_key(key)?,
            user_display_name: self.user_display_name.encrypt_with_key(key)?,
            discoverable: self.discoverable.encrypt_with_key(key)?,
            creation_date: self.creation_date,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, Fido2CredentialView> for Fido2Credential {
    fn decrypt_with_key(
        &self,
        key: &SymmetricCryptoKey,
    ) -> Result<Fido2CredentialView, CryptoError> {
        Ok(Fido2CredentialView {
            credential_id: self.credential_id.decrypt_with_key(key)?,
            key_type: self.key_type.decrypt_with_key(key)?,
            key_algorithm: self.key_algorithm.decrypt_with_key(key)?,
            key_curve: self.key_curve.decrypt_with_key(key)?,
            key_value: self.key_value.clone(),
            rp_id: self.rp_id.decrypt_with_key(key)?,
            user_handle: self.user_handle.decrypt_with_key(key)?,
            user_name: self.user_name.decrypt_with_key(key)?,
            counter: self.counter.decrypt_with_key(key)?,
            rp_name: self.rp_name.decrypt_with_key(key)?,
            user_display_name: self.user_display_name.decrypt_with_key(key)?,
            discoverable: self.discoverable.decrypt_with_key(key)?,
            creation_date: self.creation_date,
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
            credential_id: require!(value.credential_id).parse()?,
            key_type: require!(value.key_type).parse()?,
            key_algorithm: require!(value.key_algorithm).parse()?,
            key_curve: require!(value.key_curve).parse()?,
            key_value: require!(value.key_value).parse()?,
            rp_id: require!(value.rp_id).parse()?,
            user_handle: EncString::try_from_optional(value.user_handle)
                .ok()
                .flatten(),
            user_name: EncString::try_from_optional(value.user_name).ok().flatten(),
            counter: require!(value.counter).parse()?,
            rp_name: EncString::try_from_optional(value.rp_name).ok().flatten(),
            user_display_name: EncString::try_from_optional(value.user_display_name)
                .ok()
                .flatten(),
            discoverable: require!(value.discoverable).parse()?,
            creation_date: value.creation_date.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use bitwarden_crypto::SensitiveString;

    #[test]
    fn test_valid_checksum() {
        let uri = super::LoginUriView {
            uri: Some(SensitiveString::test("https://example.com")),
            r#match: Some(super::UriMatchType::Domain),
            uri_checksum: Some(SensitiveString::test(
                "EAaArVRs5qV39C9S3zO0z9ynVoWeZkuNfeMpsVDQnOk=",
            )),
        };
        assert!(uri.is_checksum_valid());
    }

    #[test]
    fn test_invalid_checksum() {
        let uri = super::LoginUriView {
            uri: Some(SensitiveString::test("https://example.com")),
            r#match: Some(super::UriMatchType::Domain),
            uri_checksum: Some(SensitiveString::test(
                "UtSgIv8LYfEdOu7yqjF7qXWhmouYGYC8RSr7/ryZg5Q=",
            )),
        };
        assert!(!uri.is_checksum_valid());
    }

    #[test]
    fn test_missing_checksum() {
        let uri = super::LoginUriView {
            uri: Some(SensitiveString::test("https://example.com")),
            r#match: Some(super::UriMatchType::Domain),
            uri_checksum: None,
        };
        assert!(!uri.is_checksum_valid());
    }

    #[test]
    fn test_generate_checksum() {
        let mut uri = super::LoginUriView {
            uri: Some(SensitiveString::test("https://test.com")),
            r#match: Some(super::UriMatchType::Domain),
            uri_checksum: None,
        };

        uri.generate_checksum();

        assert_eq!(
            uri.uri_checksum.unwrap().expose(),
            "OWk2vQvwYD1nhLZdA+ltrpBWbDa2JmHyjUEWxRZSS8w="
        );
    }
}
