use base64::{engine::general_purpose::STANDARD, Engine};
use bitwarden_crypto::{
    fingerprint, AsymmetricCryptoKey, AsymmetricEncString, AsymmetricPublicCryptoKey,
};
#[cfg(feature = "mobile")]
use bitwarden_crypto::{KeyDecryptable, SymmetricCryptoKey};
use bitwarden_generators::{password, PasswordGeneratorRequest};

use crate::{error::Error, Client};

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct AuthRequestResponse {
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

/// Initiate a new auth request.
///
/// Generates a private key and access code. The pulic key is uploaded to the server and transmitted
/// to another device. Where the user confirms the validity by confirming the fingerprint. The user
/// key is then encrypted using the public key and returned to the initiating device.
pub(crate) fn new_auth_request(email: &str) -> Result<AuthRequestResponse, Error> {
    let mut rng = rand::thread_rng();

    let key = AsymmetricCryptoKey::generate(&mut rng);

    let spki = key.to_public_der()?;

    let fingerprint = fingerprint(email, &spki)?;
    let b64 = STANDARD.encode(&spki);

    Ok(AuthRequestResponse {
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

/// Decrypt the user key using the private key generated previously.
#[cfg(feature = "mobile")]
pub(crate) fn auth_request_decrypt_user_key(
    private_key: String,
    user_key: AsymmetricEncString,
) -> Result<SymmetricCryptoKey, Error> {
    let key = AsymmetricCryptoKey::from_der(&STANDARD.decode(private_key)?)?;
    let key: String = user_key.decrypt_with_key(&key)?;

    Ok(key.parse()?)
}

/// Approve a passwordless login request.
///
/// Encrypts the user key with a public key.
pub(crate) fn approve_auth_request(
    client: &mut Client,
    public_key: String,
) -> Result<AsymmetricEncString, Error> {
    let public_key = AsymmetricPublicCryptoKey::from_der(&STANDARD.decode(public_key)?)?;

    let enc = client.get_encryption_settings()?;
    let key = enc.get_key(&None).ok_or(Error::VaultLocked)?;

    Ok(AsymmetricEncString::encrypt_rsa2048_oaep_sha1(
        &key.to_vec(),
        &public_key,
    )?)
}

#[test]
fn test_auth_request() {
    let request = new_auth_request("test@bitwarden.com").unwrap();

    let secret =
        "w2LO+nwV4oxwswVYCxlOfRUseXfvU03VzvKQHrqeklPgiMZrspUe6sOBToCnDn9Ay0tuCBn8ykVVRb7PWhub2Q==";

    let private_key =
        AsymmetricCryptoKey::from_der(&STANDARD.decode(&request.private_key).unwrap()).unwrap();

    let encrypted =
        AsymmetricEncString::encrypt_rsa2048_oaep_sha1(secret.as_bytes(), &private_key).unwrap();

    let decrypted = auth_request_decrypt_user_key(request.private_key, encrypted).unwrap();

    assert_eq!(decrypted.to_base64(), secret);
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use bitwarden_crypto::Kdf;

    use super::*;
    use crate::client::{LoginMethod, UserLoginMethod};

    #[test]
    fn test_approve() {
        let mut client = Client::new(None);
        client.set_login_method(LoginMethod::User(UserLoginMethod::Username {
            client_id: "123".to_owned(),
            email: "test@bitwarden.com".to_owned(),
            kdf: Kdf::PBKDF2 {
                iterations: NonZeroU32::new(600_000).unwrap(),
            },
        }));

        let user_key = "2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=".parse().unwrap();
        let private_key ="2.yN7l00BOlUE0Sb0M//Q53w==|EwKG/BduQRQ33Izqc/ogoBROIoI5dmgrxSo82sgzgAMIBt3A2FZ9vPRMY+GWT85JiqytDitGR3TqwnFUBhKUpRRAq4x7rA6A1arHrFp5Tp1p21O3SfjtvB3quiOKbqWk6ZaU1Np9HwqwAecddFcB0YyBEiRX3VwF2pgpAdiPbSMuvo2qIgyob0CUoC/h4Bz1be7Qa7B0Xw9/fMKkB1LpOm925lzqosyMQM62YpMGkjMsbZz0uPopu32fxzDWSPr+kekNNyLt9InGhTpxLmq1go/pXR2uw5dfpXc5yuta7DB0EGBwnQ8Vl5HPdDooqOTD9I1jE0mRyuBpWTTI3FRnu3JUh3rIyGBJhUmHqGZvw2CKdqHCIrQeQkkEYqOeJRJVdBjhv5KGJifqT3BFRwX/YFJIChAQpebNQKXe/0kPivWokHWwXlDB7S7mBZzhaAPidZvnuIhalE2qmTypDwHy22FyqV58T8MGGMchcASDi/QXI6kcdpJzPXSeU9o+NC68QDlOIrMVxKFeE7w7PvVmAaxEo0YwmuAzzKy9QpdlK0aab/xEi8V4iXj4hGepqAvHkXIQd+r3FNeiLfllkb61p6WTjr5urcmDQMR94/wYoilpG5OlybHdbhsYHvIzYoLrC7fzl630gcO6t4nM24vdB6Ymg9BVpEgKRAxSbE62Tqacxqnz9AcmgItb48NiR/He3n3ydGjPYuKk/ihZMgEwAEZvSlNxYONSbYrIGDtOY+8Nbt6KiH3l06wjZW8tcmFeVlWv+tWotnTY9IqlAfvNVTjtsobqtQnvsiDjdEVtNy/s2ci5TH+NdZluca2OVEr91Wayxh70kpM6ib4UGbfdmGgCo74gtKvKSJU0rTHakQ5L9JlaSDD5FamBRyI0qfL43Ad9qOUZ8DaffDCyuaVyuqk7cz9HwmEmvWU3VQ+5t06n/5kRDXttcw8w+3qClEEdGo1KeENcnXCB32dQe3tDTFpuAIMLqwXs6FhpawfZ5kPYvLPczGWaqftIs/RXJ/EltGc0ugw2dmTLpoQhCqrcKEBDoYVk0LDZKsnzitOGdi9mOWse7Se8798ib1UsHFUjGzISEt6upestxOeupSTOh0v4+AjXbDzRUyogHww3V+Bqg71bkcMxtB+WM+pn1XNbVTyl9NR040nhP7KEf6e9ruXAtmrBC2ah5cFEpLIot77VFZ9ilLuitSz+7T8n1yAh1IEG6xxXxninAZIzi2qGbH69O5RSpOJuJTv17zTLJQIIc781JwQ2TTwTGnx5wZLbffhCasowJKd2EVcyMJyhz6ru0PvXWJ4hUdkARJs3Xu8dus9a86N8Xk6aAPzBDqzYb1vyFIfBxP0oO8xFHgd30Cgmz8UrSE3qeWRrF8ftrI6xQnFjHBGWD/JWSvd6YMcQED0aVuQkuNW9ST/DzQThPzRfPUoiL10yAmV7Ytu4fR3x2sF0Yfi87YhHFuCMpV/DsqxmUizyiJuD938eRcH8hzR/VO53Qo3UIsqOLcyXtTv6THjSlTopQ+JOLOnHm1w8dzYbLN44OG44rRsbihMUQp+wUZ6bsI8rrOnm9WErzkbQFbrfAINdoCiNa6cimYIjvvnMTaFWNymqY1vZxGztQiMiHiHYwTfwHTXrb9j0uPM=|09J28iXv9oWzYtzK2LBT6Yht4IT4MijEkk0fwFdrVQ4=".parse().unwrap();
        client
            .initialize_user_crypto("asdfasdfasdf", user_key, private_key)
            .unwrap();

        let public_key = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAnRtpYLp9QLaEUkdPkWZX6TrMUKFoSaFamBKDL0NlS6xwtETTqYIxRVsvnHii3Dhz+fh3aHQVyBa1rBXogeH3MLERzNADwZhpWtBT9wKCXY5o0fIWYdZV/Nf0Y+0ZoKdImrGPLPmyHGfCqrvrK7g09q8+3kXUlkdAImlQqc5TiYwiHBfUQVTBq/Ae7a0FEpajx1NUM4h3edpCYxbvnpSTuzMgbmbUUS4gdCaheA2ibYxy/zkLzsaLygoibMyGNl9Y8J5n7dDrVXpUKZTihVfXwHfEZwtKNunWsmmt8rEJWVpguUDEDVSUogoxQcNaCi7KHn9ioSip76hg1jLpypO3WwIDAQAB";

        // Verify fingerprint
        let pbkey = STANDARD.decode(public_key).unwrap();
        let fingerprint = fingerprint("test@bitwarden.com", &pbkey).unwrap();
        assert_eq!(fingerprint, "spill-applaud-sweep-habitable-shrunk");

        approve_auth_request(&mut client, public_key.to_owned()).unwrap();
    }
}
