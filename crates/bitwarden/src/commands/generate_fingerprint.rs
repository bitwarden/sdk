use log::{debug, info};

use crate::crypto::fingerprint;
use crate::sdk::response::fingerprint_response::FingerprintResponse;

use crate::util::BASE64_ENGINE;
use crate::{error::Result, sdk::request::fingerprint_request::FingerprintRequest};
use base64::Engine;

pub(crate) fn generate_fingerprint(input: &FingerprintRequest) -> Result<FingerprintResponse> {
    info!("Generating fingerprint");
    debug!("{:?}", input);

    let key = BASE64_ENGINE.decode(&input.public_key)?;

    fingerprint(&input.fingerprint_material, &key)
        .map(|fingerprint| FingerprintResponse { fingerprint })
}
