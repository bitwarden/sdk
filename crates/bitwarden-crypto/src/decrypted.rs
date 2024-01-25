use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::CryptoError;

/// A wrapper for decrypted values.
///
/// Implements `Zeroize` and `ZeroizeOnDrop` to ensure that the value is zeroized on drop. Please be careful if cloning or copying the inner value using `expose` since any copy will not be zeroized.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Decrypted<V: Zeroize> {
    value: V,
}

impl<V: Zeroize> Decrypted<V> {
    pub fn new(value: V) -> Self {
        Self { value }
    }

    /// Expose the inner value. By exposing the inner value, you take responsibility for ensuring that any copy of the value is zeroized.
    pub fn expose(&self) -> &V {
        &self.value
    }
}

/// Helper to convert a `Decrypted<Vec<u8>>` to a `Decrypted<String>`, care is taken to ensure any intermediate copies are zeroed to avoid leaking sensitive data.
impl TryFrom<DecryptedVec> for DecryptedString {
    type Error = CryptoError;

    fn try_from(mut v: DecryptedVec) -> Result<Self, CryptoError> {
        let value = std::mem::take(&mut v.value);

        let rtn = String::from_utf8(value).map_err(|_| CryptoError::InvalidUtf8String);
        rtn.map(Decrypted::new)
    }
}

pub type DecryptedVec = Decrypted<Vec<u8>>;
pub type DecryptedString = Decrypted<String>;
