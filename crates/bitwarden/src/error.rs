//! Errors that can occur when using this SDK

use std::{borrow::Cow, fmt::Debug};

use bitwarden_api_api::apis::Error as ApiError;
use bitwarden_api_identity::apis::Error as IdentityError;
#[cfg(feature = "internal")]
use bitwarden_exporters::ExportError;
#[cfg(feature = "internal")]
use bitwarden_generators::{PassphraseError, PasswordError, UsernameError};
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    MissingFieldError(#[from] bitwarden_core::MissingFieldError),
    #[error(transparent)]
    VaultLocked(#[from] bitwarden_core::VaultLocked),

    #[error("The client is not authenticated or the session has expired")]
    NotAuthenticated,

    #[error("Access token is not in a valid format: {0}")]
    AccessTokenInvalid(#[from] AccessTokenInvalidError),

    #[error("The response received was invalid and could not be processed")]
    InvalidResponse,

    #[error("Cryptography error, {0}")]
    Crypto(#[from] bitwarden_crypto::CryptoError),

    #[error("Error parsing Identity response: {0}")]
    IdentityFail(crate::auth::api::response::IdentityTokenFailResponse),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    InvalidBase64(#[from] base64::DecodeError),
    #[error(transparent)]
    Chrono(#[from] chrono::ParseError),

    #[error("Received error message from server: [{}] {}", .status, .message)]
    ResponseContent { status: StatusCode, message: String },

    #[error("The state file version is invalid")]
    InvalidStateFileVersion,

    #[error("The state file could not be read")]
    InvalidStateFile,

    // Generators
    #[cfg(feature = "internal")]
    #[error(transparent)]
    UsernameError(#[from] UsernameError),
    #[cfg(feature = "internal")]
    #[error(transparent)]
    PassphraseError(#[from] PassphraseError),
    #[cfg(feature = "internal")]
    #[error(transparent)]
    PasswordError(#[from] PasswordError),

    // Vault
    #[cfg(feature = "internal")]
    #[error(transparent)]
    Cipher(#[from] bitwarden_vault::CipherError),
    #[cfg(feature = "internal")]
    #[error(transparent)]
    VaultParse(#[from] bitwarden_vault::VaultParseError),
    #[cfg(feature = "internal")]
    #[error(transparent)]
    Totp(#[from] bitwarden_vault::TotpError),

    #[cfg(feature = "internal")]
    #[error(transparent)]
    ExportError(#[from] ExportError),

    // Fido
    #[cfg(all(feature = "uniffi", feature = "internal"))]
    #[error(transparent)]
    MakeCredential(#[from] crate::platform::fido2::MakeCredentialError),
    #[cfg(all(feature = "uniffi", feature = "internal"))]
    #[error(transparent)]
    GetAssertion(#[from] crate::platform::fido2::GetAssertionError),
    #[cfg(all(feature = "uniffi", feature = "internal"))]
    #[error(transparent)]
    SilentlyDiscoverCredentials(#[from] crate::platform::fido2::SilentlyDiscoverCredentialsError),
    #[cfg(all(feature = "uniffi", feature = "internal"))]
    #[error(transparent)]
    Fido2Client(#[from] crate::platform::fido2::Fido2ClientError),

    #[cfg(feature = "uniffi")]
    #[error("Uniffi callback error: {0}")]
    UniffiCallbackError(#[from] uniffi::UnexpectedUniFFICallbackError),

    #[cfg(all(feature = "uniffi", feature = "internal"))]
    #[error("Fido2 Callback error: {0:?}")]
    Fido2CallbackError(#[from] crate::platform::fido2::Fido2CallbackError),

    #[error("Internal error: {0}")]
    Internal(Cow<'static, str>),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Internal(s.into())
    }
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Self::Internal(s.into())
    }
}

#[derive(Debug, Error)]
pub enum AccessTokenInvalidError {
    #[error("Doesn't contain a decryption key")]
    NoKey,
    #[error("Has the wrong number of parts")]
    WrongParts,
    #[error("Is the wrong version")]
    WrongVersion,
    #[error("Has an invalid identifier")]
    InvalidUuid,

    #[error("Error decoding base64: {0}")]
    InvalidBase64(#[from] base64::DecodeError),

    #[error("Invalid base64 length: expected {expected}, got {got}")]
    InvalidBase64Length { expected: usize, got: usize },
}

// Ensure that the error messages implement Send and Sync
#[cfg(test)]
const _: () = {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    fn assert_all() {
        assert_send::<Error>();
        assert_sync::<Error>();
    }
};

macro_rules! impl_bitwarden_error {
    ($name:ident) => {
        impl<T> From<$name<T>> for Error {
            fn from(e: $name<T>) -> Self {
                match e {
                    $name::Reqwest(e) => Self::Reqwest(e),
                    $name::ResponseError(e) => Self::ResponseContent {
                        status: e.status,
                        message: e.content,
                    },
                    $name::Serde(e) => Self::Serde(e),
                    $name::Io(e) => Self::Io(e),
                }
            }
        }
    };
}
impl_bitwarden_error!(ApiError);
impl_bitwarden_error!(IdentityError);

pub type Result<T, E = Error> = std::result::Result<T, E>;
