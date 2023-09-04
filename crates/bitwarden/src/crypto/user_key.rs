use base64::Engine;
use rsa::{
    pkcs8::{EncodePrivateKey, EncodePublicKey},
    RsaPrivateKey, RsaPublicKey,
};

use crate::{
    crypto::{EncString, SymmetricCryptoKey},
    error::{Error, Result},
    util::BASE64_ENGINE,
};

use super::encrypt_aes256_hmac;

pub(crate) struct UserKey(SymmetricCryptoKey);

impl UserKey {
    pub(crate) fn new(key: SymmetricCryptoKey) -> Self {
        Self(key)
    }

    pub(crate) fn make_key_pair(&self) -> Result<(String, EncString)> {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        let spki = pub_key
            .to_public_key_der()
            .map_err(|_| Error::Internal("unable to create public key"))?;

        let b64 = BASE64_ENGINE.encode(spki.as_bytes());
        let pkcs = priv_key
            .to_pkcs8_der()
            .map_err(|_| Error::Internal("unable to create private key"))?;

        let protected = encrypt_aes256_hmac(pkcs.as_bytes(), self.0.mac_key.unwrap(), self.0.key)?;

        Ok((b64, protected))
    }
}
