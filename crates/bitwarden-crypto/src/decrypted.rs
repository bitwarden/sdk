use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::CryptoError;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Decrypted<V: Zeroize> {
    value: V,
}

impl<V: Zeroize> Decrypted<V> {
    pub fn new(value: V) -> Self {
        Self { value }
    }

    pub fn expose(&self) -> &V {
        &self.value
    }
}

impl TryFrom<Decrypted<Vec<u8>>> for Decrypted<String> {
    type Error = CryptoError;

    fn try_from(v: Decrypted<Vec<u8>>) -> Result<Self, CryptoError> {
        let mut str = v.expose().to_owned();

        let rtn = String::from_utf8(str).map_err(|_| CryptoError::InvalidUtf8String);

        str.zeroize();

        rtn.map(Decrypted::new)
    }
}
