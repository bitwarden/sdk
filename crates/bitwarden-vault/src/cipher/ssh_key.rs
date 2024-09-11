use bitwarden_crypto::{
    CryptoError, EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SshKey {
    pub private_key: Option<EncString>,
    pub public_key: Option<EncString>,
    pub fingerprint: Option<EncString>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SshKeyView {
    pub private_key: Option<String>,
    pub public_key: Option<String>,
    pub fingerprint: Option<String>,
}

impl KeyEncryptable<SymmetricCryptoKey, SshKey> for SshKeyView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<SshKey, CryptoError> {
        Ok(SshKey {
            private_key: self.private_key.encrypt_with_key(key)?,
            public_key: self.public_key.encrypt_with_key(key)?,
            fingerprint: self.fingerprint.encrypt_with_key(key)?,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, SshKeyView> for SshKey {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<SshKeyView, CryptoError> {
        Ok(SshKeyView {
            private_key: self.private_key.decrypt_with_key(key).ok().flatten(),
            public_key: self.public_key.decrypt_with_key(key).ok().flatten(),
            fingerprint: self.fingerprint.decrypt_with_key(key).ok().flatten(),
        })
    }
}
