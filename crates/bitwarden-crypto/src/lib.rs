//! # Bitwarden Cryptographic primitives
//!
//! This crate contains the cryptographic primitives used throughout the SDK.  The general
//! aspiration is for this crate to handle all the difficult cryptographic operations and expose
//! higher level concepts to the rest of the SDK.
//!
//! <div class="warning">
//! Generally you should <b>not</b> find yourself needing to edit this crate! Everything written
//! here requires additional care and attention to ensure that the cryptographic primitives are
//! secure. </div>
//!
//! ## Example:
//!
//! ```rust
//! use bitwarden_crypto::{SymmetricCryptoKey, KeyEncryptable, KeyDecryptable, CryptoError};
//!
//! async fn example() -> Result<(), CryptoError> {
//!   let key = SymmetricCryptoKey::generate(rand::thread_rng());
//!
//!   let data = "Hello, World!".to_owned();
//!   let encrypted = data.clone().encrypt_with_key(&key)?;
//!   let decrypted: String = encrypted.decrypt_with_key(&key)?;
//!
//!   assert_eq!(data, decrypted);
//!   Ok(())
//! }
//! ```
//!
//! ## Development considerations
//!
//! This crate is expected to provide long term support for cryptographic operations. To that end,
//! the following considerations should be taken into account when making changes to this crate:
//!
//! - Limit public interfaces to the bare minimum.
//! - Breaking changes should be rare and well communicated.
//! - Serializable representation of keys and encrypted data must be supported indefinitely as we
//!   have no way to update all data.
//!
//! ### Conventions:
//!
//! - Pure Functions that deterministically "derive" keys from input are prefixed with `derive_`.
//! - Functions that generate non deterministically keys are prefixed with `make_`.
//!
//! ### Differences from `clients`
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
mod allocator;
pub use allocator::ZeroizingAllocator;

#[cfg(feature = "mobile")]
uniffi::setup_scaffolding!();

#[cfg(feature = "mobile")]
mod uniffi_support;
