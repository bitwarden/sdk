use std::{pin::Pin, str::FromStr};

use aes::cipher::typenum::U32;
use base64::{engine::general_purpose::STANDARD, Engine};
use generic_array::GenericArray;
use rand::Rng;
use zeroize::{Zeroize, Zeroizing};

use super::key_encryptable::CryptoKey;
use crate::CryptoError;

/// A symmetric encryption key. Used to encrypt and decrypt [`EncString`](crate::EncString)
pub struct SymmetricCryptoKey {
    // GenericArray is equivalent to [u8; N], which is a Copy type placed on the stack.
    // To keep the compiler from making stack copies when moving this struct around,
    // we use a Box to keep the values on the heap. We also pin the box to make sure
    // that the contents can't be pulled out of the box and moved
    pub(crate) key: Pin<Box<GenericArray<u8, U32>>>,
    pub(crate) mac_key: Option<Pin<Box<GenericArray<u8, U32>>>>,
}

impl Drop for SymmetricCryptoKey {
    fn drop(&mut self) {
        self.key.zeroize();
        if let Some(mac_key) = &mut self.mac_key {
            mac_key.zeroize();
        }
    }
}

impl zeroize::ZeroizeOnDrop for SymmetricCryptoKey {}

impl SymmetricCryptoKey {
    const KEY_LEN: usize = 32;
    const MAC_LEN: usize = 32;

    /// Generate a new random [SymmetricCryptoKey]
    pub fn generate(mut rng: impl rand::RngCore) -> Self {
        let mut key = Box::pin(GenericArray::<u8, U32>::default());
        let mut mac_key = Box::pin(GenericArray::<u8, U32>::default());

        rng.fill(key.as_mut_slice());
        rng.fill(mac_key.as_mut_slice());

        SymmetricCryptoKey {
            key,
            mac_key: Some(mac_key),
        }
    }

    pub(crate) fn new(
        key: Pin<Box<GenericArray<u8, U32>>>,
        mac_key: Option<Pin<Box<GenericArray<u8, U32>>>>,
    ) -> Self {
        Self { key, mac_key }
    }

    fn total_len(&self) -> usize {
        self.key.len() + self.mac_key.as_ref().map_or(0, |mac| mac.len())
    }

    pub fn to_base64(&self) -> String {
        let mut buf = self.to_vec();

        let result = STANDARD.encode(&buf);
        buf.zeroize();
        result
    }

    pub fn to_vec(&self) -> Zeroizing<Vec<u8>> {
        let mut buf = Vec::with_capacity(self.total_len());

        buf.extend_from_slice(&self.key);
        if let Some(mac) = &self.mac_key {
            buf.extend_from_slice(mac);
        }
        Zeroizing::new(buf)
    }
}

impl FromStr for SymmetricCryptoKey {
    type Err = CryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = STANDARD.decode(s).map_err(|_| CryptoError::InvalidKey)?;
        SymmetricCryptoKey::try_from(bytes.as_mut_slice())
    }
}

impl TryFrom<&mut [u8]> for SymmetricCryptoKey {
    type Error = CryptoError;

    /// Note: This function takes the byte slice by mutable reference and will zero out all
    /// the data in it. This is to prevent the key from being left in memory.
    fn try_from(value: &mut [u8]) -> Result<Self, Self::Error> {
        let result = if value.len() == Self::KEY_LEN + Self::MAC_LEN {
            let mut key = Box::pin(GenericArray::<u8, U32>::default());
            let mut mac_key = Box::pin(GenericArray::<u8, U32>::default());

            key.copy_from_slice(&value[..Self::KEY_LEN]);
            mac_key.copy_from_slice(&value[Self::KEY_LEN..]);

            Ok(SymmetricCryptoKey {
                key,
                mac_key: Some(mac_key),
            })
        } else if value.len() == Self::KEY_LEN {
            let mut key = Box::pin(GenericArray::<u8, U32>::default());

            key.copy_from_slice(&value[..Self::KEY_LEN]);

            Ok(SymmetricCryptoKey { key, mac_key: None })
        } else {
            Err(CryptoError::InvalidKeyLen)
        };

        value.zeroize();
        result
    }
}

impl CryptoKey for SymmetricCryptoKey {}

// We manually implement these to make sure we don't print any sensitive data
impl std::fmt::Debug for SymmetricCryptoKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SymmetricCryptoKey").finish()
    }
}

#[cfg(test)]
pub fn derive_symmetric_key(name: &str) -> SymmetricCryptoKey {
    use crate::{derive_shareable_key, generate_random_bytes};

    let secret: [u8; 16] = generate_random_bytes();
    derive_shareable_key(secret, name, None)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{derive_symmetric_key, SymmetricCryptoKey};

    #[test]
    fn test_symmetric_crypto_key() {
        let key = derive_symmetric_key("test");
        let key2 = SymmetricCryptoKey::from_str(&key.to_base64()).unwrap();
        assert_eq!(key.key, key2.key);
        assert_eq!(key.mac_key, key2.mac_key);

        let key = "UY4B5N4DA4UisCNClgZtRr6VLy9ZF5BXXC7cDZRqourKi4ghEMgISbCsubvgCkHf5DZctQjVot11/vVvN9NNHQ==";
        let key2 = SymmetricCryptoKey::from_str(key).unwrap();
        assert_eq!(key, key2.to_base64());
    }
}
