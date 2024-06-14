//! Errors that can occur when using this SDK

use std::{borrow::Cow, fmt::Debug};

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

    #[error("The response received was invalid and could not be processed")]
    InvalidResponse,

    #[error("Cryptography error, {0}")]
    Crypto(#[from] bitwarden_crypto::CryptoError),

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

pub type Result<T, E = Error> = std::result::Result<T, E>;
