use chrono::{DateTime, Utc};
use passkey::types::Passkey;
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

impl From<Fido2VaultItem> for Passkey {
    fn from(value: Fido2VaultItem) -> Self {
        value.fido2_credential.unwrap().into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fido2CredentialView {
    pub credential_id: String,
    pub key_type: String,
    pub key_algorithm: String,
    pub key_curve: String,
    pub key_value: String,
    pub rp_id: String,
    pub user_handle: Option<String>,
    pub user_name: Option<String>,
    pub counter: u32,
    pub rp_name: Option<String>,
    pub user_display_name: Option<String>,
    pub discoverable: bool,
    pub creation_date: DateTime<Utc>,
}

impl From<Fido2CredentialView> for Passkey {
    fn from(value: Fido2CredentialView) -> Self {
        todo!()
    }
}

impl From<Passkey> for Fido2CredentialView {
    fn from(value: Passkey) -> Self {
        todo!()
    }
}
