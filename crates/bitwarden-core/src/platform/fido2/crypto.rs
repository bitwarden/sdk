use coset::{iana, CoseKey};
use p256::{pkcs8::EncodePrivateKey, SecretKey};
use passkey::authenticator::{private_key_from_cose_key, CoseKeyPair};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoseKeyToPkcs8Error {
    #[error("Failed to extract private key from cose_key")]
    FailedToExtractPrivateKeyFromCoseKey,
    #[error("Failed to convert P256 private key to PKC8")]
    FailedToConvertP256PrivateKeyToPkcs8,
}

pub fn cose_key_to_pkcs8(cose_key: &CoseKey) -> Result<Vec<u8>, CoseKeyToPkcs8Error> {
    // cose_key.
    let secret_key = private_key_from_cose_key(cose_key).map_err(|error| {
        log::error!("Failed to extract private key from cose_key: {:?}", error);
        CoseKeyToPkcs8Error::FailedToExtractPrivateKeyFromCoseKey
    })?;

    let vec = secret_key
        .to_pkcs8_der()
        .map_err(|error| {
            log::error!("Failed to convert P256 private key to PKC8: {:?}", error);
            CoseKeyToPkcs8Error::FailedToConvertP256PrivateKeyToPkcs8
        })?
        .as_bytes()
        .to_vec();

    Ok(vec)
}

#[derive(Debug, Error)]
#[error("Failed to extract private key from secret_key")]
pub struct PrivateKeyFromSecretKeyError;

pub fn pkcs8_to_cose_key(secret_key: &[u8]) -> Result<CoseKey, PrivateKeyFromSecretKeyError> {
    let secret_key = SecretKey::from_slice(secret_key).map_err(|error| {
        log::error!("Failed to extract private key from secret_key: {:?}", error);
        PrivateKeyFromSecretKeyError
    })?;

    let cose_key_pair = CoseKeyPair::from_secret_key(&secret_key, iana::Algorithm::ES256);
    Ok(cose_key_pair.private)
}
