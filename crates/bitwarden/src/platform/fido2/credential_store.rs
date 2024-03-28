use super::Fido2VaultItem;
use crate::error::Result;
use passkey::types::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FindCredentialsParams {
    pub ids: Vec<Bytes>,
    pub rp_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveCredentialParams {
    pub vault_item: Fido2VaultItem,
}

#[async_trait::async_trait]
pub trait Fido2CredentialStore {
    async fn find_credentials(&self, params: FindCredentialsParams) -> Result<Vec<Fido2VaultItem>>;

    async fn save_credential(&mut self, params: SaveCredentialParams) -> Result<()>;
}
