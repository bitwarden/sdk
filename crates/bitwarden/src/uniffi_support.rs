use std::{num::NonZeroU32, str::FromStr};

use bitwarden_crypto::{AsymmEncString, EncString};
use uuid::Uuid;

use crate::{error::Error, UniffiCustomTypeConverter};

uniffi::ffi_converter_forward!(EncString, bitwarden_crypto::UniFfiTag, crate::UniFfiTag);

uniffi::custom_type!(NonZeroU32, u32);

impl UniffiCustomTypeConverter for NonZeroU32 {
    type Builtin = u32;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Self::new(val).ok_or(Error::ZeroNumber.into())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.get()
    }
}

uniffi::custom_type!(AsymmEncString, String);

impl UniffiCustomTypeConverter for AsymmEncString {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Self::from_str(&val).map_err(|e| e.into())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}

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
