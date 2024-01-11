use std::str::FromStr;

use aes::cipher::{generic_array::GenericArray, typenum::U32};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::Rng;

use crate::error::{CryptoError, Error};

/// A symmetric encryption key. Used to encrypt and decrypt [`EncString`](crate::crypto::EncString)
pub struct SymmetricCryptoKey {
    pub key: GenericArray<u8, U32>,
    pub mac_key: Option<GenericArray<u8, U32>>,
}

impl SymmetricCryptoKey {
    const KEY_LEN: usize = 32;
    const MAC_LEN: usize = 32;

    /// Generate a new random [SymmetricCryptoKey]
    pub fn generate(mut rng: impl rand::RngCore) -> Result<Self, Error> {
        let mut key: [u8; 64] = [0u8; 64];
        rng.fill(&mut key);

        SymmetricCryptoKey::try_from(key.as_slice())
    }

    pub fn to_base64(&self) -> String {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.key);

        if let Some(mac) = self.mac_key {
            buf.extend_from_slice(&mac);
        }

        STANDARD.encode(&buf)
    }

    #[cfg(feature = "internal")]
    pub(super) fn to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.key);
        if let Some(mac) = self.mac_key {
            buf.extend_from_slice(&mac);
        }
        buf
    }
}

impl FromStr for SymmetricCryptoKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = STANDARD.decode(s).map_err(|_| CryptoError::InvalidKey)?;
        SymmetricCryptoKey::try_from(bytes.as_slice())
    }
}

impl TryFrom<&[u8]> for SymmetricCryptoKey {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == Self::KEY_LEN + Self::MAC_LEN {
            Ok(SymmetricCryptoKey {
                key: GenericArray::clone_from_slice(&value[..Self::KEY_LEN]),
                mac_key: Some(GenericArray::clone_from_slice(&value[Self::KEY_LEN..])),
            })
        } else if value.len() == Self::KEY_LEN {
            Ok(SymmetricCryptoKey {
                key: GenericArray::clone_from_slice(value),
                mac_key: None,
            })
        } else {
            Err(CryptoError::InvalidKeyLen.into())
        }
    }
}

// We manually implement these to make sure we don't print any sensitive data
impl std::fmt::Debug for SymmetricCryptoKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key").finish()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::SymmetricCryptoKey;

    pub fn derive_symmetric_key(name: &str) -> SymmetricCryptoKey {
        use crate::crypto::{derive_shareable_key, generate_random_bytes};

        let secret: [u8; 16] = generate_random_bytes();
        derive_shareable_key(secret, name, None)
    }

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
