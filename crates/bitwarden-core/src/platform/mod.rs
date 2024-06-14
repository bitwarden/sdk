pub mod client_platform;
#[cfg(feature = "uniffi")]
pub mod fido2;
mod generate_fingerprint;
mod get_user_api_key;
mod secret_verification_request;

#[cfg(feature = "uniffi")]
pub use fido2::{ClientFido2, Fido2Authenticator, Fido2Client};
pub use generate_fingerprint::{FingerprintRequest, FingerprintResponse};
pub(crate) use get_user_api_key::get_user_api_key;
pub use get_user_api_key::UserApiKeyResponse;
pub use secret_verification_request::SecretVerificationRequest;
