//! # Bitwarden Cryptographic primitives
//!
//! This crate contains the cryptographic primitives used throughout the SDK. The crate makes a
//! best effort to abstract away cryptographic concepts into concepts such as [`EncString`],
//! [`AsymmetricEncString`] and [`SymmetricCryptoKey`].
//!
//! ## Conventions:
//!
//! - Pure Functions that deterministically "derive" keys from input are prefixed with `derive_`.
//! - Functions that generate non deterministically keys are prefixed with `make_`.
//!
//! ## Differences from `clients`
//!
//! There are some noteworthy differences compared to the other Bitwarden
//! [clients](https://github.com/bitwarden/clients). These changes are made in an effort to
//! introduce conventions in how we name things, improve best practices and abstracting away
//! internal complexity.
//!
//! - `CryptoService.makeSendKey` & `AccessService.createAccessToken` are replaced by the generic
//!   `derive_shareable_key`
//! - MasterKey operations such as `makeMasterKey` and `hashMasterKey` are moved to the MasterKey
//!   struct.

mod aes;
mod enc_string;
pub use enc_string::{AsymmetricEncString, EncString};
mod encryptable;
pub use encryptable::{Decryptable, Encryptable, KeyContainer, LocateKey};
mod error;
pub use error::CryptoError;
pub(crate) use error::Result;
mod fingerprint;
pub use fingerprint::fingerprint;
mod keys;
pub use keys::*;
mod rsa;
pub use crate::rsa::RsaKeyPair;
mod util;
pub use util::generate_random_bytes;
mod wordlist;
pub use util::pbkdf2;
pub use wordlist::EFF_LONG_WORD_LIST;

#[cfg(feature = "mobile")]
uniffi::setup_scaffolding!();

#[cfg(feature = "mobile")]
mod uniffi_support;
