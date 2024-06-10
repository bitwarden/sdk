use std::num::NonZeroU32;

use bitwarden_crypto::{AsymmetricEncString, EncString};
use uuid::Uuid;

type DateTime = chrono::DateTime<chrono::Utc>;
uniffi::ffi_converter_forward!(DateTime, bitwarden_core::UniFfiTag, crate::UniFfiTag);
uniffi::ffi_converter_forward!(Uuid, bitwarden_core::UniFfiTag, crate::UniFfiTag);

uniffi::ffi_converter_forward!(NonZeroU32, bitwarden_crypto::UniFfiTag, crate::UniFfiTag);
uniffi::ffi_converter_forward!(EncString, bitwarden_crypto::UniFfiTag, crate::UniFfiTag);
uniffi::ffi_converter_forward!(
    AsymmetricEncString,
    bitwarden_crypto::UniFfiTag,
    crate::UniFfiTag
);
