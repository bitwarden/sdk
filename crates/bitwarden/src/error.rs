//! Errors that can occur when using this SDK

use std::{borrow::Cow, fmt::Debug};

use bitwarden_api_api::apis::Error as ApiError;
use bitwarden_api_identity::apis::Error as IdentityError;
#[cfg(feature = "internal")]
use bitwarden_exporters::ExportError;
#[cfg(feature = "internal")]
use bitwarden_generators::{PassphraseError, PasswordError, UsernameError};
use log::debug;
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

    // Send
    #[cfg(feature = "internal")]
    #[error(transparent)]
    SendParseError(#[from] bitwarden_send::SendParseError),

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
    MakeCredential(#[from] bitwarden_fido::MakeCredentialError),
    #[cfg(all(feature = "uniffi", feature = "internal"))]
    #[error(transparent)]
    GetAssertion(#[from] bitwarden_fido::GetAssertionError),
    #[cfg(all(feature = "uniffi", feature = "internal"))]
    #[error(transparent)]
    SilentlyDiscoverCredentials(#[from] bitwarden_fido::SilentlyDiscoverCredentialsError),
    #[cfg(all(feature = "uniffi", feature = "internal"))]
    #[error(transparent)]
    CredentialsForAutofillError(#[from] bitwarden_fido::CredentialsForAutofillError),
    #[cfg(all(feature = "uniffi", feature = "internal"))]
    #[error(transparent)]
    DecryptFido2AutofillCredentialsError(
        #[from] crate::platform::fido2::DecryptFido2AutofillCredentialsError,
    ),
    #[cfg(all(feature = "uniffi", feature = "internal"))]
    #[error(transparent)]
    Fido2Client(#[from] bitwarden_fido::Fido2ClientError),
    #[cfg(all(feature = "uniffi", feature = "internal"))]
    #[error("Fido2 Callback error: {0:?}")]
    Fido2CallbackError(#[from] bitwarden_fido::Fido2CallbackError),

    #[cfg(feature = "uniffi")]
    #[error("Uniffi callback error: {0}")]
    UniffiCallbackError(#[from] uniffi::UnexpectedUniFFICallbackError),

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

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("{0} must not be empty")]
    Required(String),
    #[error("{0} must not exceed {1} characters in length")]
    ExceedsCharacterLength(String, u64),
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

pub type Result<T, E = Error> = std::result::Result<T, E>;

// Validation
const VALIDATION_LENGTH_CODE: &str = "length";
const VALIDATION_ONLY_WHITESPACES_CODE: &str = "only_whitespaces";

macro_rules! validate {
    ($val:expr) => {
        match $val.validate() {
            Ok(_) => (),
            Err(e) => return Err(e.into()),
        }
    };
}
pub(crate) use validate;

pub(crate) fn validate_only_whitespaces(value: &str) -> Result<(), validator::ValidationError> {
    if !value.is_empty() && value.trim().is_empty() {
        return Err(validator::ValidationError::new(
            VALIDATION_ONLY_WHITESPACES_CODE,
        ));
    }
    Ok(())
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
        debug!("Validation errors: {:#?}", e);
        for (field_name, errors) in e.field_errors() {
            for error in errors {
                match error.code.as_ref() {
                    VALIDATION_LENGTH_CODE => {
                        if error.params.contains_key("min")
                            && error.params["min"].as_u64().expect("Min provided") == 1
                            && error.params["value"]
                                .as_str()
                                .expect("Value provided")
                                .is_empty()
                        {
                            return Error::ValidationError(ValidationError::Required(
                                field_name.to_string(),
                            ));
                        } else if error.params.contains_key("max") {
                            return Error::ValidationError(
                                ValidationError::ExceedsCharacterLength(
                                    field_name.to_string(),
                                    error.params["max"].as_u64().expect("Max provided"),
                                ),
                            );
                        }
                    }
                    VALIDATION_ONLY_WHITESPACES_CODE => {
                        return Error::ValidationError(ValidationError::OnlyWhitespaces(
                            field_name.to_string(),
                        ));
                    }
                    _ => {}
                }
            }
        }
        "Unknown validation error".into()
    }
}
