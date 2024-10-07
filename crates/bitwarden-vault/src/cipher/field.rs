use bitwarden_api_api::models::CipherFieldModel;
use bitwarden_core::{
    key_management::{AsymmetricKeyRef, SymmetricKeyRef},
    require,
};
use bitwarden_crypto::{
    service::CryptoServiceContext, CryptoError, Decryptable, EncString, Encryptable,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::linked_id::LinkedIdType;
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

impl Encryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, Field> for FieldView {
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<Field, CryptoError> {
        Ok(Field {
            name: self.name.encrypt(ctx, key)?,
            value: self.value.encrypt(ctx, key)?,
            r#type: self.r#type,
            linked_id: self.linked_id,
        })
    }
}

impl Decryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, FieldView> for Field {
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<FieldView, CryptoError> {
        Ok(FieldView {
            name: self.name.decrypt(ctx, key).ok().flatten(),
            value: self.value.decrypt(ctx, key).ok().flatten(),
            r#type: self.r#type,
            linked_id: self.linked_id,
        })
    }
}

impl TryFrom<CipherFieldModel> for Field {
    type Error = VaultParseError;

    fn try_from(model: CipherFieldModel) -> Result<Self, Self::Error> {
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

impl From<bitwarden_api_api::models::FieldType> for FieldType {
    fn from(model: bitwarden_api_api::models::FieldType) -> Self {
        match model {
            bitwarden_api_api::models::FieldType::Text => FieldType::Text,
            bitwarden_api_api::models::FieldType::Hidden => FieldType::Hidden,
            bitwarden_api_api::models::FieldType::Boolean => FieldType::Boolean,
            bitwarden_api_api::models::FieldType::Linked => FieldType::Linked,
        }
    }
}
