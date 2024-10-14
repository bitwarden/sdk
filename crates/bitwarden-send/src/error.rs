use thiserror::Error;

#[derive(Debug, Error)]
pub enum SendParseError {
    #[error(transparent)]
    Chrono(#[from] chrono::ParseError),
    #[error(transparent)]
    Crypto(#[from] bitwarden_crypto::CryptoError),
    #[error(transparent)]
    MissingFieldError(#[from] bitwarden_core::MissingFieldError),
}
