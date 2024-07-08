use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};
use bitwarden_api_api::models::{SendFileModel, SendResponseModel, SendTextModel};
use bitwarden_core::require;
use bitwarden_crypto::{
    derive_shareable_key, generate_random_bytes, CryptoError, EncString, KeyDecryptable,
    KeyEncryptable, SymmetricCryptoKey,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::SendParseError;

const SEND_ITERATIONS: u32 = 100_000;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SendFile {
    pub id: Option<String>,
    pub file_name: EncString,
    pub size: Option<String>,
    /// Readable size, ex: "4.2 KB" or "1.43 GB"
    pub size_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SendFileView {
    pub id: Option<String>,
    pub file_name: String,
    pub size: Option<String>,
    /// Readable size, ex: "4.2 KB" or "1.43 GB"
    pub size_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SendText {
    pub text: Option<EncString>,
    pub hidden: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SendTextView {
    pub text: Option<String>,
    pub hidden: bool,
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema, PartialEq)]
#[repr(u8)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum SendType {
    Text = 0,
    File = 1,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Send {
    pub id: Option<Uuid>,
    pub access_id: Option<String>,

    pub name: EncString,
    pub notes: Option<EncString>,
    pub key: EncString,
    pub password: Option<String>,

    pub r#type: SendType,
    pub file: Option<SendFile>,
    pub text: Option<SendText>,

    pub max_access_count: Option<u32>,
    pub access_count: u32,
    pub disabled: bool,
    pub hide_email: bool,

    pub revision_date: DateTime<Utc>,
    pub deletion_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SendView {
    pub id: Option<Uuid>,
    pub access_id: Option<String>,

    pub name: String,
    pub notes: Option<String>,
    /// Base64 encoded key
    pub key: Option<String>,
    /// Replace or add a password to an existing send. The SDK will always return None when
    /// decrypting a [Send]
    /// TODO: We should revisit this, one variant is to have `[Create, Update]SendView` DTOs.
    pub new_password: Option<String>,
    /// Denote if an existing send has a password. The SDK will ignore this value when creating or
    /// updating sends.
    pub has_password: bool,

    pub r#type: SendType,
    pub file: Option<SendFileView>,
    pub text: Option<SendTextView>,

    pub max_access_count: Option<u32>,
    pub access_count: u32,
    pub disabled: bool,
    pub hide_email: bool,

    pub revision_date: DateTime<Utc>,
    pub deletion_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SendListView {
    pub id: Option<Uuid>,
    pub access_id: Option<String>,

    pub name: String,

    pub r#type: SendType,
    pub disabled: bool,

    pub revision_date: DateTime<Utc>,
    pub deletion_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
}

impl Send {
    pub fn get_key(
        send_key: &EncString,
        enc_key: &SymmetricCryptoKey,
    ) -> Result<SymmetricCryptoKey, CryptoError> {
        let key: Vec<u8> = send_key.decrypt_with_key(enc_key)?;
        Self::derive_shareable_key(&key)
    }

    fn derive_shareable_key(key: &[u8]) -> Result<SymmetricCryptoKey, CryptoError> {
        let key = Zeroizing::new(key.try_into().map_err(|_| CryptoError::InvalidKeyLen)?);
        Ok(derive_shareable_key(key, "send", Some("send")))
    }
}

impl KeyDecryptable<SymmetricCryptoKey, SendTextView> for SendText {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<SendTextView, CryptoError> {
        Ok(SendTextView {
            text: self.text.decrypt_with_key(key)?,
            hidden: self.hidden,
        })
    }
}

impl KeyEncryptable<SymmetricCryptoKey, SendText> for SendTextView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<SendText, CryptoError> {
        Ok(SendText {
            text: self.text.encrypt_with_key(key)?,
            hidden: self.hidden,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, SendFileView> for SendFile {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<SendFileView, CryptoError> {
        Ok(SendFileView {
            id: self.id.clone(),
            file_name: self.file_name.decrypt_with_key(key)?,
            size: self.size.clone(),
            size_name: self.size_name.clone(),
        })
    }
}

impl KeyEncryptable<SymmetricCryptoKey, SendFile> for SendFileView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<SendFile, CryptoError> {
        Ok(SendFile {
            id: self.id.clone(),
            file_name: self.file_name.encrypt_with_key(key)?,
            size: self.size.clone(),
            size_name: self.size_name.clone(),
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, SendView> for Send {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<SendView, CryptoError> {
        // For sends, we first decrypt the send key with the user key, and stretch it to it's full
        // size For the rest of the fields, we ignore the provided SymmetricCryptoKey and
        // the stretched key
        let k: Vec<u8> = self.key.decrypt_with_key(key)?;
        let key = Send::derive_shareable_key(&k)?;

        Ok(SendView {
            id: self.id,
            access_id: self.access_id.clone(),

            name: self.name.decrypt_with_key(&key).ok().unwrap_or_default(),
            notes: self.notes.decrypt_with_key(&key).ok().flatten(),
            key: Some(URL_SAFE_NO_PAD.encode(k)),
            new_password: None,
            has_password: self.password.is_some(),

            r#type: self.r#type,
            file: self.file.decrypt_with_key(&key).ok().flatten(),
            text: self.text.decrypt_with_key(&key).ok().flatten(),

            max_access_count: self.max_access_count,
            access_count: self.access_count,
            disabled: self.disabled,
            hide_email: self.hide_email,

            revision_date: self.revision_date,
            deletion_date: self.deletion_date,
            expiration_date: self.expiration_date,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, SendListView> for Send {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<SendListView, CryptoError> {
        // For sends, we first decrypt the send key with the user key, and stretch it to it's full
        // size For the rest of the fields, we ignore the provided SymmetricCryptoKey and
        // the stretched key
        let key = Send::get_key(&self.key, key)?;

        Ok(SendListView {
            id: self.id,
            access_id: self.access_id.clone(),

            name: self.name.decrypt_with_key(&key)?,
            r#type: self.r#type,

            disabled: self.disabled,

            revision_date: self.revision_date,
            deletion_date: self.deletion_date,
            expiration_date: self.expiration_date,
        })
    }
}

impl KeyEncryptable<SymmetricCryptoKey, Send> for SendView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Send, CryptoError> {
        // For sends, we first decrypt the send key with the user key, and stretch it to it's full
        // size For the rest of the fields, we ignore the provided SymmetricCryptoKey and
        // the stretched key
        let k = match (self.key, self.id) {
            // Existing send, decrypt key
            (Some(k), _) => URL_SAFE_NO_PAD
                .decode(k)
                .map_err(|_| CryptoError::InvalidKey)?,
            // New send, generate random key
            (None, None) => {
                let key = generate_random_bytes::<[u8; 16]>();
                key.to_vec()
            }
            // Existing send without key
            _ => return Err(CryptoError::InvalidKey),
        };
        let send_key = Send::derive_shareable_key(&k)?;

        Ok(Send {
            id: self.id,
            access_id: self.access_id,

            name: self.name.encrypt_with_key(&send_key)?,
            notes: self.notes.encrypt_with_key(&send_key)?,
            key: k.encrypt_with_key(key)?,
            password: self.new_password.map(|password| {
                let password = bitwarden_crypto::pbkdf2(password.as_bytes(), &k, SEND_ITERATIONS);
                STANDARD.encode(password)
            }),

            r#type: self.r#type,
            file: self.file.encrypt_with_key(&send_key)?,
            text: self.text.encrypt_with_key(&send_key)?,

            max_access_count: self.max_access_count,
            access_count: self.access_count,
            disabled: self.disabled,
            hide_email: self.hide_email,

            revision_date: self.revision_date,
            deletion_date: self.deletion_date,
            expiration_date: self.expiration_date,
        })
    }
}

impl TryFrom<SendResponseModel> for Send {
    type Error = SendParseError;

    fn try_from(send: SendResponseModel) -> Result<Self, Self::Error> {
        Ok(Send {
            id: send.id,
            access_id: send.access_id,
            name: require!(send.name).parse()?,
            notes: EncString::try_from_optional(send.notes)?,
            key: require!(send.key).parse()?,
            password: send.password,
            r#type: require!(send.r#type).into(),
            file: send.file.map(|f| (*f).try_into()).transpose()?,
            text: send.text.map(|t| (*t).try_into()).transpose()?,
            max_access_count: send.max_access_count.map(|s| s as u32),
            access_count: require!(send.access_count) as u32,
            disabled: send.disabled.unwrap_or(false),
            hide_email: send.hide_email.unwrap_or(false),
            revision_date: require!(send.revision_date).parse()?,
            deletion_date: require!(send.deletion_date).parse()?,
            expiration_date: send.expiration_date.map(|s| s.parse()).transpose()?,
        })
    }
}

impl From<bitwarden_api_api::models::SendType> for SendType {
    fn from(t: bitwarden_api_api::models::SendType) -> Self {
        match t {
            bitwarden_api_api::models::SendType::Text => SendType::Text,
            bitwarden_api_api::models::SendType::File => SendType::File,
        }
    }
}

impl TryFrom<SendFileModel> for SendFile {
    type Error = SendParseError;

    fn try_from(file: SendFileModel) -> Result<Self, Self::Error> {
        Ok(SendFile {
            id: file.id,
            file_name: require!(file.file_name).parse()?,
            size: file.size.map(|v| v.to_string()),
            size_name: file.size_name,
        })
    }
}

impl TryFrom<SendTextModel> for SendText {
    type Error = SendParseError;

    fn try_from(text: SendTextModel) -> Result<Self, Self::Error> {
        Ok(SendText {
            text: EncString::try_from_optional(text.text)?,
            hidden: text.hidden.unwrap_or(false),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bitwarden_crypto::{Kdf, KeyContainer, KeyDecryptable, KeyEncryptable, MasterKey};

    use super::*;

    struct MockKeyContainer(HashMap<Option<Uuid>, SymmetricCryptoKey>);
    impl MockKeyContainer {
        fn new(master_key: MasterKey, user_key: EncString) -> Result<Self, CryptoError> {
            let user_key = master_key.decrypt_user_key(user_key)?;
            Ok(Self(HashMap::from([(None, user_key)])))
        }
    }
    impl KeyContainer for MockKeyContainer {
        fn get_key<'a>(&'a self, org_id: &Option<Uuid>) -> Option<&'a SymmetricCryptoKey> {
            self.0.get(org_id)
        }
    }

    #[test]
    fn test_get_send_key() {
        // Initialize user encryption with some test data
        let master_key = MasterKey::derive(
            "asdfasdfasdf",
            "test@bitwarden.com",
            &Kdf::PBKDF2 {
                iterations: 345123.try_into().unwrap(),
            },
        )
        .unwrap();
        let enc = MockKeyContainer::new(
            master_key,
            "2.majkL1/hNz9yptLqNAUSnw==|RiOzMTTJMG948qu8O3Zm1EQUO2E8BuTwFKnO9LWQjMzxMWJM5GbyOq2/A+tumPbTERt4JWur/FKfgHb+gXuYiEYlXPMuVBvT7nv4LPytJuM=|IVqMxHJeR1ZXY0sGngTC0x+WqbG8p6V+BTrdgBbQXjM=".parse().unwrap(),
        ).unwrap();

        let k = enc.get_key(&None).unwrap();

        let send_key = "2.+1KUfOX8A83Xkwk1bumo/w==|Nczvv+DTkeP466cP/wMDnGK6W9zEIg5iHLhcuQG6s+M=|SZGsfuIAIaGZ7/kzygaVUau3LeOvJUlolENBOU+LX7g="
            .parse()
            .unwrap();

        // Get the send key
        let send_key = Send::get_key(&send_key, k).unwrap();
        let send_key_b64 = send_key.to_base64();
        assert_eq!(send_key_b64, "IR9ImHGm6rRuIjiN7csj94bcZR5WYTJj5GtNfx33zm6tJCHUl+QZlpNPba8g2yn70KnOHsAODLcR0um6E3MAlg==");
    }

    fn build_encryption_settings() -> MockKeyContainer {
        let master_key = MasterKey::derive(
            "asdfasdfasdf",
            "test@bitwarden.com",
            &Kdf::PBKDF2 {
                iterations: 600_000.try_into().unwrap(),
            },
        )
        .unwrap();

        MockKeyContainer::new(
            master_key,
            "2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=".parse().unwrap(),
        ).unwrap()
    }

    #[test]
    pub fn test_decrypt() {
        let enc = build_encryption_settings();
        let key = enc.get_key(&None).unwrap();

        let send = Send {
            id: "3d80dd72-2d14-4f26-812c-b0f0018aa144".parse().ok(),
            access_id: Some("ct2APRQtJk-BLLDwAYqhRA".to_owned()),
            r#type: SendType::Text,
            name: "2.STIyTrfDZN/JXNDN9zNEMw==|NDLum8BHZpPNYhJo9ggSkg==|UCsCLlBO3QzdPwvMAWs2VVwuE6xwOx/vxOooPObqnEw=".parse()
                .unwrap(),
            notes: None,
            file: None,
            text: Some(SendText {
                text: "2.2VPyLzk1tMLug0X3x7RkaQ==|mrMt9vbZsCJhJIj4eebKyg==|aZ7JeyndytEMR1+uEBupEvaZuUE69D/ejhfdJL8oKq0=".parse().ok(),
                hidden: false,
            }),
            key: "2.KLv/j0V4Ebs0dwyPdtt4vw==|jcrFuNYN1Qb3onBlwvtxUV/KpdnR1LPRL4EsCoXNAt4=|gHSywGy4Rj/RsCIZFwze4s2AACYKBtqDXTrQXjkgtIE=".parse().unwrap(),
            max_access_count: None,
            access_count: 0,
            password: None,
            disabled: false,
            revision_date: "2024-01-07T23:56:48.207363Z".parse().unwrap(),
            expiration_date: None,
            deletion_date: "2024-01-14T23:56:48Z".parse().unwrap(),
            hide_email: false,
        };

        let view: SendView = send.decrypt_with_key(key).unwrap();

        let expected = SendView {
            id: "3d80dd72-2d14-4f26-812c-b0f0018aa144".parse().ok(),
            access_id: Some("ct2APRQtJk-BLLDwAYqhRA".to_owned()),
            name: "Test".to_string(),
            notes: None,
            key: Some("Pgui0FK85cNhBGWHAlBHBw".to_owned()),
            new_password: None,
            has_password: false,
            r#type: SendType::Text,
            file: None,
            text: Some(SendTextView {
                text: Some("This is a test".to_owned()),
                hidden: false,
            }),
            max_access_count: None,
            access_count: 0,
            disabled: false,
            hide_email: false,
            revision_date: "2024-01-07T23:56:48.207363Z".parse().unwrap(),
            deletion_date: "2024-01-14T23:56:48Z".parse().unwrap(),
            expiration_date: None,
        };

        assert_eq!(view, expected);
    }

    #[test]
    pub fn test_encrypt() {
        let enc = build_encryption_settings();
        let key = enc.get_key(&None).unwrap();

        let view = SendView {
            id: "3d80dd72-2d14-4f26-812c-b0f0018aa144".parse().ok(),
            access_id: Some("ct2APRQtJk-BLLDwAYqhRA".to_owned()),
            name: "Test".to_string(),
            notes: None,
            key: Some("Pgui0FK85cNhBGWHAlBHBw".to_owned()),
            new_password: None,
            has_password: false,
            r#type: SendType::Text,
            file: None,
            text: Some(SendTextView {
                text: Some("This is a test".to_owned()),
                hidden: false,
            }),
            max_access_count: None,
            access_count: 0,
            disabled: false,
            hide_email: false,
            revision_date: "2024-01-07T23:56:48.207363Z".parse().unwrap(),
            deletion_date: "2024-01-14T23:56:48Z".parse().unwrap(),
            expiration_date: None,
        };

        // Re-encrypt and decrypt again to ensure encrypt works
        let v: SendView = view
            .clone()
            .encrypt_with_key(key)
            .unwrap()
            .decrypt_with_key(key)
            .unwrap();
        assert_eq!(v, view);
    }

    #[test]
    pub fn test_create() {
        let enc = build_encryption_settings();
        let key = enc.get_key(&None).unwrap();

        let view = SendView {
            id: None,
            access_id: Some("ct2APRQtJk-BLLDwAYqhRA".to_owned()),
            name: "Test".to_string(),
            notes: None,
            key: None,
            new_password: None,
            has_password: false,
            r#type: SendType::Text,
            file: None,
            text: Some(SendTextView {
                text: Some("This is a test".to_owned()),
                hidden: false,
            }),
            max_access_count: None,
            access_count: 0,
            disabled: false,
            hide_email: false,
            revision_date: "2024-01-07T23:56:48.207363Z".parse().unwrap(),
            deletion_date: "2024-01-14T23:56:48Z".parse().unwrap(),
            expiration_date: None,
        };

        // Re-encrypt and decrypt again to ensure encrypt works
        let v: SendView = view
            .clone()
            .encrypt_with_key(key)
            .unwrap()
            .decrypt_with_key(key)
            .unwrap();

        // Ignore key when comparing
        let t = SendView { key: None, ..v };
        assert_eq!(t, view);
    }

    #[test]
    pub fn test_create_password() {
        let enc = build_encryption_settings();
        let key = enc.get_key(&None).unwrap();

        let view = SendView {
            id: None,
            access_id: Some("ct2APRQtJk-BLLDwAYqhRA".to_owned()),
            name: "Test".to_owned(),
            notes: None,
            key: Some("Pgui0FK85cNhBGWHAlBHBw".to_owned()),
            new_password: Some("abc123".to_owned()),
            has_password: false,
            r#type: SendType::Text,
            file: None,
            text: Some(SendTextView {
                text: Some("This is a test".to_owned()),
                hidden: false,
            }),
            max_access_count: None,
            access_count: 0,
            disabled: false,
            hide_email: false,
            revision_date: "2024-01-07T23:56:48.207363Z".parse().unwrap(),
            deletion_date: "2024-01-14T23:56:48Z".parse().unwrap(),
            expiration_date: None,
        };

        let send: Send = view.encrypt_with_key(key).unwrap();

        assert_eq!(
            send.password,
            Some("vTIDfdj3FTDbejmMf+mJWpYdMXsxfeSd1Sma3sjCtiQ=".to_owned())
        );

        let v: SendView = send.decrypt_with_key(key).unwrap();
        assert_eq!(v.new_password, None);
        assert!(v.has_password);
    }
}
