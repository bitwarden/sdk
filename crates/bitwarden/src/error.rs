//! Errors that can occur when using this SDK

use std::{borrow::Cow, fmt::Debug};

use bitwarden_api_api::apis::Error as ApiError;
use bitwarden_api_identity::apis::Error as IdentityError;
#[cfg(feature = "internal")]
use bitwarden_exporters::ExportError;
#[cfg(feature = "internal")]
use bitwarden_generators::{PassphraseError, PasswordError, UsernameError};
use passkey::client::WebauthnError;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("The client is not authenticated or the session has expired")]
    NotAuthenticated,

    #[error("The client vault is locked and needs to be unlocked before use")]
    VaultLocked,

    #[error("Access token is not in a valid format: {0}")]
    AccessTokenInvalid(#[from] AccessTokenInvalidError),

    #[error("The response received was invalid and could not be processed")]
    InvalidResponse,
    #[error("The response received was missing some of the required fields: {0}")]
    MissingFields(&'static str),

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

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

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

    #[cfg(feature = "internal")]
    #[error(transparent)]
    ExportError(#[from] ExportError),

    #[error("Webauthn error: {0:?}")]
    WebauthnError(passkey::client::WebauthnError),

    #[cfg(feature = "uniffi")]
    #[error("Uniffi callback error: {0}")]
    UniffiCallback(#[from] uniffi::UnexpectedUniFFICallbackError),

    #[error("Internal error: {0}")]
    Internal(Cow<'static, str>),
}

impl From<WebauthnError> for Error {
    fn from(e: WebauthnError) -> Self {
        Self::WebauthnError(e)
    }
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

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("{0} must not exceed {1} characters in length")]
    ExceedsCharacterLength(String, i64),
    #[error("{0} must not contain only whitespaces")]
    OnlyWhitespaces(String),
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

/// This macro is used to require that a value is present or return an error otherwise.
/// It is equivalent to using `val.ok_or(Error::MissingFields)?`, but easier to use and
/// with a more descriptive error message.
/// Note that this macro will return early from the function if the value is not present.
macro_rules! require {
    ($val:expr) => {
        match $val {
            Some(val) => val,
            None => return Err($crate::error::Error::MissingFields(stringify!($val))),
        }
    };
}
pub(crate) use require;

pub type Result<T, E = Error> = std::result::Result<T, E>;

// Validation
macro_rules! validate {
    ($val:expr) => {
        match $val.validate() {
            Ok(_) => (),
            Err(e) => return Err(e.into()),
        }
    };
}
pub(crate) use validate;

pub fn validate_whitespaces_only(value: &str) -> Result<(), validator::ValidationError> {
    if value.trim().is_empty() {
        return Err(validator::ValidationError::new("empty"));
    }
    Ok(())
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
        for (_, errors) in e.field_errors() {
            for error in errors {
                let message = error.message.as_ref().unwrap().to_string();
                match error.code.as_ref() {
                    "length_exceeded" => {
                        return Error::ValidationError(ValidationError::ExceedsCharacterLength(
                            message,
                            error.params["max"].as_i64().unwrap()
                        ));
                    }
                    "empty" => {
                        return Error::ValidationError(ValidationError::OnlyWhitespaces(
                            message
                        ));
                    }
                    _ => {}
                }
            }
        }
        "Unknown validation error".into()
    }
}