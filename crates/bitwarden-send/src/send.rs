use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};
use bitwarden_api_api::models::{SendFileModel, SendResponseModel, SendTextModel};
use bitwarden_core::{
    key_management::{AsymmetricKeyRef, SymmetricKeyRef},
    require,
};
use bitwarden_crypto::{
    generate_random_bytes, service::CryptoServiceContext, CryptoError, Decryptable, EncString,
    Encryptable, UsesKey,
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

const SEND_KEY: SymmetricKeyRef = SymmetricKeyRef::Local("send_key");

impl Send {
    pub fn decrypt_key(
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        send_key: &EncString,
        enc_key: SymmetricKeyRef,
    ) -> Result<SymmetricKeyRef, CryptoError> {
        let key: Vec<u8> = send_key.decrypt(ctx, enc_key)?;
        Self::derive_shareable_key(ctx, &key)
    }

    fn derive_shareable_key(
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: &[u8],
    ) -> Result<SymmetricKeyRef, CryptoError> {
        let key = Zeroizing::new(key.try_into().map_err(|_| CryptoError::InvalidKeyLen)?);
        ctx.derive_shareable_key(SEND_KEY, key, "send", Some("send"))
    }
}

impl UsesKey<SymmetricKeyRef> for Send {
    fn uses_key(&self) -> SymmetricKeyRef {
        SymmetricKeyRef::User
    }
}

impl UsesKey<SymmetricKeyRef> for SendView {
    fn uses_key(&self) -> SymmetricKeyRef {
        SymmetricKeyRef::User
    }
}

impl Decryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, SendTextView> for SendText {
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<SendTextView, CryptoError> {
        Ok(SendTextView {
            text: self.text.decrypt(ctx, key)?,
            hidden: self.hidden,
        })
    }
}

impl Encryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, SendText> for SendTextView {
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<SendText, CryptoError> {
        Ok(SendText {
            text: self.text.encrypt(ctx, key)?,
            hidden: self.hidden,
        })
    }
}

impl Decryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, SendFileView> for SendFile {
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<SendFileView, CryptoError> {
        Ok(SendFileView {
            id: self.id.clone(),
            file_name: self.file_name.decrypt(ctx, key)?,
            size: self.size.clone(),
            size_name: self.size_name.clone(),
        })
    }
}

impl Encryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, SendFile> for SendFileView {
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<SendFile, CryptoError> {
        Ok(SendFile {
            id: self.id.clone(),
            file_name: self.file_name.encrypt(ctx, key)?,
            size: self.size.clone(),
            size_name: self.size_name.clone(),
        })
    }
}

impl Decryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, SendView> for Send {
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<SendView, CryptoError> {
        // For sends, we first decrypt the send key with the user key, and stretch it to it's full
        // size For the rest of the fields, we ignore the provided SymmetricCryptoKey and
        // the stretched key

        let k: Vec<u8> = self.key.decrypt(ctx, key)?;
        let key = Send::derive_shareable_key(ctx, &k)?;

        Ok(SendView {
            id: self.id,
            access_id: self.access_id.clone(),

            name: self.name.decrypt(ctx, key).ok().unwrap_or_default(),
            notes: self.notes.decrypt(ctx, key).ok().flatten(),
            key: Some(URL_SAFE_NO_PAD.encode(k)),
            new_password: None,
            has_password: self.password.is_some(),

            r#type: self.r#type,
            file: self.file.decrypt(ctx, key).ok().flatten(),
            text: self.text.decrypt(ctx, key).ok().flatten(),

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

impl Decryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, SendListView> for Send {
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<SendListView, CryptoError> {
        // For sends, we first decrypt the send key with the user key, and stretch it to it's full
        // size For the rest of the fields, we ignore the provided SymmetricCryptoKey and
        // the stretched key

        let k: Vec<u8> = self.key.decrypt(ctx, key)?;
        let key = Send::derive_shareable_key(ctx, &k)?;

        Ok(SendListView {
            id: self.id,
            access_id: self.access_id.clone(),

            name: self.name.decrypt(ctx, key).ok().unwrap_or_default(),
            r#type: self.r#type,

            disabled: self.disabled,

            revision_date: self.revision_date,
            deletion_date: self.deletion_date,
            expiration_date: self.expiration_date,
        })
    }
}

impl Encryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, Send> for SendView {
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<Send, CryptoError> {
        // For sends, we first decrypt the send key with the user key, and stretch it to it's full
        // size For the rest of the fields, we ignore the provided SymmetricCryptoKey and
        // the stretched key
        let k = match (&self.key, &self.id) {
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
        let send_key = Send::derive_shareable_key(ctx, &k)?;

        Ok(Send {
            id: self.id,
            access_id: self.access_id.clone(),

            name: self.name.encrypt(ctx, send_key)?,
            notes: self.notes.encrypt(ctx, send_key)?,
            key: k.as_slice().encrypt(ctx, key)?,
            password: self.new_password.as_ref().map(|password| {
                let password = bitwarden_crypto::pbkdf2(password.as_bytes(), &k, SEND_ITERATIONS);
                STANDARD.encode(password)
            }),

            r#type: self.r#type,
            file: self.file.encrypt(ctx, send_key)?,
            text: self.text.encrypt(ctx, send_key)?,

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
    use bitwarden_core::key_management::create_test_crypto_with_user_key;
    use bitwarden_crypto::SymmetricCryptoKey;

    use super::*;

    #[test]
    fn test_get_send_key() {
        // Initialize user encryption with some test data
        let user_key: SymmetricCryptoKey = "w2LO+nwV4oxwswVYCxlOfRUseXfvU03VzvKQHrqeklPgiMZrspUe6sOBToCnDn9Ay0tuCBn8ykVVRb7PWhub2Q==".to_string().try_into().unwrap();
        let crypto = create_test_crypto_with_user_key(user_key);
        let mut ctx = crypto.context();

        let send_key = "2.+1KUfOX8A83Xkwk1bumo/w==|Nczvv+DTkeP466cP/wMDnGK6W9zEIg5iHLhcuQG6s+M=|SZGsfuIAIaGZ7/kzygaVUau3LeOvJUlolENBOU+LX7g="
            .parse()
            .unwrap();

        // Get the send key
        let send_key = Send::decrypt_key(&mut ctx, &send_key, SymmetricKeyRef::User).unwrap();

        #[allow(deprecated)]
        let send_key = ctx.dangerous_get_symmetric_key(send_key).unwrap();

        let send_key_b64 = send_key.to_base64();
        assert_eq!(send_key_b64, "IR9ImHGm6rRuIjiN7csj94bcZR5WYTJj5GtNfx33zm6tJCHUl+QZlpNPba8g2yn70KnOHsAODLcR0um6E3MAlg==");
    }

    #[test]
    pub fn test_decrypt() {
        let user_key: SymmetricCryptoKey = "bYCsk857hl8QJJtxyRK65tjUrbxKC4aDifJpsml+NIv4W9cVgFvi3qVD+yJTUU2T4UwNKWYtt9pqWf7Q+2WCCg==".to_string().try_into().unwrap();
        let crypto = create_test_crypto_with_user_key(user_key);

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

        let view: SendView = crypto.decrypt(&send).unwrap();

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
        let user_key: SymmetricCryptoKey = "bYCsk857hl8QJJtxyRK65tjUrbxKC4aDifJpsml+NIv4W9cVgFvi3qVD+yJTUU2T4UwNKWYtt9pqWf7Q+2WCCg==".to_string().try_into().unwrap();
        let crypto = create_test_crypto_with_user_key(user_key);

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
        let v: SendView = crypto
            .decrypt(&crypto.encrypt(view.clone()).unwrap())
            .unwrap();
        assert_eq!(v, view);
    }

    #[test]
    pub fn test_create() {
        let user_key: SymmetricCryptoKey = "bYCsk857hl8QJJtxyRK65tjUrbxKC4aDifJpsml+NIv4W9cVgFvi3qVD+yJTUU2T4UwNKWYtt9pqWf7Q+2WCCg==".to_string().try_into().unwrap();
        let crypto = create_test_crypto_with_user_key(user_key);

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
        let v: SendView = crypto
            .decrypt(&crypto.encrypt(view.clone()).unwrap())
            .unwrap();

        // Ignore key when comparing
        let t = SendView { key: None, ..v };
        assert_eq!(t, view);
    }

    #[test]
    pub fn test_create_password() {
        let user_key: SymmetricCryptoKey = "bYCsk857hl8QJJtxyRK65tjUrbxKC4aDifJpsml+NIv4W9cVgFvi3qVD+yJTUU2T4UwNKWYtt9pqWf7Q+2WCCg==".to_string().try_into().unwrap();
        let crypto = create_test_crypto_with_user_key(user_key);

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

        let send: Send = crypto.encrypt(view).unwrap();

        assert_eq!(
            send.password,
            Some("vTIDfdj3FTDbejmMf+mJWpYdMXsxfeSd1Sma3sjCtiQ=".to_owned())
        );

        let v: SendView = crypto.decrypt(&send).unwrap();
        assert_eq!(v.new_password, None);
        assert!(v.has_password);
    }
}
