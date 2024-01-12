use crate::{
    crypto::{
        rsa::{make_key_pair, RsaKeyPair},
        SymmetricCryptoKey,
    },
    error::Result,
};

use super::purpose;

pub(crate) struct UserKey(pub(super) SymmetricCryptoKey<purpose::UserEncryption>);

impl UserKey {
    pub(crate) fn new(key: SymmetricCryptoKey<purpose::UserEncryption>) -> Self {
        Self(key)
    }

    pub(crate) fn make_key_pair(&self) -> Result<RsaKeyPair> {
        make_key_pair(&self.0)
    }
}
