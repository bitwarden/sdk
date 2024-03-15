use crate::error::Result;

impl VaultItem {
    pub fn new(cipher_id: String, name: String) -> Self {
        Self { cipher_id, name }
    }
}

#[derive(Default)]
pub struct VaultItem {
    cipher_id: String,
    name: String,
}

#[async_trait::async_trait]
pub trait Fido2GetAssertionUserInterface {
    async fn pick_credential(&self, ids: Vec<String>, rp_id: &str) -> Result<VaultItem>;
}

pub struct NewCredentialParams {
    pub credential_name: String,
    pub user_name: String,
    pub user_verification: bool,
}

pub struct NewCredentialResult {
    pub cipher_id: String,
    pub user_verified: bool,
}

#[async_trait::async_trait]
pub trait Fido2MakeCredentialUserInterface {
    async fn confirm_new_credential(
        &self,
        params: NewCredentialParams,
    ) -> Result<NewCredentialResult>;
}
