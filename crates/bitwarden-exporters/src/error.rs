use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error(transparent)]
    MissingField(#[from] bitwarden_core::MissingFieldError),
    #[error(transparent)]
    VaultLocked(#[from] bitwarden_core::VaultLocked),

    #[error("CSV error: {0}")]
    Csv(#[from] crate::csv::CsvError),
    #[error("JSON error: {0}")]
    Json(#[from] crate::json::JsonError),
    #[error("Encrypted JSON error: {0}")]
    EncryptedJsonError(#[from] crate::encrypted_json::EncryptedJsonError),

    #[error(transparent)]
    BitwardenError(#[from] bitwarden_core::Error),
    #[error(transparent)]
    BitwardenCryptoError(#[from] bitwarden_crypto::CryptoError),
}
