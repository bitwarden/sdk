//! # Bitwarden Cryptographic primitives
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

use base64::{
    alphabet,
    engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
};

#[cfg(feature = "mobile")]
uniffi::setup_scaffolding!();

pub mod aes;
mod error;
mod key_encryptable;
pub use key_encryptable::{KeyDecryptable, KeyEncryptable};
mod shareable_key;
mod symmetric_crypto_key;
pub use error::{CryptoError, Result};
pub use shareable_key::derive_shareable_key;
pub use symmetric_crypto_key::SymmetricCryptoKey;
mod encryptable;
pub use encryptable::{Decryptable, Encryptable, KeyContainer, LocateKey};
mod enc_string;
pub use enc_string::{AsymmEncString, EncString};
pub mod rsa;
mod user_key;
pub use user_key::UserKey;

mod util;

#[cfg(feature = "internal")]
mod wordlist;
#[cfg(feature = "internal")]
pub use wordlist::EFF_LONG_WORD_LIST;

#[cfg(feature = "internal")]
mod fingerprint;
#[cfg(feature = "internal")]
pub use fingerprint::fingerprint;

#[cfg(feature = "internal")]
mod master_key;
#[cfg(feature = "internal")]
pub use master_key::{HashPurpose, Kdf, MasterKey};

#[cfg(feature = "mobile")]
mod uniffi_support;
#[cfg(feature = "mobile")]
pub use uniffi_support::*;

// TODO: Replace with standard base64 engine
const BASE64_ENGINE_CONFIG: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(true)
    .with_decode_padding_mode(DecodePaddingMode::Indifferent);

pub const BASE64_ENGINE: GeneralPurpose =
    GeneralPurpose::new(&alphabet::STANDARD, BASE64_ENGINE_CONFIG);
