use crate::UniffiCustomTypeConverter;

use super::VersionedCipherData;

uniffi::custom_type!(VersionedCipherData, String);

impl UniffiCustomTypeConverter for VersionedCipherData {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(serde_json::from_str(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        // TODO: Fix unwrap?
        serde_json::to_string(&obj).unwrap()
    }
}
