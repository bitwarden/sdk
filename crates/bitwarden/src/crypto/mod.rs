//! Cryptographic primitives used in the SDK

use aes::cipher::Unsigned;
use hmac::digest::OutputSizeUser;

mod enc_string;
pub use enc_string::EncString;
mod encryptable;
pub use encryptable::{Decryptable, Encryptable};
mod aes_ops;
pub use aes_ops::{decrypt_aes256, encrypt_aes256};
mod symmetric_crypto_key;
pub use symmetric_crypto_key::SymmetricCryptoKey;
mod shareable_key;
pub(crate) use shareable_key::stretch_key;

#[cfg(feature = "internal")]
mod master_key;
#[cfg(any(feature = "internal", feature = "mobile"))]
pub(crate) use master_key::hash_kdf;
#[cfg(feature = "internal")]
pub(crate) use master_key::stretch_key_password;

#[cfg(feature = "internal")]
mod fingerprint;
#[cfg(feature = "internal")]
pub(crate) use fingerprint::fingerprint;

pub(crate) type PbkdfSha256Hmac = hmac::Hmac<sha2::Sha256>;
pub(crate) const PBKDF_SHA256_HMAC_OUT_SIZE: usize =
    <<PbkdfSha256Hmac as OutputSizeUser>::OutputSize as Unsigned>::USIZE;
