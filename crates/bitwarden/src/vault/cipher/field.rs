use bitwarden_crypto::{EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::linked_id::LinkedIdType;

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum FieldType {
    Text = 0,
    Hidden = 1,
    Boolean = 2,
    Linked = 3,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Field {
    name: Option<EncString>,
    value: Option<EncString>,
    r#type: FieldType,

    linked_id: Option<LinkedIdType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct FieldView {
    name: Option<String>,
    value: Option<String>,
    r#type: FieldType,

    linked_id: Option<LinkedIdType>,
}

impl KeyEncryptable<Field> for FieldView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> bitwarden_crypto::Result<Field> {
        Ok(Field {
            name: self.name.encrypt_with_key(key)?,
            value: self.value.encrypt_with_key(key)?,
            r#type: self.r#type,
            linked_id: self.linked_id,
        })
    }
}

impl KeyDecryptable<FieldView> for Field {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> bitwarden_crypto::Result<FieldView> {
        Ok(FieldView {
            name: self.name.decrypt_with_key(key)?,
            value: self.value.decrypt_with_key(key)?,
            r#type: self.r#type,
            linked_id: self.linked_id,
        })
    }
}
