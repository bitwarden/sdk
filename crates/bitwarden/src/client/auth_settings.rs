use std::num::NonZeroU32;

use base64::Engine;
#[cfg(feature = "internal")]
use bitwarden_api_identity::models::{KdfType, PreloginResponseModel};
use rand::Rng;
use rsa::{
    pkcs8::{EncodePrivateKey, EncodePublicKey},
    RsaPrivateKey, RsaPublicKey,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    crypto::{
        encrypt_aes256, encrypt_aes256_no_mac, EncString, PbkdfSha256Hmac, SymmetricCryptoKey,
        PBKDF_SHA256_HMAC_OUT_SIZE,
    },
    error::{Error, Result},
    util::BASE64_ENGINE,
};

#[derive(Debug)]
pub(crate) struct AuthSettings {
    pub email: String,
    pub(crate) kdf: Kdf,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum Kdf {
    PBKDF2 {
        iterations: NonZeroU32,
    },
    Argon2id {
        iterations: NonZeroU32,
        memory: NonZeroU32,
        parallelism: NonZeroU32,
    },
}

impl AuthSettings {
    #[cfg(feature = "internal")]
    pub fn new(response: PreloginResponseModel, email: String) -> Self {
        use crate::util::{
            default_argon2_iterations, default_argon2_memory, default_argon2_parallelism,
            default_pbkdf2_iterations,
        };

        let kdf = match response.kdf.unwrap_or_default() {
            KdfType::Variant0 => Kdf::PBKDF2 {
                iterations: response
                    .kdf_iterations
                    .and_then(|e| NonZeroU32::new(e as u32))
                    .unwrap_or_else(default_pbkdf2_iterations),
            },
            KdfType::Variant1 => Kdf::Argon2id {
                iterations: response
                    .kdf_iterations
                    .and_then(|e| NonZeroU32::new(e as u32))
                    .unwrap_or_else(default_argon2_iterations),
                memory: response
                    .kdf_memory
                    .and_then(|e| NonZeroU32::new(e as u32))
                    .unwrap_or_else(default_argon2_memory),
                parallelism: response
                    .kdf_parallelism
                    .and_then(|e| NonZeroU32::new(e as u32))
                    .unwrap_or_else(default_argon2_parallelism),
            },
        };

        Self { email, kdf }
    }

    pub fn make_user_password_hash(&self, password: &str) -> Result<String> {
        let key = self.make_master_key(password, &self.email)?;
        self.make_password_hash(password, key)
    }

    pub fn make_master_key(&self, password: &str, email: &str) -> Result<[u8; 32]> {
        crate::crypto::hash_kdf(password.as_bytes(), email.as_bytes(), &self.kdf)
    }

    pub fn make_password_hash(&self, password: &str, key: [u8; 32]) -> Result<String> {
        // Server expects hash + 1 iteration
        let login_hash = pbkdf2::pbkdf2_array::<PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE>(
            &key,
            password.as_bytes(),
            1,
        )
        .expect("hash is a valid fixed size");

        Ok(BASE64_ENGINE.encode(login_hash))
    }

    pub(crate) fn make_user_key(&self, key: [u8; 32]) -> Result<(SymmetricCryptoKey, EncString)> {
        let mut user_key = [0u8; 64];
        rand::thread_rng().fill(&mut user_key);

        let protected = encrypt_aes256_no_mac(&user_key, key.into())?;

        let u: &[u8] = &user_key;
        Ok((SymmetricCryptoKey::try_from(u)?, protected))
    }

    pub(crate) fn make_key_pair(
        &self,
        user_key: SymmetricCryptoKey,
    ) -> Result<(String, EncString)> {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        let spki = pub_key
            .to_public_key_der()
            .map_err(|e| Error::Internal("hi"))?;

        let b64 = BASE64_ENGINE.encode(spki.as_bytes());
        let pkcs = priv_key.to_pkcs8_der().map_err(|e| Error::Internal("hi"))?;

        let protected = encrypt_aes256(pkcs.as_bytes(), user_key.mac_key, user_key.key)?;

        Ok((b64, protected))
    }
}

#[cfg(test)]
mod tests {
    use bitwarden_api_identity::models::{KdfType, PreloginResponseModel};

    use super::AuthSettings;

    #[test]
    fn test_password_hash_pbkdf2() {
        let res = PreloginResponseModel {
            kdf: Some(KdfType::Variant0),
            kdf_iterations: Some(100_000),
            kdf_memory: None,
            kdf_parallelism: None,
        };
        let settings = AuthSettings::new(res, "test@bitwarden.com".into());

        let key = settings.make_master_key("asdfasdf", "test_salt").unwrap();

        assert_eq!(
            settings.make_password_hash("asdfasdf", key).unwrap(),
            "ZF6HjxUTSyBHsC+HXSOhZoXN+UuMnygV5YkWXCY4VmM="
        );
        assert_eq!(
            settings.make_user_password_hash("asdfasdf").unwrap(),
            "wmyadRMyBZOH7P/a/ucTCbSghKgdzDpPqUnu/DAVtSw="
        );
    }

    #[test]
    fn test_password_hash_argon2id() {
        let res = PreloginResponseModel {
            kdf: Some(KdfType::Variant1),
            kdf_iterations: Some(4),
            kdf_memory: Some(32),
            kdf_parallelism: Some(2),
        };
        let settings = AuthSettings::new(res, "test@bitwarden.com".into());

        let key = settings.make_master_key("asdfasdf", "test_salt").unwrap();

        assert_eq!(
            settings.make_password_hash("asdfasdf", key).unwrap(),
            "PR6UjYmjmppTYcdyTiNbAhPJuQQOmynKbdEl1oyi/iQ="
        );
        assert_eq!(
            settings.make_user_password_hash("asdfasdf").unwrap(),
            "ImYMPyd/X7FPrWzbt+wRfmlICWTA25yZrOob4TBMEZw="
        );
    }
}
