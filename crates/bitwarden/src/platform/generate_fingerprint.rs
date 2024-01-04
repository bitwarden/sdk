use base64::Engine;
use log::info;
use rsa::pkcs8::EncodePublicKey;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{crypto::fingerprint, error::Result, util::BASE64_ENGINE};

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

    let key = BASE64_ENGINE.decode(&input.public_key)?;

    Ok(FingerprintResponse {
        fingerprint: fingerprint(&input.fingerprint_material, &key)?,
    })
}

pub(crate) fn generate_users_fingerprint(
    client: &mut crate::Client,
    fingerprint_material: String,
) -> Result<String> {
    info!("Generating fingerprint");

    let enc_settings = client.get_encryption_settings()?;
    let private_key = enc_settings
        .private_key
        .as_ref()
        .ok_or("Missing private key")?;

    let public_key = private_key
        .to_public_key()
        .to_public_key_der()
        .map_err(|_| "Invalid key")?;

    let fingerprint = fingerprint(&fingerprint_material, public_key.as_bytes())?;

    Ok(fingerprint)
}
