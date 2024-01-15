use base64::{engine::general_purpose::STANDARD, Engine};
use rsa::{
    pkcs8::{EncodePrivateKey, EncodePublicKey},
    Oaep, RsaPrivateKey, RsaPublicKey,
};
use sha1::Sha1;

use crate::{
    error::{Result, RsaError},
    CryptoError, EncString, SymmetricCryptoKey,
};

/// RSA Key Pair
///
/// Consists of a public key and an encrypted private key.
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct RsaKeyPair {
    /// Base64 encoded DER representation of the public key
    pub public: String,
    /// Encrypted PKCS8 private key
    pub private: EncString,
}

pub(super) fn make_key_pair(key: &SymmetricCryptoKey) -> Result<RsaKeyPair> {
    let priv_key = generate_rsa();
    let pub_key = RsaPublicKey::from(&priv_key);

    let spki = pub_key
        .to_public_key_der()
        .map_err(|_| RsaError::CreatePublicKey)?;

    let b64 = STANDARD.encode(spki.as_bytes());
    let pkcs = priv_key
        .to_pkcs8_der()
        .map_err(|_| RsaError::CreatePrivateKey)?;

    let protected = EncString::encrypt_aes256_hmac(pkcs.as_bytes(), key.mac_key.unwrap(), key.key)?;

    Ok(RsaKeyPair {
        public: b64,
        private: protected,
    })
}

pub(super) fn encrypt_rsa2048_oaep_sha1(
    private_key: &RsaPrivateKey,
    data: &[u8],
) -> Result<Vec<u8>> {
    let mut rng = rand::thread_rng();

    let padding = Oaep::new::<Sha1>();
    private_key
        .to_public_key()
        .encrypt(&mut rng, padding, data)
        .map_err(|e| CryptoError::RsaError(e.into()))
}

// TODO: Move this to AsymmCryptoKey
/// Generate a new random AsymmetricCryptoKey (RSA-2048)
pub(crate) fn generate_rsa() -> RsaPrivateKey {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key")
}
