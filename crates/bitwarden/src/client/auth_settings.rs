use std::num::NonZeroU32;

#[cfg(feature = "internal")]
use bitwarden_api_identity::models::{KdfType, PreloginResponseModel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "internal")]
use crate::{
    crypto::keys::HashPurpose,
    error::Result,
};

#[derive(Debug)]
pub(crate) struct AuthSettings {
    #[cfg(feature = "internal")]
    pub email: String,
    #[cfg(feature = "internal")]
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

    #[cfg(feature = "internal")]
    pub fn derive_user_password_hash(&self, password: &str) -> Result<String> {
        use crate::crypto::{SymmetricCryptoKey, keys::FromMasterPassword};

        let master_key = SymmetricCryptoKey::<FromMasterPassword>::derive(password.as_bytes(), self.email.as_bytes(), &self.kdf)?;
        master_key.derive_master_key_hash(password.as_bytes(), HashPurpose::ServerAuthorization)
    }
}
