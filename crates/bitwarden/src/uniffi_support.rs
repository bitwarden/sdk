use std::num::NonZeroU32;

use bitwarden_crypto::{AsymmetricEncString, EncString, SensitiveString};
use uuid::Uuid;

use crate::UniffiCustomTypeConverter;

uniffi::ffi_converter_forward!(NonZeroU32, bitwarden_crypto::UniFfiTag, crate::UniFfiTag);
uniffi::ffi_converter_forward!(EncString, bitwarden_crypto::UniFfiTag, crate::UniFfiTag);
uniffi::ffi_converter_forward!(
    AsymmetricEncString,
    bitwarden_crypto::UniFfiTag,
    crate::UniFfiTag
);
uniffi::ffi_converter_forward!(
    SensitiveString,
    bitwarden_crypto::UniFfiTag,
    crate::UniFfiTag
);

type DateTime = chrono::DateTime<chrono::Utc>;
uniffi::custom_type!(DateTime, std::time::SystemTime);

impl UniffiCustomTypeConverter for chrono::DateTime<chrono::Utc> {
    type Builtin = std::time::SystemTime;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Self::from(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.into()
    }
}

uniffi::custom_type!(Uuid, String);

impl UniffiCustomTypeConverter for Uuid {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Uuid::parse_str(val.as_str()).map_err(|e| e.into())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}
