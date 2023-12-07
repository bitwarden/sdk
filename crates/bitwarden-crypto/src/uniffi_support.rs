use crate::{CryptoError, EncString, UniffiCustomTypeConverter};

uniffi::custom_type!(EncString, String);

impl UniffiCustomTypeConverter for EncString {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        val.parse().map_err(|e: CryptoError| e.into())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}
