#[cfg(feature = "internal")]
pub(crate) mod generate_fingerprint {
    use log::{debug, info};

    use crate::crypto::fingerprint;

    use crate::{error::Result, sdk::request::fingerprint_request::FingerprintRequest};

    pub(crate) fn generate_fingerprint(input: &FingerprintRequest) -> Result<String> {
        info!("Generating fingerprint");
        debug!("{:?}", input);

        fingerprint(&input.fingerprint_material, input.public_key.as_bytes())
    }
}
