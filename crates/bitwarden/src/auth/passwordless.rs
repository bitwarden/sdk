use base64::{engine::general_purpose::STANDARD, Engine};
use bitwarden_crypto::{fingerprint, AsymmetricCryptoKey};
#[cfg(feature = "mobile")]
use bitwarden_crypto::{AsymmetricEncString, KeyDecryptable, SymmetricCryptoKey};
use bitwarden_generators::{password, PasswordGeneratorRequest};

use crate::error::Error;

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PasswordlessLoginRequest {
    /// Base64 encoded private key
    /// This key is temporarily passed back and will most likely not be available in the future
    pub private_key: String,
    /// Base64 encoded public key
    pub public_key: String,
    /// Fingerprint of the public key
    pub fingerprint: String,
    /// Access code
    pub access_code: String,
}

pub(crate) fn new_passwordless_request(email: &str) -> Result<PasswordlessLoginRequest, Error> {
    let mut rng = rand::thread_rng();

    let key = AsymmetricCryptoKey::generate(&mut rng);

    let spki = key.to_public_der()?;

    let fingerprint = fingerprint(email, &spki)?;
    let b64 = STANDARD.encode(&spki);

    Ok(PasswordlessLoginRequest {
        private_key: STANDARD.encode(key.to_der()?),
        public_key: b64,
        fingerprint,
        access_code: password(PasswordGeneratorRequest {
            length: 25,
            lowercase: true,
            uppercase: true,
            numbers: true,
            special: false,
            ..Default::default()
        })?,
    })
}

#[cfg(feature = "mobile")]
pub(crate) fn passwordless_decrypt_user_key(
    private_key: String,
    user_key: AsymmetricEncString,
) -> Result<SymmetricCryptoKey, Error> {
    let key = AsymmetricCryptoKey::from_der(&STANDARD.decode(private_key)?)?;
    let key: String = user_key.decrypt_with_key(&key)?;

    Ok(key.parse()?)
}

#[test]
fn test_passwordless_login_request() {
    let request = new_passwordless_request("test@bitwarden.com").unwrap();

    let secret =
        "w2LO+nwV4oxwswVYCxlOfRUseXfvU03VzvKQHrqeklPgiMZrspUe6sOBToCnDn9Ay0tuCBn8ykVVRb7PWhub2Q==";

    let private_key =
        AsymmetricCryptoKey::from_der(&STANDARD.decode(&request.private_key).unwrap()).unwrap();

    let encrypted =
        AsymmetricEncString::encrypt_rsa2048_oaep_sha1(secret.as_bytes(), &private_key).unwrap();

    let decrypted = passwordless_decrypt_user_key(request.private_key, encrypted).unwrap();

    assert_eq!(decrypted.to_base64(), secret);
}
