use std::collections::HashMap;

use bitwarden_crypto::{AsymmetricCryptoKey, KeyContainer, SymmetricCryptoKey};
#[cfg(feature = "internal")]
use bitwarden_crypto::{AsymmetricEncString, EncString, MasterKey};
use uuid::Uuid;

#[cfg(feature = "internal")]
use crate::error::Result;

pub struct EncryptionSettings {
    user_key: SymmetricCryptoKey,
    pub(crate) private_key: Option<AsymmetricCryptoKey>,
    org_keys: HashMap<Uuid, SymmetricCryptoKey>,
}

impl std::fmt::Debug for EncryptionSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncryptionSettings").finish()
    }
}

impl EncryptionSettings {
    /// Initialize the encryption settings with the master key and the encrypted user keys
    #[cfg(feature = "internal")]
    pub(crate) fn new(
        master_key: MasterKey,
        user_key: EncString,
        private_key: EncString,
    ) -> Result<Self> {
        // Decrypt the user key
        let user_key = master_key.decrypt_user_key(user_key)?;
        Self::new_decrypted_key(user_key, private_key)
    }

    /// Initialize the encryption settings with the decrypted user key and the encrypted user
    /// private key This should only be used when unlocking the vault via biometrics or when the
    /// vault is set to lock: "never" Otherwise handling the decrypted user key is dangerous and
    /// discouraged
    #[cfg(feature = "internal")]
    pub(crate) fn new_decrypted_key(
        user_key: SymmetricCryptoKey,
        private_key: EncString,
    ) -> Result<Self> {
        use bitwarden_crypto::KeyDecryptable;

        let private_key = {
            let dec: Vec<u8> = private_key.decrypt_with_key(&user_key)?;
            Some(AsymmetricCryptoKey::from_der(&dec)?)
        };

        Ok(EncryptionSettings {
            user_key,
            private_key,
            org_keys: HashMap::new(),
        })
    }

    /// Initialize the encryption settings with only a single decrypted key.
    /// This is used only for logging in Secrets Manager with an access token
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
        org_enc_keys: Vec<(Uuid, AsymmetricEncString)>,
    ) -> Result<&mut Self> {
        use bitwarden_core::VaultLocked;
        use bitwarden_crypto::KeyDecryptable;

        let private_key = self.private_key.as_ref().ok_or(VaultLocked())?;

        // Make sure we only keep the keys given in the arguments and not any of the previous
        // ones, which might be from organizations that the user is no longer a part of anymore
        self.org_keys.clear();

        // Decrypt the org keys with the private key
        for (org_id, org_enc_key) in org_enc_keys {
            let mut dec: Vec<u8> = org_enc_key.decrypt_with_key(private_key)?;

            let org_key = SymmetricCryptoKey::try_from(dec.as_mut_slice())?;

            self.org_keys.insert(org_id, org_key);
        }

        Ok(self)
    }

    pub(crate) fn get_key(&self, org_id: &Option<Uuid>) -> Option<&SymmetricCryptoKey> {
        // If we don't have a private key set (to decode multiple org keys), we just use the main
        // user key
        if self.private_key.is_none() {
            return Some(&self.user_key);
        }

        match org_id {
            Some(org_id) => self.org_keys.get(org_id),
            None => Some(&self.user_key),
        }
    }
}

impl KeyContainer for EncryptionSettings {
    fn get_key(&self, org_id: &Option<Uuid>) -> Option<&SymmetricCryptoKey> {
        EncryptionSettings::get_key(self, org_id)
    }
}
