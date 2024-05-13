use std::{num::NonZeroU32, str::FromStr};

use crate::{
    AsymmetricEncString, CryptoError, EncString, SensitiveString, SensitiveVec,
    UniffiCustomTypeConverter,
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
        Ok(SensitiveString::new(Box::new(val)))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.expose().to_owned()
    }
}

uniffi::custom_type!(SensitiveVec, Vec<u8>);

impl UniffiCustomTypeConverter for SensitiveVec {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(SensitiveVec::new(Box::new(val)))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.expose().to_owned()
    }
}

/// Uniffi doesn't seem to be generating the SensitiveVec unless it's being used by
/// a record somewhere. This is a workaround to make sure the type is generated.
#[derive(uniffi::Record)]
struct SupportSensitiveVec {
    sensitive_string: SensitiveVec,
}
