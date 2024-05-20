/// Integration test for registering a new user and unlocking the vault
#[cfg(feature = "mobile")]
#[tokio::test]
async fn test_register_initialize_crypto() {
    use std::num::NonZeroU32;

    use bitwarden::{
        mobile::crypto::{InitUserCryptoMethod, InitUserCryptoRequest},
        Client,
    };
    use bitwarden_crypto::Kdf;

    let mut client = Client::new(None);

    let email = "test@bitwarden.com";
    let password = "test123";
    let kdf = Kdf::PBKDF2 {
        iterations: NonZeroU32::new(600_000).unwrap(),
    };

    let register_response = client
        .auth()
        .make_register_keys(email.to_owned(), password.to_owned(), kdf.clone())
        .unwrap();

    // Ensure we can initialize the crypto with the new keys
    client
        .crypto()
        .initialize_user_crypto(InitUserCryptoRequest {
            kdf_params: kdf,
            email: email.to_owned(),
            private_key: register_response.keys.private.to_string(),

            method: InitUserCryptoMethod::Password {
                password: password.to_owned(),
                user_key: register_response.encrypted_user_key,
            },
        })
        .await
        .unwrap();
}
