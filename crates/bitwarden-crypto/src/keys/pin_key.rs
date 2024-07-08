use crate::{
    keys::{
        key_encryptable::CryptoKey,
        utils::{derive_kdf_key, stretch_kdf_key},
    },
    CryptoError, EncString, Kdf, KeyDecryptable, KeyEncryptable, Result, SymmetricCryptoKey,
};

/// Pin Key.
///
/// Derived from a specific password, used for pin encryption and exports.
pub struct PinKey(SymmetricCryptoKey);

impl PinKey {
    pub fn new(key: SymmetricCryptoKey) -> Self {
        Self(key)
    }

    /// Derives a users pin key from their password, email and KDF.
    pub fn derive(password: &[u8], email: &[u8], kdf: &Kdf) -> Result<Self> {
        derive_kdf_key(password, email, kdf).map(Self)
    }

    /// Decrypt the users user key
    pub fn decrypt_user_key(&self, user_key: EncString) -> Result<SymmetricCryptoKey> {
        let mut dec: Vec<u8> = match user_key {
            // Legacy. user_keys were encrypted using `AesCbc256_B64` a long time ago. We've since
            // moved to using `AesCbc256_HmacSha256_B64`. However, we still need to support
            // decrypting these old keys.
            EncString::AesCbc256_B64 { .. } => user_key.decrypt_with_key(&self.0)?,
            _ => {
                let stretched_key = stretch_kdf_key(&self.0)?;
                user_key.decrypt_with_key(&stretched_key)?
            }
        };

        SymmetricCryptoKey::try_from(dec.as_mut_slice())
    }

    pub fn encrypt_user_key(&self, user_key: &SymmetricCryptoKey) -> Result<EncString> {
        let stretched_key = stretch_kdf_key(&self.0)?;

        EncString::encrypt_aes256_hmac(
            &user_key.to_vec(),
            stretched_key
                .mac_key
                .as_ref()
                .ok_or(CryptoError::InvalidMac)?,
            &stretched_key.key,
        )
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
