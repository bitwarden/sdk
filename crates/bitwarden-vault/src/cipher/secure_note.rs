use bitwarden_core::require;
use bitwarden_crypto::{CryptoError, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::VaultParseError;

use super::versioning::migrated;

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum SecureNoteType {
    Generic = 0,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SecureNote {
    r#type: SecureNoteType,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SecureNoteView {
    pub r#type: SecureNoteType,
}

impl KeyEncryptable<SymmetricCryptoKey, SecureNote> for SecureNoteView {
    fn encrypt_with_key(self, _key: &SymmetricCryptoKey) -> Result<SecureNote, CryptoError> {
        Ok(SecureNote {
            r#type: self.r#type,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, SecureNoteView> for SecureNote {
    fn decrypt_with_key(&self, _key: &SymmetricCryptoKey) -> Result<SecureNoteView, CryptoError> {
        Ok(SecureNoteView {
            r#type: self.r#type,
        })
    }
}

impl TryFrom<migrated::CipherSecureNoteModel> for SecureNote {
    type Error = VaultParseError;

    fn try_from(model: migrated::CipherSecureNoteModel) -> Result<Self, Self::Error> {
        Ok(Self {
            r#type: require!(model.r#type).into(),
        })
    }
}

impl From<migrated::SecureNoteType> for SecureNoteType {
    fn from(model: migrated::SecureNoteType) -> Self {
        match model {
            migrated::SecureNoteType::Generic => SecureNoteType::Generic,
        }
    }
}
