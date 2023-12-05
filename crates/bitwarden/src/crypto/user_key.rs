use bitwarden_crypto::symmetric_crypto_key::SymmetricCryptoKey;

use crate::{
    crypto::rsa::{make_key_pair, RsaKeyPair},
    error::Result,
};

pub(crate) struct UserKey(pub(super) SymmetricCryptoKey);

impl UserKey {
    pub(crate) fn new(key: SymmetricCryptoKey) -> Self {
        Self(key)
    }

    pub(crate) fn make_key_pair(&self) -> Result<RsaKeyPair> {
        make_key_pair(&self.0)
    }
}
