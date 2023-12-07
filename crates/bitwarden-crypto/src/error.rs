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
    #[error("Missing Key")]
    MissingKey,

    #[error("EncString error, {0}")]
    EncString(#[from] EncStringParseError),

    #[error("Rsa error, {0}")]
    RsaError(#[from] RsaError),
}

#[derive(Debug, Error)]
pub enum EncStringParseError {
    #[error("No type detected, missing '.' separator")]
    NoType,
    #[error("Invalid type, got {enc_type} with {parts} parts")]
    InvalidType { enc_type: String, parts: usize },
    #[error("Error decoding base64: {0}")]
    InvalidBase64(#[from] base64::DecodeError),
    #[error("Invalid length: expected {expected}, got {got}")]
    InvalidLength { expected: usize, got: usize },
}

#[derive(Debug, Error)]
pub enum RsaError {
    #[error("Unable to create public key")]
    CreatePublicKey,
    #[error("Unable to create private key")]
    CreatePrivateKey,
}

pub type Result<T, E = CryptoError> = std::result::Result<T, E>;
