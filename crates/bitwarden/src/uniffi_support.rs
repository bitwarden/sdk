use std::{num::NonZeroU32, str::FromStr};

use uniffi::{FfiConverter, MetadataBuffer, RustBuffer};
use uuid::Uuid;

use crate::{crypto::CipherString, error::Error, UniFfiTag};

// Could probably be replaced with https://github.com/mozilla/uniffi-rs/pull/1661
macro_rules! ffi_converter_impl {
    ($type:ty, $intermediate_rust:ty, $intermediate_ffi:ty, $to_intermediate:expr, $from_intermediate:expr) => {
        unsafe impl FfiConverter<UniFfiTag> for $type {
            uniffi::ffi_converter_default_return!(crate::UniFfiTag);

            type FfiType = $intermediate_ffi;

            fn lower(obj: Self) -> Self::FfiType {
                <$intermediate_rust as FfiConverter<UniFfiTag>>::lower(($to_intermediate)(obj))
            }
            fn try_lift(v: Self::FfiType) -> uniffi::Result<Self> {
                let s = <$intermediate_rust as FfiConverter<UniFfiTag>>::try_lift(v)?;
                Ok(($from_intermediate)(s)?)
            }
            fn write(obj: Self, buf: &mut Vec<u8>) {
                <$intermediate_rust as FfiConverter<UniFfiTag>>::write(
                    ($to_intermediate)(obj),
                    buf,
                );
            }
            fn try_read(buf: &mut &[u8]) -> uniffi::Result<Self> {
                let s = <$intermediate_rust as FfiConverter<UniFfiTag>>::try_read(buf)?;
                Ok(($from_intermediate)(s)?)
            }
            const TYPE_ID_META: MetadataBuffer =
                MetadataBuffer::from_code(uniffi::metadata::codes::TYPE_CUSTOM)
                    .concat_str("bitwarden")
                    .concat_str(stringify!($type))
                    .concat(<$intermediate_rust as FfiConverter<UniFfiTag>>::TYPE_ID_META);
        }
    };
}

ffi_converter_impl!(
    Uuid,
    String,
    RustBuffer,
    |u: Self| u.to_string(),
    |s: String| Self::parse_str(&s)
);

ffi_converter_impl!(
    CipherString,
    String,
    RustBuffer,
    |u: Self| u.to_string(),
    |s: String| { Self::from_str(&s) }
);

// Can't have a generic in the type
type DateTime = chrono::DateTime<chrono::Utc>;
// TODO: Convert to systemtime
ffi_converter_impl!(
    DateTime,
    std::time::SystemTime,
    RustBuffer,
    |u: Self| u.into(),
    |s: std::time::SystemTime| { Ok::<_, Error>(Self::from(s)) }
);

ffi_converter_impl!(NonZeroU32, u32, u32, |u: Self| u.get(), |s: u32| {
    Self::new(s).ok_or(Error::Internal("Number is zero"))
});
