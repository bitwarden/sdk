use base64::{engine::general_purpose::STANDARD, Engine};
use bitwarden_crypto::{
    AsymmetricEncString, AsymmetricPublicCryptoKey, DeviceKey, EncString, Kdf, SensitiveVec,
    SymmetricCryptoKey, TrustDeviceResponse, UserKey,
};

use crate::{error::Result, Client};

/// This function generates a new user key and key pair, initializes the client's crypto with the
/// generated user key, and encrypts the user key with the organization public key for admin
/// password reset. If remember_device is true, it also generates a device key.
pub(super) fn make_register_tde_keys(
    client: &mut Client,
    email: String,
    org_public_key: String,
    remember_device: bool,
) -> Result<RegisterTdeKeyResponse> {
    let public_key = AsymmetricPublicCryptoKey::from_der(SensitiveVec::new(Box::new(
        STANDARD.decode(org_public_key)?,
    )))?;

    let mut rng = rand::thread_rng();

    let user_key = UserKey::new(SymmetricCryptoKey::generate(&mut rng));
    let key_pair = user_key.make_key_pair()?;

    let admin_reset =
        AsymmetricEncString::encrypt_rsa2048_oaep_sha1(user_key.0.to_vec().expose(), &public_key)?;

    let device_key = if remember_device {
        Some(DeviceKey::trust_device(&user_key.0)?)
    } else {
        None
    };

    client.set_login_method(crate::client::LoginMethod::User(
        crate::client::UserLoginMethod::Username {
            client_id: "".to_owned(),
            email,
            kdf: Kdf::default(),
        },
    ));
    client.initialize_user_crypto_decrypted_key(user_key.0, key_pair.private.clone())?;

    Ok(RegisterTdeKeyResponse {
        private_key: key_pair.private,
        public_key: key_pair.public,

        admin_reset,
        device_key,
    })
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct RegisterTdeKeyResponse {
    pub private_key: EncString,
    pub public_key: String,

    pub admin_reset: AsymmetricEncString,
    pub device_key: Option<TrustDeviceResponse>,
}
