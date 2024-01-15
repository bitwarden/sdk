use super::{
    AsymmEncString, AsymmetricCryptoKey, EncString, KeyDecryptable, KeyEncryptable,
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
pub struct CreateDeviceKey {
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
    pub fn trust_device(user_key: UserKey) -> Result<CreateDeviceKey> {
        let device_key = DeviceKey(SymmetricCryptoKey::generate(rand::thread_rng()));

        let device_private_key = AsymmetricCryptoKey::generate();

        let protected_user_key =
            AsymmEncString::encrypt_rsa2048_oaep_sha1(&user_key.0.key, &device_private_key)?;

        let spki = device_private_key
            .to_public_der()
            .map_err(|_| "Invalid key")?;

        let protected_device_public_key = spki.encrypt_with_key(&user_key.0)?;

        let pkcs8 = device_private_key.to_der().map_err(|_| "Invalid key")?;

        let protected_device_private_key = pkcs8.encrypt_with_key(&device_key.0)?;

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
        let device_private_key = AsymmetricCryptoKey::from_der(device_private_key.as_slice())?;

        let dec: Vec<u8> = protected_user_key.decrypt_with_key(&device_private_key)?;
        let user_key: SymmetricCryptoKey = dec.as_slice().try_into()?;

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
