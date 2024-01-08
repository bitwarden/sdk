use crate::{
    rsa::{make_key_pair, RsaKeyPair},
    Result, SymmetricCryptoKey,
};

/// User Key
///
/// The User Key is the symmetric encryption key used to decrypt the user's vault.
pub struct UserKey(pub SymmetricCryptoKey);

impl UserKey {
    pub fn new(key: SymmetricCryptoKey) -> Self {
        Self(key)
    }

    pub fn make_key_pair(&self) -> Result<RsaKeyPair> {
        make_key_pair(&self.0)
    }
}
