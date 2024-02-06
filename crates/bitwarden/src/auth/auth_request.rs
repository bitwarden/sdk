use base64::{engine::general_purpose::STANDARD, Engine};
use bitwarden_crypto::{
    fingerprint, AsymmetricCryptoKey, AsymmetricEncString, AsymmetricPublicCryptoKey,
};
#[cfg(feature = "mobile")]
use bitwarden_crypto::{EncString, KeyDecryptable, SymmetricCryptoKey};
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
    let mut key: Vec<u8> = user_key.decrypt_with_key(&key)?;

    Ok(SymmetricCryptoKey::try_from(key.as_mut_slice())?)
}

/// Decrypt the user key using the private key generated previously.
#[cfg(feature = "mobile")]
pub(crate) fn auth_request_decrypt_master_key(
    private_key: String,
    master_key: AsymmetricEncString,
    user_key: EncString,
) -> Result<SymmetricCryptoKey, Error> {
    use bitwarden_crypto::MasterKey;

    let key = AsymmetricCryptoKey::from_der(&STANDARD.decode(private_key)?)?;
    let mut master_key: Vec<u8> = master_key.decrypt_with_key(&key)?;
    let master_key = MasterKey::new(SymmetricCryptoKey::try_from(master_key.as_mut_slice())?);

    Ok(master_key.decrypt_user_key(user_key)?)
}

/// Approve an auth request.
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
    use zeroize::Zeroizing;

    let request = new_auth_request("test@bitwarden.com").unwrap();

    let secret: &[u8] = &[
        111, 32, 97, 169, 4, 241, 174, 74, 239, 206, 113, 86, 174, 68, 216, 238, 52, 85, 156, 27,
        134, 149, 54, 55, 91, 147, 45, 130, 131, 237, 51, 31, 191, 106, 155, 14, 160, 82, 47, 40,
        96, 31, 114, 127, 212, 187, 167, 110, 205, 116, 198, 243, 218, 72, 137, 53, 248, 43, 255,
        67, 35, 61, 245, 93,
    ];

    let private_key =
        AsymmetricCryptoKey::from_der(&STANDARD.decode(&request.private_key).unwrap()).unwrap();

    let encrypted = AsymmetricEncString::encrypt_rsa2048_oaep_sha1(secret, &private_key).unwrap();

    let decrypted = auth_request_decrypt_user_key(request.private_key, encrypted).unwrap();

    assert_eq!(decrypted.to_vec(), Zeroizing::new(secret.to_owned()));
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use bitwarden_crypto::Kdf;

    use super::*;
    use crate::{
        client::{LoginMethod, UserLoginMethod},
        mobile::crypto::{AuthRequestMethod, InitUserCryptoMethod, InitUserCryptoRequest},
    };

    #[test]
    fn test_approve() {
        let mut client = Client::new(None);
        client.set_login_method(LoginMethod::User(UserLoginMethod::Username {
            client_id: "7b821276-e27c-400b-9853-606393c87f18".to_owned(),
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

        let public_key = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvyLRDUwXB4BfQ507D4meFPmwn5zwy3IqTPJO4plrrhnclWahXa240BzyFW9gHgYu+Jrgms5xBfRTBMcEsqqNm7+JpB6C1B6yvnik0DpJgWQw1rwvy4SUYidpR/AWbQi47n/hvnmzI/sQxGddVfvWu1iTKOlf5blbKYAXnUE5DZBGnrWfacNXwRRdtP06tFB0LwDgw+91CeLSJ9py6dm1qX5JIxoO8StJOQl65goLCdrTWlox+0Jh4xFUfCkb+s3px+OhSCzJbvG/hlrSRcUz5GnwlCEyF3v5lfUtV96MJD+78d8pmH6CfFAp2wxKRAbGdk+JccJYO6y6oIXd3Fm7twIDAQAB";

        // Verify fingerprint
        let pbkey = STANDARD.decode(public_key).unwrap();
        let fingerprint = fingerprint("test@bitwarden.com", &pbkey).unwrap();
        assert_eq!(fingerprint, "childless-unfair-prowler-dropbox-designate");

        approve_auth_request(&mut client, public_key.to_owned()).unwrap();
    }

    #[tokio::test]
    async fn test_decrypt_user_key() {
        let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCzLtEUdxfcLxDj84yaGFsVF5hZ8Hjlb08NMQDy1RnBma06I3ZESshLYzVz4r/gegMn9OOltfV/Yxlyvida8oW6qdlfJ7AVz6Oa8pV7BiL40C7b76+oqraQpyYw2HChANB1AhXL9SqWngKmLZwjA7qiCrmcc0kZHeOb4KnKtp9iVvPVs+8veFvKgYO4ba2AAOHKFdR0W55/agXfAy+fWUAkC8mc9ikyJdQWaPV6OZvC2XFkOseBQm9Rynudh3BQpoWiL6w620efe7t5k+02/EyOFJL9f/XEEjM/+Yo0t3LAfkuhHGeKiRST59Xc9hTEmyJTeVXROtz+0fjqOp3xkaObAgMBAAECggEACs4xhnO0HaZhh1/iH7zORMIRXKeyxP2LQiTR8xwN5JJ9wRWmGAR9VasS7EZFTDidIGVME2u/h4s5EqXnhxfO+0gGksVvgNXJ/qw87E8K2216g6ZNo6vSGA7H1GH2voWwejJ4/k/cJug6dz2S402rRAKh2Wong1arYHSkVlQp3diiMa5FHAOSE+Cy09O2ZsaF9IXQYUtlW6AVXFrBEPYH2kvkaPXchh8VETMijo6tbvoKLnUHe+wTaDMls7hy8exjtVyI59r3DNzjy1lNGaGb5QSnFMXR+eHhPZc844Wv02MxC15zKABADrl58gpJyjTl6XpDdHCYGsmGpVGH3X9TQQKBgQDz/9beFjzq59ve6rGwn+EtnQfSsyYT+jr7GN8lNEXb3YOFXBgPhfFIcHRh2R00Vm9w2ApfAx2cd8xm2I6HuvQ1Os7g26LWazvuWY0Qzb+KaCLQTEGH1RnTq6CCG+BTRq/a3J8M4t38GV5TWlzv8wr9U4dl6FR4efjb65HXs1GQ4QKBgQC7/uHfrOTEHrLeIeqEuSl0vWNqEotFKdKLV6xpOvNuxDGbgW4/r/zaxDqt0YBOXmRbQYSEhmO3oy9J6XfE1SUln0gbavZeW0HESCAmUIC88bDnspUwS9RxauqT5aF8ODKN/bNCWCnBM1xyonPOs1oT1nyparJVdQoG//Y7vkB3+wKBgBqLqPq8fKAp3XfhHLfUjREDVoiLyQa/YI9U42IOz9LdxKNLo6p8rgVthpvmnRDGnpUuS+KOWjhdqDVANjF6G3t3DG7WNl8Rh5Gk2H4NhFswfSkgQrjebFLlBy9gjQVCWXt8KSmjvPbiY6q52Aaa8IUjA0YJAregvXxfopxO+/7BAoGARicvEtDp7WWnSc1OPoj6N14VIxgYcI7SyrzE0d/1x3ffKzB5e7qomNpxKzvqrVP8DzG7ydh8jaKPmv1MfF8tpYRy3AhmN3/GYwCnPqT75YYrhcrWcVdax5gmQVqHkFtIQkRSCIftzPLlpMGKha/YBV8c1fvC4LD0NPh/Ynv0gtECgYEAyOZg95/kte0jpgUEgwuMrzkhY/AaUJULFuR5MkyvReEbtSBQwV5tx60+T95PHNiFooWWVXiLMsAgyI2IbkxVR1Pzdri3gWK5CTfqb7kLuaj/B7SGvBa2Sxo478KS5K8tBBBWkITqo+wLC0mn3uZi1dyMWO1zopTA+KtEGF2dtGQ=";

        let enc_user_key = "4.dxbd5OMwi/Avy7DQxvLV+Z7kDJgHBtg/jAbgYNO7QU0Zii4rLFNco2lS5aS9z42LTZHc2p5HYwn2ZwkZNfHsQ6//d5q40MDgGYJMKBXOZP62ZHhct1XsvYBmtcUtIOm5j2HSjt2pjEuGAc1LbyGIWRJJQ3Lp1ULbL2m71I+P23GF36JyOM8SUWvpvxE/3+qqVhRFPG2VqMCYa2kLLxwVfUmpV+KKjX1TXsrq6pfJIwHNwHw4h7MSfD8xTy2bx4MiBt638Z9Vt1pGsSQkh9RgPvCbnhuCpZQloUgJ8ByLVEcrlKx3yaaxiQXvte+ZhuOI7rGdjmoVoOzisooje4JgYw==".parse().unwrap();
        let dec = auth_request_decrypt_user_key(private_key.to_owned(), enc_user_key).unwrap();

        assert_eq!(
            dec.to_vec().as_ref(),
            vec![
                201, 37, 234, 213, 21, 75, 40, 70, 149, 213, 234, 16, 19, 251, 162, 245, 161, 74,
                34, 245, 211, 151, 211, 192, 95, 10, 117, 50, 88, 223, 23, 157
            ]
        );
    }

    #[tokio::test]
    async fn test_decrypt_master_key() {
        let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCzLtEUdxfcLxDj84yaGFsVF5hZ8Hjlb08NMQDy1RnBma06I3ZESshLYzVz4r/gegMn9OOltfV/Yxlyvida8oW6qdlfJ7AVz6Oa8pV7BiL40C7b76+oqraQpyYw2HChANB1AhXL9SqWngKmLZwjA7qiCrmcc0kZHeOb4KnKtp9iVvPVs+8veFvKgYO4ba2AAOHKFdR0W55/agXfAy+fWUAkC8mc9ikyJdQWaPV6OZvC2XFkOseBQm9Rynudh3BQpoWiL6w620efe7t5k+02/EyOFJL9f/XEEjM/+Yo0t3LAfkuhHGeKiRST59Xc9hTEmyJTeVXROtz+0fjqOp3xkaObAgMBAAECggEACs4xhnO0HaZhh1/iH7zORMIRXKeyxP2LQiTR8xwN5JJ9wRWmGAR9VasS7EZFTDidIGVME2u/h4s5EqXnhxfO+0gGksVvgNXJ/qw87E8K2216g6ZNo6vSGA7H1GH2voWwejJ4/k/cJug6dz2S402rRAKh2Wong1arYHSkVlQp3diiMa5FHAOSE+Cy09O2ZsaF9IXQYUtlW6AVXFrBEPYH2kvkaPXchh8VETMijo6tbvoKLnUHe+wTaDMls7hy8exjtVyI59r3DNzjy1lNGaGb5QSnFMXR+eHhPZc844Wv02MxC15zKABADrl58gpJyjTl6XpDdHCYGsmGpVGH3X9TQQKBgQDz/9beFjzq59ve6rGwn+EtnQfSsyYT+jr7GN8lNEXb3YOFXBgPhfFIcHRh2R00Vm9w2ApfAx2cd8xm2I6HuvQ1Os7g26LWazvuWY0Qzb+KaCLQTEGH1RnTq6CCG+BTRq/a3J8M4t38GV5TWlzv8wr9U4dl6FR4efjb65HXs1GQ4QKBgQC7/uHfrOTEHrLeIeqEuSl0vWNqEotFKdKLV6xpOvNuxDGbgW4/r/zaxDqt0YBOXmRbQYSEhmO3oy9J6XfE1SUln0gbavZeW0HESCAmUIC88bDnspUwS9RxauqT5aF8ODKN/bNCWCnBM1xyonPOs1oT1nyparJVdQoG//Y7vkB3+wKBgBqLqPq8fKAp3XfhHLfUjREDVoiLyQa/YI9U42IOz9LdxKNLo6p8rgVthpvmnRDGnpUuS+KOWjhdqDVANjF6G3t3DG7WNl8Rh5Gk2H4NhFswfSkgQrjebFLlBy9gjQVCWXt8KSmjvPbiY6q52Aaa8IUjA0YJAregvXxfopxO+/7BAoGARicvEtDp7WWnSc1OPoj6N14VIxgYcI7SyrzE0d/1x3ffKzB5e7qomNpxKzvqrVP8DzG7ydh8jaKPmv1MfF8tpYRy3AhmN3/GYwCnPqT75YYrhcrWcVdax5gmQVqHkFtIQkRSCIftzPLlpMGKha/YBV8c1fvC4LD0NPh/Ynv0gtECgYEAyOZg95/kte0jpgUEgwuMrzkhY/AaUJULFuR5MkyvReEbtSBQwV5tx60+T95PHNiFooWWVXiLMsAgyI2IbkxVR1Pzdri3gWK5CTfqb7kLuaj/B7SGvBa2Sxo478KS5K8tBBBWkITqo+wLC0mn3uZi1dyMWO1zopTA+KtEGF2dtGQ=";

        let enc_master_key = "4.dxbd5OMwi/Avy7DQxvLV+Z7kDJgHBtg/jAbgYNO7QU0Zii4rLFNco2lS5aS9z42LTZHc2p5HYwn2ZwkZNfHsQ6//d5q40MDgGYJMKBXOZP62ZHhct1XsvYBmtcUtIOm5j2HSjt2pjEuGAc1LbyGIWRJJQ3Lp1ULbL2m71I+P23GF36JyOM8SUWvpvxE/3+qqVhRFPG2VqMCYa2kLLxwVfUmpV+KKjX1TXsrq6pfJIwHNwHw4h7MSfD8xTy2bx4MiBt638Z9Vt1pGsSQkh9RgPvCbnhuCpZQloUgJ8ByLVEcrlKx3yaaxiQXvte+ZhuOI7rGdjmoVoOzisooje4JgYw==".parse().unwrap();
        let enc_user_key = "2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=".parse().unwrap();
        let dec =
            auth_request_decrypt_master_key(private_key.to_owned(), enc_master_key, enc_user_key)
                .unwrap();

        assert_eq!(
            dec.to_vec().as_ref(),
            vec![
                109, 128, 172, 147, 206, 123, 134, 95, 16, 36, 155, 113, 201, 18, 186, 230, 216,
                212, 173, 188, 74, 11, 134, 131, 137, 242, 105, 178, 105, 126, 52, 139, 248, 91,
                215, 21, 128, 91, 226, 222, 165, 67, 251, 34, 83, 81, 77, 147, 225, 76, 13, 41,
                102, 45, 183, 218, 106, 89, 254, 208, 251, 101, 130, 10,
            ]
        );
    }

    #[tokio::test]
    async fn test_device_login() {
        let kdf = Kdf::PBKDF2 {
            iterations: NonZeroU32::new(600_000).unwrap(),
        };
        let email = "test@bitwarden.com";

        let user_key = "2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=".parse().unwrap();
        let private_key = "2.yN7l00BOlUE0Sb0M//Q53w==|EwKG/BduQRQ33Izqc/ogoBROIoI5dmgrxSo82sgzgAMIBt3A2FZ9vPRMY+GWT85JiqytDitGR3TqwnFUBhKUpRRAq4x7rA6A1arHrFp5Tp1p21O3SfjtvB3quiOKbqWk6ZaU1Np9HwqwAecddFcB0YyBEiRX3VwF2pgpAdiPbSMuvo2qIgyob0CUoC/h4Bz1be7Qa7B0Xw9/fMKkB1LpOm925lzqosyMQM62YpMGkjMsbZz0uPopu32fxzDWSPr+kekNNyLt9InGhTpxLmq1go/pXR2uw5dfpXc5yuta7DB0EGBwnQ8Vl5HPdDooqOTD9I1jE0mRyuBpWTTI3FRnu3JUh3rIyGBJhUmHqGZvw2CKdqHCIrQeQkkEYqOeJRJVdBjhv5KGJifqT3BFRwX/YFJIChAQpebNQKXe/0kPivWokHWwXlDB7S7mBZzhaAPidZvnuIhalE2qmTypDwHy22FyqV58T8MGGMchcASDi/QXI6kcdpJzPXSeU9o+NC68QDlOIrMVxKFeE7w7PvVmAaxEo0YwmuAzzKy9QpdlK0aab/xEi8V4iXj4hGepqAvHkXIQd+r3FNeiLfllkb61p6WTjr5urcmDQMR94/wYoilpG5OlybHdbhsYHvIzYoLrC7fzl630gcO6t4nM24vdB6Ymg9BVpEgKRAxSbE62Tqacxqnz9AcmgItb48NiR/He3n3ydGjPYuKk/ihZMgEwAEZvSlNxYONSbYrIGDtOY+8Nbt6KiH3l06wjZW8tcmFeVlWv+tWotnTY9IqlAfvNVTjtsobqtQnvsiDjdEVtNy/s2ci5TH+NdZluca2OVEr91Wayxh70kpM6ib4UGbfdmGgCo74gtKvKSJU0rTHakQ5L9JlaSDD5FamBRyI0qfL43Ad9qOUZ8DaffDCyuaVyuqk7cz9HwmEmvWU3VQ+5t06n/5kRDXttcw8w+3qClEEdGo1KeENcnXCB32dQe3tDTFpuAIMLqwXs6FhpawfZ5kPYvLPczGWaqftIs/RXJ/EltGc0ugw2dmTLpoQhCqrcKEBDoYVk0LDZKsnzitOGdi9mOWse7Se8798ib1UsHFUjGzISEt6upestxOeupSTOh0v4+AjXbDzRUyogHww3V+Bqg71bkcMxtB+WM+pn1XNbVTyl9NR040nhP7KEf6e9ruXAtmrBC2ah5cFEpLIot77VFZ9ilLuitSz+7T8n1yAh1IEG6xxXxninAZIzi2qGbH69O5RSpOJuJTv17zTLJQIIc781JwQ2TTwTGnx5wZLbffhCasowJKd2EVcyMJyhz6ru0PvXWJ4hUdkARJs3Xu8dus9a86N8Xk6aAPzBDqzYb1vyFIfBxP0oO8xFHgd30Cgmz8UrSE3qeWRrF8ftrI6xQnFjHBGWD/JWSvd6YMcQED0aVuQkuNW9ST/DzQThPzRfPUoiL10yAmV7Ytu4fR3x2sF0Yfi87YhHFuCMpV/DsqxmUizyiJuD938eRcH8hzR/VO53Qo3UIsqOLcyXtTv6THjSlTopQ+JOLOnHm1w8dzYbLN44OG44rRsbihMUQp+wUZ6bsI8rrOnm9WErzkbQFbrfAINdoCiNa6cimYIjvvnMTaFWNymqY1vZxGztQiMiHiHYwTfwHTXrb9j0uPM=|09J28iXv9oWzYtzK2LBT6Yht4IT4MijEkk0fwFdrVQ4=";

        // Initialize an existing client which is unlocked
        let mut existing_device = Client::new(None);
        existing_device.set_login_method(LoginMethod::User(UserLoginMethod::Username {
            client_id: "123".to_owned(),
            email: email.to_owned(),
            kdf: kdf.clone(),
        }));

        existing_device
            .initialize_user_crypto("asdfasdfasdf", user_key, private_key.parse().unwrap())
            .unwrap();

        // Initialize a new device which will request to be logged in
        let mut new_device = Client::new(None);

        // Initialize an auth request, and approve it on the existing device
        let auth_req = new_auth_request(email).unwrap();
        let approved_req = approve_auth_request(&mut existing_device, auth_req.public_key).unwrap();

        // Unlock the vault using the approved request
        new_device
            .crypto()
            .initialize_user_crypto(InitUserCryptoRequest {
                kdf_params: kdf,
                email: email.to_owned(),
                private_key: private_key.to_owned(),
                method: InitUserCryptoMethod::AuthRequest {
                    request_private_key: auth_req.private_key,
                    method: AuthRequestMethod::UserKey {
                        protected_user_key: approved_req,
                    },
                },
            })
            .await
            .unwrap();

        // We can validate that the vault is unlocked correctly by confirming the user key is the
        // same
        assert_eq!(
            existing_device
                .get_encryption_settings()
                .unwrap()
                .get_key(&None)
                .unwrap()
                .to_base64(),
            new_device
                .get_encryption_settings()
                .unwrap()
                .get_key(&None)
                .unwrap()
                .to_base64()
        );
    }
}
