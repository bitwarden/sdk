use crate::{
    error::Result, AsymmEncString, AsymmetricCryptoKey, EncString, KeyDecryptable, KeyEncryptable,
    SymmetricCryptoKey, UserKey,
};

/// Device Key
///
/// Encrypts the DevicePrivateKey
/// Allows the device to decrypt the UserKey, via the DevicePrivateKey.
#[derive(Debug)]
pub struct DeviceKey(SymmetricCryptoKey);

#[derive(Debug)]
pub struct CreateDeviceKey {
    pub device_key: DeviceKey,
    /// UserKey encrypted with DevicePublicKey
    pub protected_user_key: AsymmEncString,
    /// DevicePrivateKey encrypted with [DeviceKey]
    pub protected_device_private_key: EncString,
    /// DevicePublicKey encrypted with [UserKey](super::UserKey)
    pub protected_device_public_key: EncString,
}

// We need to support the following scenarios:
//
// - Creating device key and encrypting users key
// - Decrypting users key using device key

impl DeviceKey {
    /// Generate a new device key
    ///
    /// Note: Input has to be a SymmetricCryptoKey instead of UserKey because that's what we get from EncSettings.
    pub fn trust_device(user_key: &SymmetricCryptoKey) -> Result<CreateDeviceKey> {
        let device_key = DeviceKey(SymmetricCryptoKey::generate(rand::thread_rng()));

        let device_private_key = AsymmetricCryptoKey::generate();

        // Encrypt both the key and mac_key of the user key
        let data = [user_key.key, user_key.mac_key.unwrap_or_default()].concat();

        let protected_user_key =
            AsymmEncString::encrypt_rsa2048_oaep_sha1(&data, &device_private_key)?;

        let protected_device_public_key = device_private_key
            .to_public_der()?
            .encrypt_with_key(user_key)?;

        let protected_device_private_key = device_private_key
            .to_der()?
            .encrypt_with_key(&device_key.0)?;

        Ok(CreateDeviceKey {
            device_key,
            protected_user_key,
            protected_device_private_key,
            protected_device_public_key,
        })
    }

    /// Decrypt the user key using the device key
    pub fn decrypt_user_key(
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
    use crate::derive_symmetric_key;

    use super::*;

    #[test]
    fn test_trust_device() {
        let key = derive_symmetric_key("test");

        // Call trust_device function
        let result = DeviceKey::trust_device(&key).unwrap();

        let decrypted = result
            .device_key
            .decrypt_user_key(
                result.protected_device_private_key,
                result.protected_user_key,
            )
            .unwrap();

        assert_eq!(key.key, decrypted.0.key);
        assert_eq!(key.mac_key, decrypted.0.mac_key);
    }
}
