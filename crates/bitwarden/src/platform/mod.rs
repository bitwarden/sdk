#[cfg(feature = "uniffi")]
pub mod client_fido;
pub mod client_platform;
mod generate_fingerprint;
mod get_user_api_key;
mod secret_verification_request;

pub use generate_fingerprint::{FingerprintRequest, FingerprintResponse};
pub(crate) use get_user_api_key::get_user_api_key;
pub use get_user_api_key::UserApiKeyResponse;
pub use secret_verification_request::SecretVerificationRequest;

#[cfg(feature = "uniffi")]
pub mod fido2 {
    pub use bitwarden_fido::*;
}
