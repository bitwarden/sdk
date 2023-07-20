use std::str::FromStr;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{client::encryption_settings::{EncryptionSettings, SymmetricCryptoKey}, error::Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EncryptPerformanceRequest {
    pub key: String,
    pub num_operations: u32,
}

pub fn encrypt_performance(request: &EncryptPerformanceRequest) -> Result<()> {
    let key = SymmetricCryptoKey::from_str(&request.key)?;
    let encryption_settings = EncryptionSettings::new_single_key(key);
    for i in 0..request.num_operations {
        let _ = encryption_settings.encrypt(&i.to_string().as_bytes(), &None)?;
    }
    Ok(())
}
