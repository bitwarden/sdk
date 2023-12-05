use std::fmt::Debug;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("The provided key is not the expected type")]
    InvalidKey,
    #[error("The cipher's MAC doesn't match the expected value")]
    InvalidMac,
    #[error("Error while decrypting EncString")]
    KeyDecrypt,
    #[error("The cipher key has an invalid length")]
    InvalidKeyLen,
    #[error("There is no encryption key for the provided organization")]
    NoKeyForOrg,
    #[error("The value is not a valid UTF8 String")]
    InvalidUtf8String,
}

pub type Result<T, E = CryptoError> = std::result::Result<T, E>;
