use base64::{engine::general_purpose::STANDARD, Engine};
use log::{debug, info};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{crypto::fingerprint, error::Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct FingerprintRequest {
    /// The input material, used in the fingerprint generation process.
    pub fingerprint_material: String,
    /// The user's public key encoded with base64.
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FingerprintResponse {
    pub fingerprint: String,
}

pub(crate) fn generate_fingerprint(input: &FingerprintRequest) -> Result<FingerprintResponse> {
    info!("Generating fingerprint");
    debug!("{:?}", input);

    let key = STANDARD.decode(&input.public_key)?;

    Ok(FingerprintResponse {
        fingerprint: fingerprint(&input.fingerprint_material, &key)?,
    })
}
