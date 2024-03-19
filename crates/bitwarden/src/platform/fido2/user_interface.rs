use crate::error::Result;
use passkey::types::webauthn;
use serde::{Deserialize, Serialize};

impl VaultItem {
    pub fn new(cipher_id: String, name: String) -> Self {
        Self { cipher_id, name }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultItem {
    cipher_id: String,
    name: String,
}

pub struct NewCredentialParams {
    pub credential_name: String,
    pub user_name: String,
}

pub struct NewCredentialResult {
    pub vault_item: VaultItem,
}

pub struct PickCredentialParams {
    pub ids: Vec<String>,
    pub rp_id: String,
}

pub struct PickCredentialResult {
    pub vault_item: VaultItem,
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
