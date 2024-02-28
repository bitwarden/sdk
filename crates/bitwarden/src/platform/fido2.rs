use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fido2ClientGetAssertionRequest {
    /// WebAuthn-compatible JSON string of the PublicKeyCredentialRequestOptions
    pub webauthn_json: String,
}

pub(crate) fn client_get_assertion(request: Fido2ClientGetAssertionRequest) -> Result<String> {
    Ok("client_get_assertion".to_string())
}
