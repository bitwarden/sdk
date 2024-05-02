use std::{num::NonZeroU32, str::FromStr};

use crate::{
    AsymmetricEncString, CryptoError, EncString, SensitiveString, UniffiCustomTypeConverter,
};

uniffi::custom_type!(NonZeroU32, u32);

impl UniffiCustomTypeConverter for NonZeroU32 {
    type Builtin = u32;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Self::new(val).ok_or(CryptoError::ZeroNumber.into())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.get()
    }
}

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

uniffi::custom_type!(AsymmetricEncString, String);

impl UniffiCustomTypeConverter for AsymmetricEncString {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Self::from_str(&val).map_err(|e| e.into())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}

uniffi::custom_type!(SensitiveString, String);

impl UniffiCustomTypeConverter for SensitiveString {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(SensitiveString::new(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.as_str().to_owned()
    }
}
