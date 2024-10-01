use bitwarden_core::require;
use bitwarden_crypto::{
    CryptoError, EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{linked_id::LinkedIdType, versioning::migrated};
use crate::VaultParseError;

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum FieldType {
    Text = 0,
    Hidden = 1,
    Boolean = 2,
    Linked = 3,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Field {
    name: Option<EncString>,
    value: Option<EncString>,
    r#type: FieldType,

    linked_id: Option<LinkedIdType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct FieldView {
    pub name: Option<String>,
    pub value: Option<String>,
    pub r#type: FieldType,

    pub linked_id: Option<LinkedIdType>,
}

impl KeyEncryptable<SymmetricCryptoKey, Field> for FieldView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Field, CryptoError> {
        Ok(Field {
            name: self.name.encrypt_with_key(key)?,
            value: self.value.encrypt_with_key(key)?,
            r#type: self.r#type,
            linked_id: self.linked_id,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, FieldView> for Field {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<FieldView, CryptoError> {
        Ok(FieldView {
            name: self.name.decrypt_with_key(key).ok().flatten(),
            value: self.value.decrypt_with_key(key).ok().flatten(),
            r#type: self.r#type,
            linked_id: self.linked_id,
        })
    }
}

impl TryFrom<migrated::CipherFieldModel> for Field {
    type Error = VaultParseError;

    fn try_from(model: migrated::CipherFieldModel) -> Result<Self, Self::Error> {
        Ok(Self {
            name: EncString::try_from_optional(model.name)?,
            value: EncString::try_from_optional(model.value)?,
            r#type: require!(model.r#type).into(),
            linked_id: model
                .linked_id
                .map(|id| (id as u32).try_into())
                .transpose()?,
        })
    }
}

impl From<migrated::FieldType> for FieldType {
    fn from(model: migrated::FieldType) -> Self {
        match model {
            migrated::FieldType::Text => FieldType::Text,
            migrated::FieldType::Hidden => FieldType::Hidden,
            migrated::FieldType::Boolean => FieldType::Boolean,
            migrated::FieldType::Linked => FieldType::Linked,
        }
    }
}
