use log::{debug, info};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::crypto::fingerprint;
use crate::error::Result;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FingerprintRequest {
    /// The input material, used in the fingerprint generation process.
    pub fingerprint_material: String,
    /// The user's public key
    pub public_key: String,
}

#[allow(dead_code)]
pub(crate) fn generate_fingerprint(input: &FingerprintRequest) -> Result<String> {
    info!("Generating fingerprint");
    debug!("{:?}", input);

    fingerprint(&input.fingerprint_material, input.public_key.as_bytes())
}
