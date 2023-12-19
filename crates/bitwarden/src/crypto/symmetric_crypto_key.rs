use std::str::FromStr;

use aes::cipher::{generic_array::GenericArray, typenum::U32};
use base64::Engine;

use crate::{
    crypto::{derive_shareable_key, CryptoKey},
    error::{CryptoError, Error, Result},
    util::BASE64_ENGINE,
};

/// A symmetric encryption key. Used to encrypt and decrypt [`EncString`](crate::crypto::EncString)
pub struct SymmetricCryptoKey {
    pub(super) key: GenericArray<u8, U32>,
    pub(super) mac_key: Option<GenericArray<u8, U32>>,
}

impl SymmetricCryptoKey {
    const KEY_LEN: usize = 32;
    const MAC_LEN: usize = 32;

    pub fn generate(name: &str) -> Self {
        use rand::Rng;
        let secret: [u8; 16] = rand::thread_rng().gen();
        derive_shareable_key(secret, name, None)
    }

    pub fn to_base64(&self) -> String {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.key);

        if let Some(mac) = self.mac_key {
            buf.extend_from_slice(&mac);
        }

        BASE64_ENGINE.encode(&buf)
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
        let bytes = BASE64_ENGINE
            .decode(s)
            .map_err(|_| CryptoError::InvalidKey)?;
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

impl CryptoKey for SymmetricCryptoKey {}

// We manually implement these to make sure we don't print any sensitive data
impl std::fmt::Debug for SymmetricCryptoKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SymmetricCryptoKey").finish()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::SymmetricCryptoKey;

    #[test]
    fn test_symmetric_crypto_key() {
        let key = SymmetricCryptoKey::generate("test");
        let key2 = SymmetricCryptoKey::from_str(&key.to_base64()).unwrap();
        assert_eq!(key.key, key2.key);
        assert_eq!(key.mac_key, key2.mac_key);

        let key = "UY4B5N4DA4UisCNClgZtRr6VLy9ZF5BXXC7cDZRqourKi4ghEMgISbCsubvgCkHf5DZctQjVot11/vVvN9NNHQ==";
        let key2 = SymmetricCryptoKey::from_str(key).unwrap();
        assert_eq!(key, key2.to_base64());
    }
}
