use super::VaultItem;
use crate::error::Result;
use passkey::types::{ctap2, webauthn::PublicKeyCredentialDescriptor, Bytes};

pub struct FindCredentialsParams {
    pub ids: Vec<Bytes>,
    pub rp_id: String,
}

pub struct SaveCredentialParams {
    pub cred: VaultItem,
    pub user: ctap2::make_credential::PublicKeyCredentialUserEntity,
    pub rp: ctap2::make_credential::PublicKeyCredentialRpEntity,
}

#[async_trait::async_trait]
pub trait Fido2CredentialStore {
    async fn find_credentials(&self, params: FindCredentialsParams) -> Result<Vec<VaultItem>>;

    async fn save_credential(&mut self, params: SaveCredentialParams) -> Result<()>;
}
