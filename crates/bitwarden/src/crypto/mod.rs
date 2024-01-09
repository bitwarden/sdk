//! # Cryptographic primitives
//!
//! This module contains the cryptographic primitives used throughout the SDK. The module makes a
//! best effort to abstract away cryptographic concepts into concepts such as
//! [`EncString`] and [`SymmetricCryptoKey`].
//!
//! ## Conventions:
//!
//! - Pure Functions that deterministically "derive" keys from input are prefixed with `derive_`.
//! - Functions that generate new keys are prefixed with `make_`.
//!
//! ## Differences from [`clients`](https://github.com/bitwarden/clients)
//!
//! There are some noteworthy differences compared to the other Bitwarden clients. These changes
//! are made in an effort to introduce conventions in how we name things, improve best practices
//! and abstracting away internal complexity.
//!
//! - `CryptoService.makeSendKey` & `AccessService.createAccessToken` are replaced by the generic
//!   `derive_shareable_key`
//! - MasterKey operations such as `makeMasterKey` and `hashMasterKey` are moved to the MasterKey
//!   struct.
//!

use aes::cipher::{generic_array::GenericArray, ArrayLength, Unsigned};
use hmac::digest::OutputSizeUser;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::error::Result;

mod enc_string;
pub use enc_string::{AsymmEncString, EncString};
mod encryptable;
pub use encryptable::{Decryptable, Encryptable, LocateKey};
mod key_encryptable;
pub use key_encryptable::{KeyDecryptable, KeyEncryptable};
mod aes_ops;
use aes_ops::{decrypt_aes256_hmac, encrypt_aes256_hmac};
mod symmetric_crypto_key;
pub use symmetric_crypto_key::SymmetricCryptoKey;
mod shareable_key;
pub(crate) use shareable_key::derive_shareable_key;

#[cfg(feature = "internal")]
mod master_key;
#[cfg(feature = "internal")]
pub use master_key::HashPurpose;
#[cfg(feature = "internal")]
pub(crate) use master_key::MasterKey;
#[cfg(feature = "internal")]
mod user_key;
#[cfg(feature = "internal")]
pub(crate) use user_key::UserKey;
#[cfg(feature = "internal")]
mod rsa;
#[cfg(feature = "internal")]
pub use self::rsa::RsaKeyPair;

#[cfg(feature = "internal")]
mod fingerprint;
#[cfg(feature = "internal")]
pub(crate) use fingerprint::fingerprint;

pub(crate) type PbkdfSha256Hmac = hmac::Hmac<sha2::Sha256>;
pub(crate) const PBKDF_SHA256_HMAC_OUT_SIZE: usize =
    <<PbkdfSha256Hmac as OutputSizeUser>::OutputSize as Unsigned>::USIZE;

/// RFC5869 HKDF-Expand operation
fn hkdf_expand<T: ArrayLength<u8>>(prk: &[u8], info: Option<&str>) -> Result<GenericArray<u8, T>> {
    let hkdf = hkdf::Hkdf::<sha2::Sha256>::from_prk(prk).map_err(|_| "invalid prk length")?;
    let mut key = GenericArray::<u8, T>::default();

    let i = info.map(|i| i.as_bytes()).unwrap_or(&[]);
    hkdf.expand(i, &mut key).map_err(|_| "invalid length")?;

    Ok(key)
}

/// Generate random bytes that are cryptographically secure
pub(crate) fn generate_random_bytes<T>() -> T
where
    Standard: Distribution<T>,
{
    rand::thread_rng().gen()
}
