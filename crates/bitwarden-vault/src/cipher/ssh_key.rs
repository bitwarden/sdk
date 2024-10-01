use bitwarden_crypto::{
    CryptoError, EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SshKey {
    /// Ssh private key (ed25519/rsa) in unencrypted openssh private key format https://github.com/openssh/openssh-portable/blob/master/PROTOCOL.key
    pub private_key: Option<EncString>,
    /// Ssh public key (ed25519/rsa) according to RFC4253 https://datatracker.ietf.org/doc/html/rfc4253#section-6.6
    pub public_key: Option<EncString>,
    /// Ssh fingerprint using SHA256 in the format: SHA256:BASE64_ENCODED_FINGERPRINT
    pub fingerprint: Option<EncString>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SshKeyView {
    /// Ssh private key (ed25519/rsa) in unencrypted openssh private key format https://github.com/openssh/openssh-portable/blob/master/PROTOCOL.key
    pub private_key: Option<String>,
    /// Ssh public key (ed25519/rsa) according to RFC4253 https://datatracker.ietf.org/doc/html/rfc4253#section-6.6
    pub public_key: Option<String>,
    /// Ssh fingerprint using SHA256 in the format: SHA256:BASE64_ENCODED_FINGERPRINT
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
