//! Sensitive data types
//!
//! Provides `Sensitive` types that are designed to hold sensitive data. The data is zeroized when
//! dropped.

#[allow(clippy::module_inception)]
mod sensitive;
pub use sensitive::{Sensitive, SensitiveVec};
mod decrypted;
pub use decrypted::{Decrypted, DecryptedString, DecryptedVec};
mod string;
pub use string::SensitiveString;
mod base64;
