use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("CSV error: {0}")]
    Csv(#[from] crate::csv::CsvError),
    #[error("JSON error: {0}")]
    Json(#[from] crate::json::JsonError),
    #[error("Encrypted JSON error: {0}")]
    EncryptedJsonError(#[from] crate::encrypted_json::EncryptedJsonError),

    #[error(transparent)]
    BitwardenError(#[from] bitwarden_core::error::Error),
    #[error(transparent)]
    BitwardenCryptoError(#[from] bitwarden_crypto::CryptoError),
}
