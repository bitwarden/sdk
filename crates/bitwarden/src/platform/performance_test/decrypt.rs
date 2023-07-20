use std::{str::FromStr, fmt::Display};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{client::encryption_settings::{EncryptionSettings, SymmetricCryptoKey}, crypto::CipherString, error::Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DecryptPerformanceRequest {
    pub cipher_text: String,
    pub key: String,
    pub num_operations: u32,
}

impl Display for DecryptPerformanceRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} cipher_text", self.cipher_text)
    }
}

#[inline]
pub fn decrypt_performance(
    request: &DecryptPerformanceRequest,
) -> Result<DecryptPerformanceResponse> {
    let key = SymmetricCryptoKey::from_str(&request.key)?;
    let encryption_settings = EncryptionSettings::new_single_key(key);
    let cipher = CipherString::from_str(&request.cipher_text)?;
    let mut result = Vec::<String>::new();
    for _ in 0..request.num_operations {
        result.push(encryption_settings.decrypt(&cipher, &None)?);
    }
    Ok(DecryptPerformanceResponse { clear_text: result })
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DecryptPerformanceResponse {
    pub clear_text: Vec<String>,
}
