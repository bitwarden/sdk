use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fido2VaultItem {
    pub cipher_id: String,
    pub name: String,
    pub fido2_credential: Option<Fido2CredentialView>,
}

impl Fido2VaultItem {
    pub fn new(
        cipher_id: String,
        name: String,
        fido2_credential: Option<Fido2CredentialView>,
    ) -> Self {
        Self {
            cipher_id,
            name,
            fido2_credential,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fido2CredentialView {
    credential_id: String,
    key_type: String,
    key_algorithm: String,
    key_curve: String,
    key_value: String,
    rp_id: String,
    user_handle: Option<String>,
    user_name: Option<String>,
    counter: String,
    rp_name: Option<String>,
    user_display_name: Option<String>,
    discoverable: String,
    creation_date: DateTime<Utc>,
}
