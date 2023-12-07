use crate::{
    rsa::{make_key_pair, RsaKeyPair},
    Result, SymmetricCryptoKey,
};

pub struct UserKey(pub SymmetricCryptoKey);

impl UserKey {
    pub fn new(key: SymmetricCryptoKey) -> Self {
        Self(key)
    }

    pub fn make_key_pair(&self) -> Result<RsaKeyPair> {
        make_key_pair(&self.0)
    }
}
