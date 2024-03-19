use passkey::types::{
    ctap2::{self, StatusCode},
    webauthn::PublicKeyCredentialDescriptor,
};

use super::VaultItem;

#[async_trait::async_trait]
pub trait Fido2CredentialStore {
    async fn find_credentials(
        &self,
        ids: Option<&[PublicKeyCredentialDescriptor]>,
        rp_id: &str,
    ) -> std::result::Result<Vec<VaultItem>, StatusCode>;

    async fn save_credential(
        &mut self,
        cred: VaultItem,
        user: ctap2::make_credential::PublicKeyCredentialUserEntity,
        rp: ctap2::make_credential::PublicKeyCredentialRpEntity,
    ) -> Result<(), StatusCode>;
}
