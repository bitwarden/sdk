use std::num::NonZeroU32;

use base64::Engine;
#[cfg(feature = "internal")]
use bitwarden_api_identity::models::{KdfType, PreloginResponseModel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    crypto::{PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE},
    error::Result,
    util::BASE64_ENGINE,
};

#[derive(Debug)]
pub(crate) struct AuthSettings {
    pub email: String,
    pub(crate) kdf: Kdf,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
        self.make_password_hash(password, &self.email)
    }

    pub fn make_password_hash(&self, password: &str, salt: &str) -> Result<String> {
        let hash: [u8; 32] =
            crate::crypto::hash_kdf(password.as_bytes(), salt.as_bytes(), &self.kdf)?;

        // Server expects hash + 1 iteration
        let login_hash = pbkdf2::pbkdf2_array::<PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE>(
            &hash,
            password.as_bytes(),
            1,
        )
        .expect("hash is a valid fixed size");

        Ok(BASE64_ENGINE.encode(login_hash))
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

        assert_eq!(
            settings
                .make_password_hash("asdfasdf", "test_salt")
                .unwrap(),
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

        assert_eq!(
            settings
                .make_password_hash("asdfasdf", "test_salt")
                .unwrap(),
            "PR6UjYmjmppTYcdyTiNbAhPJuQQOmynKbdEl1oyi/iQ="
        );
        assert_eq!(
            settings.make_user_password_hash("asdfasdf").unwrap(),
            "ImYMPyd/X7FPrWzbt+wRfmlICWTA25yZrOob4TBMEZw="
        );
    }
}
