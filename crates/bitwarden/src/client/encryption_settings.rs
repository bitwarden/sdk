use std::collections::HashMap;

use rsa::RsaPrivateKey;
use uuid::Uuid;
#[cfg(feature = "internal")]
use {
    crate::{client::UserLoginMethod, crypto::KeyDecryptable},
    rsa::{pkcs8::DecodePrivateKey, Oaep},
};

use crate::{
    crypto::{encrypt_aes256_hmac, EncString, SymmetricCryptoKey},
    error::{CryptoError, Result},
};

pub struct EncryptionSettings {
    user_key: SymmetricCryptoKey,
    private_key: Option<RsaPrivateKey>,
    org_keys: HashMap<Uuid, SymmetricCryptoKey>,
}

impl std::fmt::Debug for EncryptionSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncryptionSettings").finish()
    }
}

impl EncryptionSettings {
    #[cfg(feature = "internal")]
    pub(crate) fn new(
        login_method: &UserLoginMethod,
        password: &str,
        user_key: EncString,
        private_key: EncString,
    ) -> Result<Self> {
        use crate::crypto::MasterKey;

        match login_method {
            UserLoginMethod::Username { email, kdf, .. }
            | UserLoginMethod::ApiKey { email, kdf, .. } => {
                // Derive master key from password
                let master_key = MasterKey::derive(password.as_bytes(), email.as_bytes(), kdf)?;

                // Decrypt the user key
                let user_key = master_key.decrypt_user_key(user_key)?;

                // Decrypt the private key with the user key
                let private_key = {
                    let dec: Vec<u8> = private_key.decrypt_with_key(&user_key)?;
                    Some(
                        rsa::RsaPrivateKey::from_pkcs8_der(&dec)
                            .map_err(|_| CryptoError::InvalidKey)?,
                    )
                };

                Ok(EncryptionSettings {
                    user_key,
                    private_key,
                    org_keys: HashMap::new(),
                })
            }
        }
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
        org_enc_keys: Vec<(Uuid, EncString)>,
    ) -> Result<&mut Self> {
        use crate::error::Error;

        let private_key = self.private_key.as_ref().ok_or(Error::VaultLocked)?;

        // Decrypt the org keys with the private key
        for (org_id, org_enc_key) in org_enc_keys {
            let data = match org_enc_key {
                EncString::Rsa2048_OaepSha1_B64 { data } => data,
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

    pub(crate) fn get_key(&self, org_id: &Option<Uuid>) -> Option<&SymmetricCryptoKey> {
        // If we don't have a private key set (to decode multiple org keys), we just use the main user key
        if self.private_key.is_none() {
            return Some(&self.user_key);
        }

        match org_id {
            Some(org_id) => self.org_keys.get(org_id),
            None => Some(&self.user_key),
        }
    }

    pub(crate) fn encrypt(&self, data: &[u8], org_id: &Option<Uuid>) -> Result<EncString> {
        let key = self.get_key(org_id).ok_or(CryptoError::NoKeyForOrg)?;

        let dec = encrypt_aes256_hmac(data, key.mac_key.ok_or(CryptoError::InvalidMac)?, key.key)?;
        Ok(dec)
    }
}

#[cfg(test)]
mod tests {
    use super::SymmetricCryptoKey;
    use crate::crypto::{KeyDecryptable, KeyEncryptable};

    #[test]
    fn test_encryption_settings() {
        let key = SymmetricCryptoKey::generate("test");

        let test_string = "encrypted_test_string".to_string();
        let cipher = test_string.clone().encrypt_with_key(&key).unwrap();

        let decrypted_str: String = cipher.decrypt_with_key(&key).unwrap();
        assert_eq!(decrypted_str, test_string);
    }
}
