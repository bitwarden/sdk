use std::str::FromStr;

use aes::cipher::{generic_array::GenericArray, typenum::U32};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::Rng;

use crate::CryptoError;

use super::key_encryptable::CryptoKey;

/// A symmetric encryption key. Used to encrypt and decrypt [`EncString`](crate::EncString)
pub struct SymmetricCryptoKey {
    pub(crate) key: GenericArray<u8, U32>,
    pub(crate) mac_key: Option<GenericArray<u8, U32>>,
}

impl SymmetricCryptoKey {
    const KEY_LEN: usize = 32;
    const MAC_LEN: usize = 32;

    /// Generate a new random [SymmetricCryptoKey]
    pub fn generate(mut rng: impl rand::RngCore) -> Self {
        let mut key = [0u8; Self::KEY_LEN];
        let mut mac_key = [0u8; Self::MAC_LEN];

        rng.fill(&mut key);
        rng.fill(&mut mac_key);

        SymmetricCryptoKey {
            key: key.into(),
            mac_key: Some(mac_key.into()),
        }
    }

    pub fn to_base64(&self) -> String {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.key);

        if let Some(mac) = self.mac_key {
            buf.extend_from_slice(&mac);
        }

        STANDARD.encode(&buf)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.key);
        if let Some(mac) = self.mac_key {
            buf.extend_from_slice(&mac);
        }
        buf
    }
}

impl FromStr for SymmetricCryptoKey {
    type Err = CryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = STANDARD.decode(s).map_err(|_| CryptoError::InvalidKey)?;
        SymmetricCryptoKey::try_from(bytes.as_slice())
    }
}

impl TryFrom<&[u8]> for SymmetricCryptoKey {
    type Error = CryptoError;

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
            Err(CryptoError::InvalidKeyLen)
        }
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
