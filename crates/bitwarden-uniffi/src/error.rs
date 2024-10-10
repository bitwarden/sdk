use bitwarden_exporters::ExportError;
use bitwarden_generators::{PassphraseError, PasswordError, UsernameError};

// Name is converted from *Error to *Exception, so we can't just name the enum Error because
// Exception already exists
#[derive(uniffi::Error, thiserror::Error, Debug)]
#[uniffi(flat_error)]
pub enum BitwardenError {
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

pub type Result<T, E = BitwardenError> = std::result::Result<T, E>;
