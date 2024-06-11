use coset::{
    iana::{self},
    CoseKey,
};
use p256::{pkcs8::EncodePrivateKey, SecretKey};
use passkey::authenticator::{private_key_from_cose_key, CoseKeyPair};

use crate::error::{Error, Result};

pub fn cose_key_to_pkcs8(cose_key: &CoseKey) -> Result<Vec<u8>> {
    // cose_key.
    let secret_key = private_key_from_cose_key(cose_key).map_err(|error| {
        log::error!("Failed to extract private key from cose_key: {:?}", error);
        Error::Internal("Failed to extract private key from cose_key".into())
    })?;

    let vec = secret_key
        .to_pkcs8_der()
        .map_err(|error| {
            log::error!("Failed to convert P256 private key to PKC8: {:?}", error);
            Error::Internal("Failed to convert P256 private key to PKC8".into())
        })?
        .as_bytes()
        .to_vec();

    Ok(vec)
}

pub fn pkcs8_to_cose_key(secret_key: &[u8]) -> Result<CoseKey> {
    let secret_key = SecretKey::from_slice(secret_key).map_err(|error| {
        log::error!("Failed to extract private key from secret_key: {:?}", error);
        Error::Internal("Failed to extract private key from secret_key".into())
    })?;

    let cose_key_pair = CoseKeyPair::from_secret_key(&secret_key, iana::Algorithm::ES256);
    Ok(cose_key_pair.private)
}
