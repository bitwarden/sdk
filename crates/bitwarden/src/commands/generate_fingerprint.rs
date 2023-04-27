use log::{debug, info};

use crate::crypto::fingerprint;

use base64::Engine;
use crate::util::BASE64_ENGINE;
use crate::{error::Result, sdk::request::fingerprint_request::FingerprintRequest};

pub(crate) fn generate_fingerprint(input: &FingerprintRequest) -> Result<String> {
    info!("Generating fingerprint");
    debug!("{:?}", input);

    let key = BASE64_ENGINE.decode(&input.public_key)?;

    fingerprint(&input.fingerprint_material, &key)
}
