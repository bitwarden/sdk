use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(test, derive(Default))]
pub struct IdentityTokenPayloadResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    token_type: String,
    scope: String,

    pub(crate) encrypted_payload: String,
}
