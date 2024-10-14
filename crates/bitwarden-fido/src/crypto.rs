use coset::{iana, CoseKey};
use p256::{
    pkcs8::{DecodePrivateKey, EncodePrivateKey},
    SecretKey,
};
use passkey::authenticator::{private_key_from_cose_key, CoseKeyPair};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoseKeyToPkcs8Error {
    #[error("Failed to extract private key from cose_key")]
    FailedToExtractPrivateKeyFromCoseKey,
    #[error("Failed to convert P256 private key to PKC8")]
    FailedToConvertP256PrivateKeyToPkcs8,
}

pub(crate) fn cose_key_to_pkcs8(cose_key: &CoseKey) -> Result<Vec<u8>, CoseKeyToPkcs8Error> {
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
    let secret_key = SecretKey::from_pkcs8_der(secret_key).map_err(|error| {
        log::error!("Failed to extract private key from secret_key: {:?}", error);
        PrivateKeyFromSecretKeyError
    })?;

    let cose_key_pair = CoseKeyPair::from_secret_key(&secret_key, iana::Algorithm::ES256);
    Ok(cose_key_pair.private)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn private_key_for_testing() -> CoseKey {
        // Hardcoded CoseKey for testing purposes
        let bytes = vec![
            166, 1, 2, 3, 38, 32, 1, 33, 88, 32, 200, 30, 161, 146, 196, 121, 165, 149, 92, 232,
            49, 48, 245, 253, 73, 234, 204, 3, 209, 153, 166, 77, 59, 232, 70, 16, 206, 77, 84,
            156, 28, 77, 34, 88, 32, 82, 141, 165, 28, 241, 82, 31, 33, 183, 206, 29, 91, 93, 111,
            216, 216, 26, 62, 211, 49, 191, 86, 238, 118, 241, 124, 131, 106, 214, 95, 170, 160,
            35, 88, 32, 147, 171, 4, 49, 68, 170, 47, 51, 74, 211, 94, 40, 212, 244, 95, 55, 154,
            92, 171, 241, 0, 55, 84, 151, 79, 244, 151, 198, 135, 45, 97, 238,
        ];

        <CoseKey as coset::CborSerializable>::from_slice(bytes.as_slice()).unwrap()
    }

    #[test]
    fn test_cose_key_to_pkcs8_and_back() {
        let cose_key = private_key_for_testing();

        let pkcs8 = cose_key_to_pkcs8(&cose_key).expect("CoseKey to PKCS8 failed");
        let cose_key2 = pkcs8_to_cose_key(&pkcs8).expect("PKCS8 to CoseKey failed");

        assert_eq!(cose_key, cose_key2);
    }

    fn pkcs8_key_for_testing() -> Vec<u8> {
        vec![
            0x30, 0x81, 0x87, 0x02, 0x01, 0x00, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce,
            0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, 0x04,
            0x6d, 0x30, 0x6b, 0x02, 0x01, 0x01, 0x04, 0x20, 0x06, 0x76, 0x5e, 0x85, 0xe0, 0x7f,
            0xef, 0x43, 0xaa, 0x17, 0xe0, 0x7a, 0xd7, 0x85, 0x63, 0x01, 0x80, 0x70, 0x8c, 0x6c,
            0x61, 0x43, 0x7d, 0xc3, 0xb1, 0xe6, 0xf9, 0x09, 0x24, 0xeb, 0x1f, 0xf5, 0xa1, 0x44,
            0x03, 0x42, 0x00, 0x04, 0x35, 0x9a, 0x52, 0xf3, 0x82, 0x44, 0x66, 0x5f, 0x3f, 0xe2,
            0xc4, 0x0b, 0x1c, 0x16, 0x34, 0xc5, 0x60, 0x07, 0x3a, 0x25, 0xfe, 0x7e, 0x7f, 0x7f,
            0xda, 0xd4, 0x1c, 0x36, 0x90, 0x00, 0xee, 0xb1, 0x8e, 0x92, 0xb3, 0xac, 0x91, 0x7f,
            0xb1, 0x8c, 0xa4, 0x85, 0xe7, 0x03, 0x07, 0xd1, 0xf5, 0x5b, 0xd3, 0x7b, 0xc3, 0x56,
            0x11, 0xdf, 0xbc, 0x7a, 0x97, 0x70, 0x32, 0x4b, 0x3c, 0x84, 0x05, 0x71,
        ]
    }

    #[test]
    fn test_pkcs8_to_cose_key_and_back() {
        let pkcs8 = pkcs8_key_for_testing();

        let cose_key = pkcs8_to_cose_key(&pkcs8).expect("PKCS8 to CoseKey failed");
        let pkcs8_2 = cose_key_to_pkcs8(&cose_key).expect("CoseKey to PKCS8 failed");

        assert_eq!(pkcs8, pkcs8_2);
    }
}
