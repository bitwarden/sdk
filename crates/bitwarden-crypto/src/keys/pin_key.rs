use crate::{
    keys::{
        key_encryptable::CryptoKey,
        utils::{derive_kdf_key, stretch_kdf_key},
    },
    EncString, Kdf, KeyEncryptable, Result, SymmetricCryptoKey,
};

/// Pin Key.
///
/// Derived from a specific password, used for pin encryption and exports.
pub struct PinKey(SymmetricCryptoKey);

impl PinKey {
    pub fn new(key: SymmetricCryptoKey) -> Self {
        Self(key)
    }

    /// Derives a users master key from their password, email and KDF.
    pub fn derive(password: &[u8], salt: &[u8], kdf: &Kdf) -> Result<Self> {
        derive_kdf_key(password, salt, kdf).map(Self)
    }
}

impl CryptoKey for PinKey {}

impl KeyEncryptable<PinKey, EncString> for &[u8] {
    fn encrypt_with_key(self, key: &PinKey) -> Result<EncString> {
        let stretched_key = stretch_kdf_key(&key.0)?;

        self.encrypt_with_key(&stretched_key)
    }
}

impl KeyEncryptable<PinKey, EncString> for String {
    fn encrypt_with_key(self, key: &PinKey) -> Result<EncString> {
        self.as_bytes().encrypt_with_key(key)
    }
}
