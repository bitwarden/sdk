use std::fmt::{Display, Formatter};

use bitwarden_exporters::ExportError;
use bitwarden_generators::{PassphraseError, PasswordError, UsernameError};

// Name is converted from *Error to *Exception, so we can't just name the enum Error because
// Exception already exists
#[derive(uniffi::Error, Debug)]
#[uniffi(flat_error)]
pub enum BitwardenError {
    E(Error),
}

impl From<bitwarden_core::Error> for BitwardenError {
    fn from(e: bitwarden_core::Error) -> Self {
        Self::E(e.into())
    }
}

impl From<Error> for BitwardenError {
    fn from(e: Error) -> Self {
        Self::E(e)
    }
}

impl Display for BitwardenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::E(e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for BitwardenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BitwardenError::E(e) => Some(e),
        }
    }
}

pub type Result<T, E = BitwardenError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] bitwarden_core::Error),

    // Generators
    #[error(transparent)]
    UsernameError(#[from] UsernameError),
    #[error(transparent)]
    PassphraseError(#[from] PassphraseError),
    #[error(transparent)]
    PasswordError(#[from] PasswordError),

    // Vault
    #[error(transparent)]
    Cipher(#[from] bitwarden_vault::CipherError),
    #[error(transparent)]
    Totp(#[from] bitwarden_vault::TotpError),

    #[error(transparent)]
    ExportError(#[from] ExportError),

    // Fido
    #[error(transparent)]
    MakeCredential(#[from] bitwarden_fido::MakeCredentialError),
    #[error(transparent)]
    GetAssertion(#[from] bitwarden_fido::GetAssertionError),
    #[error(transparent)]
    SilentlyDiscoverCredentials(#[from] bitwarden_fido::SilentlyDiscoverCredentialsError),
    #[error(transparent)]
    CredentialsForAutofillError(#[from] bitwarden_fido::CredentialsForAutofillError),
    #[error(transparent)]
    DecryptFido2AutofillCredentialsError(
        #[from] bitwarden_fido::DecryptFido2AutofillCredentialsError,
    ),
    #[error(transparent)]
    Fido2Client(#[from] bitwarden_fido::Fido2ClientError),
}
