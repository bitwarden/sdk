use bitwarden_crypto::{KeyDecryptable, KeyEncryptable, SymmetricCryptoKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum SecureNoteType {
    Generic = 0,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SecureNote {
    r#type: SecureNoteType,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SecureNoteView {
    r#type: SecureNoteType,
}

impl KeyEncryptable<SecureNote> for SecureNoteView {
    fn encrypt_with_key(self, _key: &SymmetricCryptoKey) -> bitwarden_crypto::Result<SecureNote> {
        Ok(SecureNote {
            r#type: self.r#type,
        })
    }
}

impl KeyDecryptable<SecureNoteView> for SecureNote {
    fn decrypt_with_key(
        &self,
        _key: &SymmetricCryptoKey,
    ) -> bitwarden_crypto::Result<SecureNoteView> {
        Ok(SecureNoteView {
            r#type: self.r#type,
        })
    }
}
