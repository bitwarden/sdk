use base64::Engine;
use bitwarden_crypto::symmetric_crypto_key::SymmetricCryptoKey;
use rsa::{
    pkcs8::{EncodePrivateKey, EncodePublicKey},
    RsaPrivateKey, RsaPublicKey,
};

use crate::{
    crypto::EncString,
    error::{Error, Result},
    util::BASE64_ENGINE,
};

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct RsaKeyPair {
    /// Base64 encoded DER representation of the public key
    pub public: String,
    /// Encrypted PKCS8 private key
    pub private: EncString,
}

pub(super) fn make_key_pair(key: &SymmetricCryptoKey) -> Result<RsaKeyPair> {
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

    let protected = EncString::encrypt_aes256_hmac(pkcs.as_bytes(), key.mac_key.unwrap(), key.key)?;

    Ok(RsaKeyPair {
        public: b64,
        private: protected,
    })
}
