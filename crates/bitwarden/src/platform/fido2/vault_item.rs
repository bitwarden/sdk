use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fido2VaultItem {
    pub cipher_id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
