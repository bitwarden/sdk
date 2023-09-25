use std::{str::FromStr, marker::PhantomData};

use aes::cipher::{generic_array::GenericArray, typenum::U32};
use base64::Engine;

use crate::{
    error::{CryptoError, Error, Result},
    util::BASE64_ENGINE, crypto::RsaKeyPair,
};

/// Marker trait to annotate the purpose of a key
pub trait KeyPurpose {}

/// Marker trait to annotate that the key is capable of generating an asymmetric key pair
pub trait AsymmetricKeyPairGeneration : KeyPurpose {}

/// A symmetric encryption key. Used to encrypt and decrypt [`EncString`](crate::crypto::EncString)
pub(crate) struct SymmetricCryptoKey<TKeyPurpose : KeyPurpose> {
    pub key: GenericArray<u8, U32>,
    pub mac_key: Option<GenericArray<u8, U32>>,
    _type: PhantomData<TKeyPurpose>,
}

impl<TKeyPurpose : KeyPurpose> SymmetricCryptoKey<TKeyPurpose> {
    const KEY_LEN: usize = 32;
    const MAC_LEN: usize = 32;

    pub fn make_key(key: GenericArray<u8, U32>, mac_key: Option<GenericArray<u8, U32>>) -> Self {
        SymmetricCryptoKey {
            key,
            mac_key,
            _type: PhantomData,
        }
    }

    pub fn to_base64(&self) -> String {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.key);

        if let Some(mac) = self.mac_key {
            buf.extend_from_slice(&mac);
        }

        BASE64_ENGINE.encode(&buf)
    }
}

impl<TKeyPurpose : AsymmetricKeyPairGeneration> SymmetricCryptoKey<TKeyPurpose> {
    pub fn make_key_pair(&self) -> Result<RsaKeyPair> {
        crate::crypto::rsa::make_key_pair(&self)
    }
}

impl<TKeyPurpose> FromStr for SymmetricCryptoKey<TKeyPurpose> where TKeyPurpose : KeyPurpose {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = BASE64_ENGINE
            .decode(s)
            .map_err(|_| CryptoError::InvalidKey)?;
        SymmetricCryptoKey::try_from(bytes.as_slice())
    }
}

impl<TKeyPurpose> TryFrom<&[u8]> for SymmetricCryptoKey<TKeyPurpose> where TKeyPurpose : KeyPurpose {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == Self::KEY_LEN + Self::MAC_LEN {
            Ok(SymmetricCryptoKey {
                key: GenericArray::clone_from_slice(&value[..Self::KEY_LEN]),
                mac_key: Some(GenericArray::clone_from_slice(&value[Self::KEY_LEN..])),
                _type: PhantomData,
            })
        } else if value.len() == Self::KEY_LEN {
            Ok(SymmetricCryptoKey {
                key: GenericArray::clone_from_slice(value),
                mac_key: None,
                _type: PhantomData,
            })
        } else {
            Err(CryptoError::InvalidKeyLen.into())
        }
    }
}

// We manually implement these to make sure we don't print any sensitive data
impl<TKeyPurpose: KeyPurpose> std::fmt::Debug for SymmetricCryptoKey<TKeyPurpose> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key").finish()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    struct SymmetricCryptoKeyTests {}
    impl KeyPurpose for SymmetricCryptoKeyTests {}

    #[test]
    fn test_symmetric_crypto_key() {
        let key = SymmetricCryptoKey::<SymmetricCryptoKeyTests>::generate("test");
        let key2 = SymmetricCryptoKey::<SymmetricCryptoKeyTests>::from_str(&key.to_base64()).unwrap();
        assert_eq!(key.key, key2.key);
        assert_eq!(key.mac_key, key2.mac_key);

        let key = "UY4B5N4DA4UisCNClgZtRr6VLy9ZF5BXXC7cDZRqourKi4ghEMgISbCsubvgCkHf5DZctQjVot11/vVvN9NNHQ==";
        let key2 = SymmetricCryptoKey::<SymmetricCryptoKeyTests>::from_str(key).unwrap();
        assert_eq!(key, key2.to_base64());
    }
}
