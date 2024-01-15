use std::{marker::PhantomData, str::FromStr};

use aes::cipher::{generic_array::GenericArray, typenum::U32};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::Rng;

use crate::{
    crypto::{purpose, CryptoKey, KeyPurpose},
    error::{CryptoError, Error, Result},
};

/// A symmetric encryption key. Used to encrypt and decrypt [`EncString`](crate::crypto::EncString)
#[repr(C)]
pub struct SymmetricCryptoKey<Purpose: KeyPurpose> {
    pub(super) key: GenericArray<u8, U32>,
    pub(super) mac_key: Option<GenericArray<u8, U32>>,

    pub(super) _marker: std::marker::PhantomData<Purpose>,
}

macro_rules! from_convert {
    ( $( $from:ident -> $to:ident ,)* ) => {
        $(
            impl From<SymmetricCryptoKey<$from>> for SymmetricCryptoKey<$to> {
                fn from(key: SymmetricCryptoKey<$from>) -> Self {
                    SymmetricCryptoKey {
                        key: key.key,
                        mac_key: key.mac_key,
                        _marker: std::marker::PhantomData,
                    }
                }
            }

            impl From<&SymmetricCryptoKey<$from>> for &SymmetricCryptoKey<$to> {
                fn from(key: &SymmetricCryptoKey<$from>) -> Self {
                    // Safety: We're only transmuting the PhantomData which is a ZST and the type is repr(C), so this should be safe?
                    // TODO: We should probably find a better way to do this if possible
                    unsafe { std::mem::transmute(key) }
                }
            }
        )*
    };
}
mod conversion {
    use super::{purpose::*, SymmetricCryptoKey};

    from_convert! {
        UserEncryption -> UserOrOrgEncryption,
        OrgEncryption -> UserOrOrgEncryption,

        UserOrOrgEncryption -> CipherEncryption,

        Shareable -> SendEncryption,
        Shareable -> PayloadEncryption,

        // TODO: This hack is needed because EncryptionSettings mixes the concept of the single org key in secrets manager as the user key
        UserEncryption -> OrgEncryption,
    }

    #[cfg(test)]
    from_convert! {
        Shareable -> Testing,
        Testing -> UserEncryption,
        Testing -> Master,
    }
}

impl<Purpose: KeyPurpose> SymmetricCryptoKey<Purpose> {
    const KEY_LEN: usize = 32;
    const MAC_LEN: usize = 32;

    /// Generate a new random [SymmetricCryptoKey]
    pub fn generate(mut rng: impl rand::RngCore) -> Self {
        let mut key = [0u8; 32];
        let mut mac_key = [0u8; 32];

        rng.fill(&mut key);
        rng.fill(&mut mac_key);

        SymmetricCryptoKey {
            key: key.into(),
            mac_key: Some(mac_key.into()),
            _marker: PhantomData,
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

impl<Purpose: KeyPurpose> FromStr for SymmetricCryptoKey<Purpose> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = STANDARD.decode(s).map_err(|_| CryptoError::InvalidKey)?;
        SymmetricCryptoKey::try_from(bytes.as_slice())
    }
}

impl<Purpose: KeyPurpose> TryFrom<&[u8]> for SymmetricCryptoKey<Purpose> {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == Self::KEY_LEN + Self::MAC_LEN {
            Ok(SymmetricCryptoKey {
                key: GenericArray::clone_from_slice(&value[..Self::KEY_LEN]),
                mac_key: Some(GenericArray::clone_from_slice(&value[Self::KEY_LEN..])),
                _marker: PhantomData,
            })
        } else if value.len() == Self::KEY_LEN {
            Ok(SymmetricCryptoKey {
                key: GenericArray::clone_from_slice(value),
                mac_key: None,
                _marker: PhantomData,
            })
        } else {
            Err(CryptoError::InvalidKeyLen.into())
        }
    }
}

impl<Purpose: KeyPurpose> CryptoKey<Purpose> for SymmetricCryptoKey<Purpose> {}

// We manually implement these to make sure we don't print any sensitive data
impl<Purpose: KeyPurpose> std::fmt::Debug for SymmetricCryptoKey<Purpose> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SymmetricCryptoKey").finish()
    }
}

#[cfg(test)]
pub fn derive_symmetric_key(name: &str) -> SymmetricCryptoKey<purpose::Testing> {
    use crate::crypto::{derive_shareable_key, generate_random_bytes};

    let secret: [u8; 16] = generate_random_bytes();
    derive_shareable_key(secret, name, None).into()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::crypto::purpose;

    use super::{derive_symmetric_key, SymmetricCryptoKey};

    #[test]
    fn test_symmetric_crypto_key() {
        let key = derive_symmetric_key("test");
        let key2 = SymmetricCryptoKey::<purpose::Testing>::from_str(&key.to_base64()).unwrap();
        assert_eq!(key.key, key2.key);
        assert_eq!(key.mac_key, key2.mac_key);

        let key = "UY4B5N4DA4UisCNClgZtRr6VLy9ZF5BXXC7cDZRqourKi4ghEMgISbCsubvgCkHf5DZctQjVot11/vVvN9NNHQ==";
        let key2 = SymmetricCryptoKey::<purpose::Testing>::from_str(key).unwrap();
        assert_eq!(key, key2.to_base64());
    }
}
