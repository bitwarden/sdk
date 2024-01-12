use rsa::{
    pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey},
    Oaep, RsaPrivateKey,
};

use sha1::Sha1;

use super::{
    rsa::generate_rsa, AsymmEncString, EncString, KeyDecryptable, KeyEncryptable,
    SymmetricCryptoKey, UserKey,
};
use crate::error::Result;

/// Device Key
///
/// Encrypts the DevicePrivateKey
/// Allows the device to decrypt the UserKey, via the DevicePrivateKey.
#[derive(Debug)]
pub(crate) struct DeviceKey(SymmetricCryptoKey);

#[derive(Debug)]
struct CreateDeviceKey {
    device_key: DeviceKey,
    /// UserKey encrypted with DevicePublicKey
    protected_user_key: AsymmEncString,
    /// DevicePrivateKey encrypted with [DeviceKey]
    protected_device_private_key: EncString,
    /// DevicePublicKey encrypted with [UserKey](super::UserKey)
    protected_device_public_key: EncString,
}

// We need to support the following scenarios:
//
// - Creating device key and encrypting users key
// - Decrypting users key using device key

impl DeviceKey {
    /// Generate a new device key
    fn trust_device(user_key: UserKey) -> Result<CreateDeviceKey> {
        let device_key = DeviceKey(SymmetricCryptoKey::generate(rand::thread_rng()));

        let device_private_key = generate_rsa();

        let mut rng = rand::thread_rng();
        let padding = Oaep::new::<Sha1>();

        let protected_user_key = AsymmEncString::Rsa2048_OaepSha1_B64 {
            data: device_private_key
                .to_public_key()
                .encrypt(&mut rng, padding, &user_key.0.key)
                .map_err(|e| e.to_string())?,
        };

        let spki = device_private_key
            .to_public_key()
            .to_public_key_der()
            .map_err(|_| "Invalid key")?;

        let protected_device_public_key = spki.as_bytes().encrypt_with_key(&user_key.0)?;

        let pkcs8 = device_private_key
            .to_pkcs8_der()
            .map_err(|_| "Invalid key")?;

        let protected_device_private_key = pkcs8.as_bytes().encrypt_with_key(&device_key.0)?;

        Ok(CreateDeviceKey {
            device_key,
            protected_user_key,
            protected_device_private_key,
            protected_device_public_key,
        })
    }

    /// Decrypt the user key using the device key
    async fn decrypt_user_key(
        &self,
        protected_device_private_key: EncString,
        protected_user_key: AsymmEncString,
    ) -> Result<UserKey> {
        let device_private_key: Vec<u8> = protected_device_private_key.decrypt_with_key(&self.0)?;
        let device_private_key = RsaPrivateKey::from_pkcs8_der(device_private_key.as_slice())
            .map_err(|e| e.to_string())?;

        let user_key: SymmetricCryptoKey = protected_user_key
            .decrypt(&device_private_key)?
            .as_slice()
            .try_into()?;

        Ok(UserKey(user_key))
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::symmetric_crypto_key::derive_symmetric_key;

    use super::*;

    #[test]
    fn test_trust_device() {
        let user_key = UserKey(derive_symmetric_key("test"));

        // Call trust_device function
        let result = DeviceKey::trust_device(user_key).unwrap();

        println!("result: {:?}", result);
    }
}
