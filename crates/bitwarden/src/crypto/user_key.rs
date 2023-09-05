use crate::{
    crypto::{
        rsa::{make_key_pair, RsaKeyPair},
        SymmetricCryptoKey,
    },
    error::Result,
};

pub(crate) struct UserKey(SymmetricCryptoKey);

impl UserKey {
    pub(crate) fn new(key: SymmetricCryptoKey) -> Self {
        Self(key)
    }

    pub(crate) fn make_key_pair(&self) -> Result<RsaKeyPair> {
        make_key_pair(&self.0)
    }
}
