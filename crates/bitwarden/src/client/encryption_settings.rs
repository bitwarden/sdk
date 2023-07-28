use std::{collections::HashMap, str::FromStr};

use aes::cipher::{
    block_padding::Pkcs7, generic_array::GenericArray, typenum::U32, BlockDecryptMut,
    BlockEncryptMut, KeyIvInit,
};
use base64::Engine;
use hmac::Mac;
use rand::RngCore;
use rsa::RsaPrivateKey;
use uuid::Uuid;

use crate::{
    crypto::{CipherString, PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE},
    error::{CryptoError, Error, Result},
    util::BASE64_ENGINE,
};

#[cfg(feature = "internal")]
use {
    crate::client::auth_settings::AuthSettings,
    rsa::{pkcs8::DecodePrivateKey, Oaep},
};

pub struct SymmetricCryptoKey {
    pub key: GenericArray<u8, U32>,
    pub mac_key: Option<GenericArray<u8, U32>>,
}

impl SymmetricCryptoKey {
    const KEY_LEN: usize = 32;
    const MAC_LEN: usize = 32;

    pub fn generate(name: &str) -> Self {
        use rand::Rng;
        let secret: [u8; 16] = rand::thread_rng().gen();
        crate::crypto::stretch_key(secret, name, None)
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

pub struct EncryptionSettings {
    user_key: SymmetricCryptoKey,
    private_key: Option<RsaPrivateKey>,
    org_keys: HashMap<Uuid, SymmetricCryptoKey>,
}

// We manually implement these to make sure we don't print any sensitive data
impl std::fmt::Debug for SymmetricCryptoKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key").finish()
    }
}
impl std::fmt::Debug for EncryptionSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncryptionSettings").finish()
    }
}

impl EncryptionSettings {
    #[cfg(feature = "internal")]
    pub(crate) fn new(
        auth: &AuthSettings,
        password: &str,
        user_key: CipherString,
        private_key: CipherString,
    ) -> Result<Self> {
        // Stretch keys from the provided password
        let (key, mac_key) = crate::crypto::stretch_key_password(
            password.as_bytes(),
            auth.email.as_bytes(),
            &auth.kdf,
        )?;

        // Decrypt the user key with the stretched key
        let user_key = {
            let (iv, mac, data) = match user_key {
                CipherString::AesCbc256_HmacSha256_B64 { iv, mac, data } => (iv, mac, data),
                _ => return Err(CryptoError::InvalidKey.into()),
            };

            let dec = decrypt_aes256(&iv, &mac, data, Some(mac_key), key)?;
            SymmetricCryptoKey::try_from(dec.as_slice())?
        };

        // Decrypt the private key with the user key
        let private_key = {
            let dec = decrypt(&private_key, &user_key)?;
            Some(rsa::RsaPrivateKey::from_pkcs8_der(&dec).map_err(|_| CryptoError::InvalidKey)?)
        };

        Ok(EncryptionSettings {
            user_key,
            private_key,
            org_keys: HashMap::new(),
        })
    }

    pub(crate) fn new_single_key(key: SymmetricCryptoKey) -> Self {
        EncryptionSettings {
            user_key: key,
            private_key: None,
            org_keys: HashMap::new(),
        }
    }

    #[cfg(feature = "internal")]
    pub(crate) fn set_org_keys(
        &mut self,
        org_enc_keys: Vec<(Uuid, CipherString)>,
    ) -> Result<&mut Self> {
        let private_key = self.private_key.as_ref().ok_or(Error::VaultLocked)?;

        // Decrypt the org keys with the private key
        for (org_id, org_enc_key) in org_enc_keys {
            let data = match org_enc_key {
                CipherString::Rsa2048_OaepSha1_B64 { data } => data,
                _ => return Err(CryptoError::InvalidKey.into()),
            };

            let dec = private_key
                .decrypt(Oaep::new::<sha1::Sha1>(), &data)
                .map_err(|_| CryptoError::KeyDecrypt)?;

            let org_key = SymmetricCryptoKey::try_from(dec.as_slice())?;

            self.org_keys.insert(org_id, org_key);
        }

        Ok(self)
    }

    fn get_key(&self, org_id: &Option<Uuid>) -> Option<&SymmetricCryptoKey> {
        // If we don't have a private key set (to decode multiple org keys), we just use the main user key
        if self.private_key.is_none() {
            return Some(&self.user_key);
        }

        match org_id {
            Some(org_id) => self.org_keys.get(org_id),
            None => Some(&self.user_key),
        }
    }

    pub(crate) fn decrypt(&self, cipher: &CipherString, org_id: &Option<Uuid>) -> Result<String> {
        let key = self.get_key(org_id).ok_or(CryptoError::NoKeyForOrg)?;
        let dec = decrypt(cipher, key)?;
        String::from_utf8(dec).map_err(|_| CryptoError::InvalidUtf8String.into())
    }

    pub(crate) fn encrypt(&self, data: &[u8], org_id: &Option<Uuid>) -> Result<CipherString> {
        let key = self.get_key(org_id).ok_or(CryptoError::NoKeyForOrg)?;

        let dec = encrypt_aes256(data, key.mac_key, key.key)?;
        Ok(dec)
    }
}

pub fn decrypt(cipher: &CipherString, key: &SymmetricCryptoKey) -> Result<Vec<u8>> {
    match cipher {
        CipherString::AesCbc256_HmacSha256_B64 { iv, mac, data } => {
            let dec = decrypt_aes256(iv, mac, data.clone(), key.mac_key, key.key)?;
            Ok(dec)
        }
        _ => Err(CryptoError::InvalidKey.into()),
    }
}

pub fn decrypt_aes256(
    iv: &[u8; 16],
    mac: &[u8; 32],
    data: Vec<u8>,
    mac_key: Option<GenericArray<u8, U32>>,
    key: GenericArray<u8, U32>,
) -> Result<Vec<u8>> {
    let mac_key = match mac_key {
        Some(k) => k,
        None => return Err(CryptoError::InvalidMac.into()),
    };

    // Validate HMAC
    let res = validate_mac(&mac_key, iv, &data)?;
    if res != *mac {
        return Err(CryptoError::InvalidMac.into());
    }

    // Decrypt data
    let iv = GenericArray::from_slice(iv);
    let mut data = data;
    let decrypted_key_slice = cbc::Decryptor::<aes::Aes256>::new(&key, iv)
        .decrypt_padded_mut::<Pkcs7>(&mut data)
        .map_err(|_| CryptoError::KeyDecrypt)?;

    //Data is decrypted in place and returns a subslice of the original Vec, to avoid cloning it, we truncate to the subslice length
    let decrypted_len = decrypted_key_slice.len();
    data.truncate(decrypted_len);

    Ok(data)
}

pub fn encrypt_aes256(
    data_dec: &[u8],
    mac_key: Option<GenericArray<u8, U32>>,
    key: GenericArray<u8, U32>,
) -> Result<CipherString> {
    let mac_key = match mac_key {
        Some(k) => k,
        None => return Err(CryptoError::InvalidMac.into()),
    };

    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    let data = cbc::Encryptor::<aes::Aes256>::new(&key, &iv.into())
        .encrypt_padded_vec_mut::<Pkcs7>(data_dec);

    let mac = validate_mac(&mac_key, &iv, &data)?;

    Ok(CipherString::AesCbc256_HmacSha256_B64 { iv, mac, data })
}

fn validate_mac(mac_key: &[u8], iv: &[u8], data: &[u8]) -> Result<[u8; 32]> {
    let mut hmac = PbkdfSha256Hmac::new_from_slice(mac_key).expect("HMAC can take key of any size");
    hmac.update(iv);
    hmac.update(data);
    let mac: [u8; PBKDF_SHA256_HMAC_OUT_SIZE] = (*hmac.finalize().into_bytes())
        .try_into()
        .map_err(|_| CryptoError::InvalidMac)?;

    Ok(mac)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{EncryptionSettings, SymmetricCryptoKey};
    use crate::crypto::{Decryptable, Encryptable};

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

    #[test]
    fn test_encryption_settings() {
        let key = SymmetricCryptoKey::generate("test");
        let settings = EncryptionSettings::new_single_key(key);

        let test_string = "encrypted_test_string".to_string();
        let cipher = test_string.clone().encrypt(&settings, &None).unwrap();

        let decrypted_str = cipher.decrypt(&settings, &None).unwrap();
        assert_eq!(decrypted_str, test_string);
    }
}
