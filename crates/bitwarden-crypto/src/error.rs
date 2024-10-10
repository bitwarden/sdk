use std::fmt::Debug;

use thiserror::Error;
use uuid::Uuid;

use crate::fingerprint::FingerprintError;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("The provided key is not the expected type")]
    InvalidKey,
    #[error("The cipher's MAC doesn't match the expected value")]
    InvalidMac,
    #[error("The key provided expects mac protected encstrings, but the mac is missing")]
    MacNotProvided,
    #[error("Error while decrypting EncString")]
    KeyDecrypt,
    #[error("The cipher key has an invalid length")]
    InvalidKeyLen,
    #[error("The value is not a valid UTF8 String")]
    InvalidUtf8String,
    #[error("Missing Key for organization with ID {0}")]
    MissingKey(Uuid),
    #[error("The item was missing a required field: {0}")]
    MissingField(&'static str),

    #[error("Insufficient KDF parameters")]
    InsufficientKdfParameters,

    #[error("EncString error, {0}")]
    EncString(#[from] EncStringParseError),

    #[error("Rsa error, {0}")]
    RsaError(#[from] RsaError),

    #[error("Fingerprint error, {0}")]
    FingerprintError(#[from] FingerprintError),

    #[error("Argon2 error, {0}")]
    ArgonError(#[from] argon2::Error),

    #[error("Number is zero")]
    ZeroNumber,
}

#[derive(Debug, Error)]
pub enum EncStringParseError {
    #[error("No type detected, missing '.' separator")]
    NoType,
    #[error("Invalid symmetric type, got type {enc_type} with {parts} parts")]
    InvalidTypeSymm { enc_type: String, parts: usize },
    #[error("Invalid asymmetric type, got type {enc_type} with {parts} parts")]
    InvalidTypeAsymm { enc_type: String, parts: usize },
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
    #[error("Rsa error, {0}")]
    Rsa(#[from] rsa::Error),
}

/// Alias for `Result<T, CryptoError>`.
pub(crate) type Result<T, E = CryptoError> = std::result::Result<T, E>;
