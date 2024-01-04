use std::num::NonZeroU32;

#[cfg(feature = "internal")]
use bitwarden_api_identity::models::{KdfType, PreloginResponseModel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "internal")]
use crate::error::{Error, Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
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

impl From<Kdf> for bitwarden_crypto::Kdf {
    fn from(kdf: Kdf) -> Self {
        match kdf {
            Kdf::PBKDF2 { iterations } => bitwarden_crypto::Kdf::PBKDF2 {
                iterations: iterations.get(),
            },
            Kdf::Argon2id {
                iterations,
                memory,
                parallelism,
            } => bitwarden_crypto::Kdf::Argon2id {
                iterations: iterations.get(),
                memory: memory.get(),
                parallelism: parallelism.get(),
            },
        }
    }
}

impl From<&Kdf> for bitwarden_crypto::Kdf {
    fn from(kdf: &Kdf) -> Self {
        match kdf {
            Kdf::PBKDF2 { iterations } => bitwarden_crypto::Kdf::PBKDF2 {
                iterations: *iterations,
            },
            Kdf::Argon2id {
                iterations,
                memory,
                parallelism,
            } => bitwarden_crypto::Kdf::Argon2id {
                iterations: *iterations,
                memory: *memory,
                parallelism: *parallelism,
            },
        }
    }
}

#[cfg(feature = "internal")]
impl TryFrom<PreloginResponseModel> for Kdf {
    type Error = Error;

    fn try_from(response: PreloginResponseModel) -> Result<Kdf> {
        use crate::util::{
            default_argon2_iterations, default_argon2_memory, default_argon2_parallelism,
            default_pbkdf2_iterations,
        };

        let kdf = response.kdf.ok_or("KDF not found")?;

        Ok(match kdf {
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
        })
    }
}
