use bitwarden_crypto::{CryptoError, MasterKey, RsaKeyPair};

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct KeyConnectorResponse {
    pub master_key: String,
    pub encrypted_user_key: String,
    pub keys: RsaKeyPair,
}

pub(super) fn make_key_connector_keys(
    mut rng: impl rand::RngCore,
) -> Result<KeyConnectorResponse, CryptoError> {
    let master_key = MasterKey::generate(&mut rng);
    let (user_key, encrypted_user_key) = master_key.make_user_key()?;
    let keys = user_key.make_key_pair()?;

    Ok(KeyConnectorResponse {
        master_key: master_key.to_base64(),
        encrypted_user_key: encrypted_user_key.to_string(),
        keys,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_make_key_connector_keys() {
        let mut rng = ChaCha8Rng::from_seed([0u8; 32]);

        let result = make_key_connector_keys(&mut rng).unwrap();

        assert_eq!(
            result.master_key,
            "PgDvL4lfQNZ/W7joHwmloSyEDsPOmn87GBvhiO9xGh4="
        );
    }
}
