use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{Decryptable, Encryptable},
    error::Result,
};

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
pub enum SecureNoteType {
    Generic = 0,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecureNote {
    r#type: SecureNoteType,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecureNoteView {
    r#type: SecureNoteType,
}

impl Encryptable<SecureNote> for SecureNoteView {
    fn encrypt(self, _enc: &EncryptionSettings, _org_id: &Option<Uuid>) -> Result<SecureNote> {
        Ok(SecureNote {
            r#type: self.r#type,
        })
    }
}

impl Decryptable<SecureNoteView> for SecureNote {
    fn decrypt(&self, _enc: &EncryptionSettings, _org_id: &Option<Uuid>) -> Result<SecureNoteView> {
        Ok(SecureNoteView {
            r#type: self.r#type,
        })
    }
}
