use std::collections::HashMap;

#[cfg(feature = "internal")]
use crate::{
    client::UserLoginMethod,
    crypto::{AsymmEncString, EncString, KeyDecryptable},
    error::Result,
};
use uuid::Uuid;

use crate::crypto::{purpose, AsymmetricCryptoKey, SymmetricCryptoKey};

pub struct EncryptionSettings {
    user_key: SymmetricCryptoKey<purpose::UserEncryption>,
    pub(crate) private_key: Option<AsymmetricCryptoKey<purpose::OrgEncryption>>,
    org_keys: HashMap<Uuid, SymmetricCryptoKey<purpose::OrgEncryption>>,
}

impl std::fmt::Debug for EncryptionSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncryptionSettings").finish()
    }
}

impl EncryptionSettings {
    /// Initialize the encryption settings with the user password and their encrypted keys
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

                Self::new_decrypted_key(user_key, private_key)
            }
        }
    }

    /// Initialize the encryption settings with the decrypted user key and the encrypted user private key
    /// This should only be used when unlocking the vault via biometrics or when the vault is set to lock: "never"
    /// Otherwise handling the decrypted user key is dangerous and discouraged
    #[cfg(feature = "internal")]
    pub(crate) fn new_decrypted_key(
        user_key: SymmetricCryptoKey<purpose::UserEncryption>,
        private_key: EncString,
    ) -> Result<Self> {
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
    pub(crate) fn new_single_key(key: SymmetricCryptoKey<purpose::UserEncryption>) -> Self {
        EncryptionSettings {
            user_key: key,
            private_key: None,
            org_keys: HashMap::new(),
        }
    }

    #[cfg(feature = "internal")]
    pub(crate) fn set_org_keys(
        &mut self,
        org_enc_keys: Vec<(Uuid, AsymmEncString)>,
    ) -> Result<&mut Self> {
        use crate::error::Error;

        let private_key = self.private_key.as_ref().ok_or(Error::VaultLocked)?;

        // Make sure we only keep the keys given in the arguments and not any of the previous
        // ones, which might be from organizations that the user is no longer a part of anymore
        self.org_keys.clear();

        // Decrypt the org keys with the private key
        for (org_id, org_enc_key) in org_enc_keys {
            let dec: Vec<u8> = org_enc_key.decrypt_with_key(private_key)?;

            let org_key = SymmetricCryptoKey::try_from(dec.as_slice())?;

            self.org_keys.insert(org_id, org_key);
        }

        Ok(self)
    }

    pub(crate) fn get_user_key(&self) -> Option<&SymmetricCryptoKey<purpose::UserEncryption>> {
        Some(&self.user_key)
    }

    pub(crate) fn get_org_key(
        &self,
        org_id: Uuid,
    ) -> Option<&SymmetricCryptoKey<purpose::OrgEncryption>> {
        // TODO: This hack is needed because EncryptionSettings mixes the concept of the single org key in secrets manager as the user key
        if self.private_key.is_none() {
            return Some((&self.user_key).into());
        }
        self.org_keys.get(&org_id)
    }
}
