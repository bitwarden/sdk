use base64::Engine;
use rsa::{
    pkcs8::{
        der::Decode, DecodePrivateKey, EncodePrivateKey, EncodePublicKey, SubjectPublicKeyInfo,
    },
    Oaep, RsaPrivateKey, RsaPublicKey,
};
use sha1::Sha1;

use crate::{
    crypto::{encrypt_aes256_hmac, EncString, SymmetricCryptoKey},
    error::{CryptoError, Error, Result},
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

    let protected = encrypt_aes256_hmac(pkcs.as_bytes(), key.mac_key.unwrap(), key.key)?;

    Ok(RsaKeyPair {
        public: b64,
        private: protected,
    })
}

pub(super) fn decrypt_rsa(data: Vec<u8>, key: &RsaPrivateKey) -> Result<Vec<u8>> {
    key.decrypt(Oaep::new::<Sha1>(), &data)
        .map_err(|_| CryptoError::InvalidKey.into()) // need better error
}

pub fn encrypt_rsa(data: Vec<u8>, key: &RsaPublicKey) -> Result<Vec<u8>> {
    let mut rng = rand::thread_rng();
    key.encrypt(&mut rng, Oaep::new::<Sha1>(), &data)
        .map_err(|_| CryptoError::InvalidKey.into()) // need better error
}

pub fn public_key_from_b64(b64: &str) -> Result<RsaPublicKey> {
    let public_key_bytes = BASE64_ENGINE.decode(b64)?;
    let public_key_info = SubjectPublicKeyInfo::from_der(&public_key_bytes).unwrap(); // TODO: error handling
    RsaPublicKey::try_from(public_key_info).map_err(|_| Error::Crypto(CryptoError::InvalidKey))
}

pub fn private_key_from_bytes(bytes: &Vec<u8>) -> Result<RsaPrivateKey> {
    rsa::RsaPrivateKey::from_pkcs8_der(bytes).map_err(|_| Error::Crypto(CryptoError::InvalidKey))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::BASE64_ENGINE;
    use base64::Engine;
    use rsa::pkcs8::{der::Decode, DecodePrivateKey, SubjectPublicKeyInfo};

    const PRIVATE_KEY_B64: &str = concat!(
    "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCXRVrCX+2hfOQS8Hz",
    "YUS2oc/jGVTZpv+/Ryuoh9d8ihYX9dd0cYh2tl6KWdFc88lPUH11Oxqy20Rk2e5r/RF6T9yM0Me3NPnaKt+hlhLtfoc0h86L",
    "nhD56A9FDUfuI0dVnPcrwNv0YJIo94LwxtbqBULNvXl6wJ7WAbODrCQy5ZgMVg+iH+gGpwiqsZqHt+KuoHWcN53MSPDfaF4/",
    "YMB99U3TziJMOOJask1TEEnakMPln11PczNDazT17DXIxYrbPfutPdh6sLs6AQOajdZijfEvepgnOe7cQ7aeatiOJFrjTApK",
    "PGxOVRzEMX4XS4xbyhH0QxQeB6l16l8C0uxIBAgMBAAECggEASaWfeVDA3cVzOPFSpvJm20OTE+R6uGOU+7vh36TX/POq92q",
    "Buwbd0h0oMD32FxsXywd2IxtBDUSiFM9699qufTVuM0Q3tZw6lHDTOVG08+tPdr8qSbMtw7PGFxN79fHLBxejjO4IrM9lapj",
    "WpxEF+11x7r+wM+0xRZQ8sNFYG46aPfIaty4BGbL0I2DQ2y8I57iBCAy69eht59NLMm27fRWGJIWCuBIjlpfzET1j2HLXUIh",
    "5bTBNzqaN039WH49HczGE3mQKVEJZc/efk3HaVd0a1Sjzyn0QY+N1jtZN3jTRbuDWA1AknkX1LX/0tUhuS3/7C3ejHxjw4Dk",
    "1ZLo5/QKBgQDIWvqFn0+IKRSu6Ua2hDsufIHHUNLelbfLUMmFthxabcUn4zlvIscJO00Tq/ezopSRRvbGiqnxjv/mYxucvOU",
    "BeZtlus0Q9RTACBtw9TGoNTmQbEunJ2FOSlqbQxkBBAjgGEppRPt30iGj/VjAhCATq2MYOa/X4dVR51BqQAFIEwKBgQDBSIf",
    "TFKC/hDk6FKZlgwvupWYJyU9RkyfstPErZFmzoKhPkQ3YORo2oeAYmVUbS9I2iIYpYpYQJHX8jMuCbCz4ONxTCuSIXYQYUcU",
    "q4PglCKp31xBAE6TN8SvhfME9/MvuDssnQinAHuF0GDAhF646T3LLS1not6Vszv7brwSoGwKBgQC88v/8cGfi80ssQZeMnVv",
    "q1UTXIeQcQnoY5lGHJl3K8mbS3TnXE6c9j417Fdz+rj8KWzBzwWXQB5pSPflWcdZO886Xu/mVGmy9RWgLuVFhXwCwsVEPjNX",
    "5ramRb0/vY0yzenUCninBsIxFSbIfrPtLUYCc4hpxr+sr2Mg/y6jpvQKBgBezMRRs3xkcuXepuI2R+BCXL1/b02IJTUf1F+1",
    "eLLGd7YV0H+J3fgNc7gGWK51hOrF9JBZHBGeOUPlaukmPwiPdtQZpu4QNE3l37VlIpKTF30E6mb+BqR+nht3rUjarnMXgAoE",
    "Z18y6/KIjpSMpqC92Nnk/EBM9EYe6Cf4eA9ApAoGAeqEUg46UTlJySkBKURGpIs3v1kkf5I0X8DnOhwb+HPxNaiEdmO7ckm8",
    "+tPVgppLcG0+tMdLjigFQiDUQk2y3WjyxP5ZvXu7U96jaJRI8PFMoE06WeVYcdIzrID2HvqH+w0UQJFrLJ/0Mn4stFAEzXKZ",
    "BokBGnjFnTnKcs7nv/O8=");

    const PUBLIC_KEY_B64: &str = concat!(
    "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAl0Vawl/toXzkEvB82FEtqHP",
    "4xlU2ab/v0crqIfXfIoWF/XXdHGIdrZeilnRXPPJT1B9dTsasttEZNnua/0Rek/cjNDHtzT52irfoZYS7X6HNIfOi54Q+egP",
    "RQ1H7iNHVZz3K8Db9GCSKPeC8MbW6gVCzb15esCe1gGzg6wkMuWYDFYPoh/oBqcIqrGah7firqB1nDedzEjw32heP2DAffVN",
    "084iTDjiWrJNUxBJ2pDD5Z9dT3MzQ2s09ew1yMWK2z37rT3YerC7OgEDmo3WYo3xL3qYJznu3EO2nmrYjiRa40wKSjxsTlUc",
    "xDF+F0uMW8oR9EMUHgepdepfAtLsSAQIDAQAB");

    const DATA_B64: &str = concat!(
    "A1/p8BQzN9UrbdYxUY2Va5+kPLyfZXF9JsZrjeEXcaclsnHurdxVAJcnbEqYMP3UXV",
    "4YAS/mpf+Rxe6/X0WS1boQdA0MAHSgx95hIlAraZYpiMLLiJRKeo2u8YivCdTM9V5vuAEJwf9Tof/qFsFci3sApdbATkorCT",
    "zFOIEPF2S1zgperEP23M01mr4dWVdYN18B32YF67xdJHMbFhp5dkQwv9CmscoWq7OE5HIfOb+JAh7BEZb+CmKhM3yWJvoR/D",
    "/5jcercUtK2o+XrzNrL4UQ7yLZcFz6Bfwb/j6ICYvqd/YJwXNE6dwlL57OfwJyCdw2rRYf0/qI00t9u8Iitw==");

    #[test]
    fn test_decrypt_rsa() {
        let private_key_bytes = BASE64_ENGINE.decode(PRIVATE_KEY_B64).unwrap();
        let private_key = rsa::RsaPrivateKey::from_pkcs8_der(&private_key_bytes).unwrap();
        let data_bytes = BASE64_ENGINE.decode(DATA_B64).unwrap();

        let result = decrypt_rsa(data_bytes, &private_key).unwrap();
        let result_string = String::from_utf8(result).unwrap();

        assert_eq!(result_string, "EncryptMe!");
    }

    #[test]
    fn test_encrypt_rsa() {
        let public_key_bytes = BASE64_ENGINE.decode(PUBLIC_KEY_B64).unwrap();
        let info = SubjectPublicKeyInfo::from_der(&public_key_bytes).unwrap();
        let public_key = RsaPublicKey::try_from(info).unwrap();

        let private_key_bytes = BASE64_ENGINE.decode(PRIVATE_KEY_B64).unwrap();
        let private_key = rsa::RsaPrivateKey::from_pkcs8_der(&private_key_bytes).unwrap();

        let encrypted = encrypt_rsa("EncryptMe!".as_bytes().to_vec(), &public_key).unwrap();
        let decrypted = decrypt_rsa(encrypted, &private_key).unwrap();

        let result_string = String::from_utf8(decrypted).unwrap();

        assert_eq!(result_string, "EncryptMe!");
    }
}
