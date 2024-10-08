use bitwarden_api_api::models::CipherSecureNoteModel;
use bitwarden_core::{
    key_management::{AsymmetricKeyRef, SymmetricKeyRef},
    require,
};
use bitwarden_crypto::{service::CryptoServiceContext, CryptoError, Decryptable, Encryptable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::VaultParseError;

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

impl Encryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, SecureNote>
    for SecureNoteView
{
    fn encrypt(
        &self,
        _ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        _key: SymmetricKeyRef,
    ) -> Result<SecureNote, CryptoError> {
        Ok(SecureNote {
            r#type: self.r#type,
        })
    }
}

impl Decryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, SecureNoteView>
    for SecureNote
{
    fn decrypt(
        &self,
        _ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        _key: SymmetricKeyRef,
    ) -> Result<SecureNoteView, CryptoError> {
        Ok(SecureNoteView {
            r#type: self.r#type,
        })
    }
}

impl TryFrom<CipherSecureNoteModel> for SecureNote {
    type Error = VaultParseError;

    fn try_from(model: CipherSecureNoteModel) -> Result<Self, Self::Error> {
        Ok(Self {
            r#type: require!(model.r#type).into(),
        })
    }
}

impl From<bitwarden_api_api::models::SecureNoteType> for SecureNoteType {
    fn from(model: bitwarden_api_api::models::SecureNoteType) -> Self {
        match model {
            bitwarden_api_api::models::SecureNoteType::Generic => SecureNoteType::Generic,
        }
    }
}
