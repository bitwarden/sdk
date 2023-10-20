use bitwarden_api_api::models::CipherSecureNoteModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{Decryptable, Encryptable},
    error::{Error, Result},
};

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

impl TryFrom<CipherSecureNoteModel> for SecureNote {
    type Error = Error;

    fn try_from(model: CipherSecureNoteModel) -> Result<Self> {
        Ok(Self {
            r#type: model.r#type.map(|t| t.into()).ok_or(Error::MissingFields)?,
        })
    }
}

impl From<bitwarden_api_api::models::SecureNoteType> for SecureNoteType {
    fn from(model: bitwarden_api_api::models::SecureNoteType) -> Self {
        match model {
            bitwarden_api_api::models::SecureNoteType::Variant0 => SecureNoteType::Generic,
        }
    }
}
