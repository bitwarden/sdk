use std::str::FromStr;

use base64::Engine;
use rsa::RsaPrivateKey;

use crate::{
    crypto::CryptoKey,
    error::{CryptoError, Error, Result},
    util::BASE64_ENGINE,
};

/// An asymmetric encryption key. Used to encrypt and decrypt [`EncString`](crate::crypto::EncString)
pub struct AsymmetricCryptoKey {
    pub(in crate::crypto) key: RsaPrivateKey,
}

impl AsymmetricCryptoKey {
    pub fn from_pem(pem: &str) -> Result<Self> {
        use rsa::pkcs8::DecodePrivateKey;
        Ok(Self {
            key: rsa::RsaPrivateKey::from_pkcs8_pem(pem).map_err(|_| CryptoError::InvalidKey)?,
        })
    }
}

impl FromStr for AsymmetricCryptoKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = BASE64_ENGINE
            .decode(s)
            .map_err(|_| CryptoError::InvalidKey)?;
        AsymmetricCryptoKey::try_from(bytes.as_slice())
    }
}

impl TryFrom<&[u8]> for AsymmetricCryptoKey {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        use rsa::pkcs8::DecodePrivateKey;
        Ok(Self {
            key: rsa::RsaPrivateKey::from_pkcs8_der(value).map_err(|_| CryptoError::InvalidKey)?,
        })
    }
}

impl CryptoKey for AsymmetricCryptoKey {}

// We manually implement these to make sure we don't print any sensitive data
impl std::fmt::Debug for AsymmetricCryptoKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsymmetricCryptoKey").finish()
    }
}

/*
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::AsymmetricCryptoKey;

    #[test]
    fn test_asymmetric_crypto_key() {
        let key = AsymmetricCryptoKey::generate("test");
        let key2 = AsymmetricCryptoKey::from_str(&key.to_base64()).unwrap();
        assert_eq!(key.key, key2.key);
        assert_eq!(key.mac_key, key2.mac_key);

        let key = "UY4B5N4DA4UisCNClgZtRr6VLy9ZF5BXXC7cDZRqourKi4ghEMgISbCsubvgCkHf5DZctQjVot11/vVvN9NNHQ==";
        let key2 = AsymmetricCryptoKey::from_str(key).unwrap();
        assert_eq!(key, key2.to_base64());
    }
}
*/
