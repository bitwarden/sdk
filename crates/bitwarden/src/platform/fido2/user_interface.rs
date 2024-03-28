use super::Fido2VaultItem;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NewCredentialParams {
    pub credential_name: String,
    pub user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NewCredentialResult {
    pub vault_item: Fido2VaultItem,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PickCredentialParams {
    pub ids: Vec<String>,
    pub rp_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PickCredentialResult {
    pub vault_item: Fido2VaultItem,
}

#[async_trait::async_trait]
pub trait Fido2UserInterface {
    async fn confirm_new_credential(
        &self,
        params: NewCredentialParams,
    ) -> Result<NewCredentialResult>;

    async fn pick_credential(&self, params: PickCredentialParams) -> Result<PickCredentialResult>;

    async fn check_user_verification(&self) -> bool;

    async fn check_user_presence(&self) -> bool;

    fn is_presence_enabled(&self) -> bool;

    fn is_verification_enabled(&self) -> Option<bool>;
}
