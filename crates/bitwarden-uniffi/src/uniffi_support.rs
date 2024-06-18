use bitwarden_crypto::{AsymmetricEncString, EncString};
use uuid::Uuid;

// Forward the type definitions to the main bitwarden crate
type DateTime = chrono::DateTime<chrono::Utc>;
uniffi::ffi_converter_forward!(DateTime, bitwarden_core::UniFfiTag, crate::UniFfiTag);
uniffi::ffi_converter_forward!(EncString, bitwarden::UniFfiTag, crate::UniFfiTag);
uniffi::ffi_converter_forward!(AsymmetricEncString, bitwarden::UniFfiTag, crate::UniFfiTag);
uniffi::ffi_converter_forward!(Uuid, bitwarden_core::UniFfiTag, crate::UniFfiTag);
