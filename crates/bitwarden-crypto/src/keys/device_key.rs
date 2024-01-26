use crate::{
    error::Result, AsymmetricCryptoKey, AsymmetricEncString, EncString, KeyDecryptable,
    KeyEncryptable, SymmetricCryptoKey, UserKey,
};

/// Device Key
///
/// Encrypts the DevicePrivateKey
/// Allows the device to decrypt the UserKey, via the DevicePrivateKey.
#[derive(Debug)]
pub struct DeviceKey(SymmetricCryptoKey);

#[derive(Debug)]
pub struct TrustDeviceResponse {
    pub device_key: DeviceKey,
    /// UserKey encrypted with DevicePublicKey
    pub protected_user_key: AsymmetricEncString,
    /// DevicePrivateKey encrypted with [DeviceKey]
    pub protected_device_private_key: EncString,
    /// DevicePublicKey encrypted with [UserKey](super::UserKey)
    pub protected_device_public_key: EncString,
}

impl DeviceKey {
    /// Generate a new device key
    ///
    /// Note: Input has to be a SymmetricCryptoKey instead of UserKey because that's what we get
    /// from EncSettings.
    pub fn trust_device(user_key: &SymmetricCryptoKey) -> Result<TrustDeviceResponse> {
        let mut rng = rand::thread_rng();
        let device_key = DeviceKey(SymmetricCryptoKey::generate(&mut rng));

        let device_private_key = AsymmetricCryptoKey::generate(&mut rng);

        // Encrypt both the key and mac_key of the user key
        let data = user_key.to_vec();

        let protected_user_key =
            AsymmetricEncString::encrypt_rsa2048_oaep_sha1(&data, &device_private_key)?;

        let protected_device_public_key = device_private_key
            .to_public_der()?
            .encrypt_with_key(user_key)?;

        let protected_device_private_key = device_private_key
            .to_der()?
            .encrypt_with_key(&device_key.0)?;

        Ok(TrustDeviceResponse {
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
        protected_user_key: AsymmetricEncString,
    ) -> Result<UserKey> {
        let device_private_key: Vec<u8> = protected_device_private_key.decrypt_with_key(&self.0)?;
        let device_private_key = AsymmetricCryptoKey::from_der(device_private_key.as_slice())?;

        let mut dec: Vec<u8> = protected_user_key.decrypt_with_key(&device_private_key)?;
        let user_key: SymmetricCryptoKey = dec.as_mut_slice().try_into()?;

        Ok(UserKey(user_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derive_symmetric_key;

    #[test]
    fn test_trust_device() {
        let key = derive_symmetric_key("test");

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

    #[test]
    fn test_decrypt_user_key() {
        // Example keys from desktop app
        let user_key: &mut [u8] = &mut [
            109, 128, 172, 147, 206, 123, 134, 95, 16, 36, 155, 113, 201, 18, 186, 230, 216, 212,
            173, 188, 74, 11, 134, 131, 137, 242, 105, 178, 105, 126, 52, 139, 248, 91, 215, 21,
            128, 91, 226, 222, 165, 67, 251, 34, 83, 81, 77, 147, 225, 76, 13, 41, 102, 45, 183,
            218, 106, 89, 254, 208, 251, 101, 130, 10,
        ];
        let user_key = SymmetricCryptoKey::try_from(user_key).unwrap();

        let key_data: &mut [u8] = &mut [
            114, 235, 60, 115, 172, 156, 203, 145, 195, 130, 215, 250, 88, 146, 215, 230, 12, 109,
            245, 222, 54, 217, 255, 211, 221, 105, 230, 236, 65, 52, 209, 133, 76, 208, 113, 254,
            194, 216, 156, 19, 230, 62, 32, 93, 87, 7, 144, 156, 117, 142, 250, 32, 182, 118, 187,
            8, 247, 7, 203, 201, 65, 147, 206, 247,
        ];
        let device_key = DeviceKey(key_data.try_into().unwrap());

        let protected_user_key: AsymmetricEncString = "4.f+VbbacRhO2q4MOUSdt1AIjQ2FuLAvg4aDxJMXAh3VxvbmUADj8Ct/R7XEpPUqApmbRS566jS0eRVy8Sk08ogoCdj1IFN9VsIky2i2X1WHK1fUnr3UBmXE3tl2NPBbx56U+h73S2jNTSyet2W18Jg2q7/w8KIhR3J41QrG9aGoOTN93to3hb5W4z6rdrSI0e7GkizbwcIA0NH7Z1JyAhrjPm9+tjRjg060YbEbGaWTAOkZWfgbLjr8bY455DteO2xxG139cOx7EBo66N+YhjsLi0ozkeUyPQkoWBdKMcQllS7jCfB4fDyJA05ALTbk74syKkvqFxqwmQbg+aVn+dcw==".parse().unwrap();

        let protected_device_private_key: EncString = "2.GyQfUYWW6Byy4UV5icFLxg==|EMiU7OTF79N6tfv3+YUs5zJhBAgqv6sa5YCoPl6yAETh7Tfk+JmbeizxXFPj5Q1X/tcVpDZl/3fGcxtnIxg1YtvDFn7j8uPnoApOWhCKmwcvJSIkt+qvX3lELNBwZXozSiy7PbQ0JbCMe2d4MkimR5k8+lE9FB3208yYK7nOJhlrsUCnOekCYEU9/4NCMA8tz8SpITx/MN4JJ1TQ/KjPJYLt+3JNUxK47QlgREWQvyVzCRt7ZGtcgIJ/U1qycAWMpEg9NkuV8j5QRA1S7VBsA6qliJwys5+dmTuIOmOMwdKFZDc4ZvWoRkPp2TSJBu7L8sSAgU6mmDWac8iQ+9Ka/drdfwYLrH8GAZvURk79tSpRrT7+PAFe2QdUtliUIyiqkh8iJVjZube4hRnEsRuX9V9b+UdtAr6zAj7mugO/VAu5T9J38V79V2ohG3NtXysDeKLXpAlkhjllWXeq/wret2fD4WiwqEDj0G2A/PY3F3OziIgp0UKc00AfqrPq8OVK3A+aowwVqdYadgxyoVCKWJ8unJeAXG7MrMQ9tHpzF6COoaEy7Wwoc17qko33zazwLZbfAjB4oc8Ea26jRKnJZP56sVZAjOSQQMziAsA08MRaa/DQhgRea1+Ygba0gMft8Dww8anN2gQBveTZRBWyqXYgN3U0Ity5gNauT8RnFk9faqVFt2Qxnp0JgJ+PsqEt5Hn4avBRZQQ7o8VvPnxYLDKFe3I2m6HFYFWRhOGeDYxexIuaiF2iIAYFVUmnDuWpgnUiL4XJ3KHDsjkPzcV3z4D2Knr/El2VVXve8jhDjETfovmmN28+i2e29PXvKIymTskMFpFCQPc7wBY/Id7pmgb3SujKYNpkAS2sByDoRir0my49DDGfta0dENssJhFd3x+87fZbEj3cMiikg2pBwpTLgmfIUa5cVZU2s8JZ9wu7gaioYzvX+elHa3EHLcnEUoJTtSf9kjb+Nbq4ktMgYAO2wIC96t1LvmqK4Qn2cOdw5QNlRqALhqe5V31kyIcwRMK0AyIoOPhnSqtpYdFiR3LDTvZA8dU0vSsuchCwHNMeRUtKvdzN/tk+oeznyY/mpakUESN501lEKd/QFLtJZsDZTtNlcA8fU3kDtws4ZIMR0O5+PFmgQFSU8OMobf9ClUzy/wHTvYGyDuSwbOoPeS955QKkUKXCNMj33yrPr+ioHQ1BNwLX3VmMF4bNRBY/vr+CG0/EZi0Gwl0kyHGl0yWEtpQuu+/PaROJeOraWy5D1UoZZhY4n0zJZBt1eg3FZ2rhKv4gdUc50nZpeNWE8pIqZ6RQ7qPJuqfF1Z+G73iOSnLYCHDiiFmhD5ivf9IGkTAcWcBsQ/2wcSj9bFJr4DrKfsbQ4CkSWICWVn/W+InKkO6BTsBbYmvte5SvbaN+UOtiUSkHLBCCr8273VNgcB/hgtbUires3noxYZJxoczr+i7vdlEgQnWEKrpo0CifsFxGwYS3Yy2K79iwvDMaLPDf73zLSbuoUl6602F2Mzcjnals67f+gSpaDvWt7Kg9c/ZfGjq8oNxVaXJnX3gSDsO+fhwVAtnDApL+tL8cFfxGerW4KGi9/74woH+C3MMIViBtNnrpEuvxUW97Dg5nd40oGDeyi/q+8HdcxkneyFY=|JYdol19Yi+n1r7M+06EwK5JCi2s/CWqKui2Cy6hEb3k=".parse().unwrap();

        let decrypted = device_key
            .decrypt_user_key(protected_device_private_key, protected_user_key)
            .unwrap();

        assert_eq!(decrypted.0.key, user_key.key);
        assert_eq!(decrypted.0.mac_key, user_key.mac_key);
    }
}
