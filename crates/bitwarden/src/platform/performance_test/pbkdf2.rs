use std::num::NonZeroU32;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{client::auth_settings::Kdf, error::Result, crypto::hash_kdf};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Pbkdf2PerformanceRequest {
    pub password: String,
    pub num_operations: u32,
}

pub fn pbkdf2_performance(request: &Pbkdf2PerformanceRequest) -> Result<()> {
    let kdf = Kdf::PBKDF2 {
        iterations: NonZeroU32::new(request.num_operations).unwrap(),
    };
    let _ = hash_kdf(request.password.as_bytes(), "bitwarden_benchmark".as_bytes(), &kdf);
    Ok(())
}
