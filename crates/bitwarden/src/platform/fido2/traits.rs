use passkey::authenticator::UIHint;

use crate::{
    error::Result,
    vault::{Cipher, CipherView, Fido2CredentialNewView},
};

#[async_trait::async_trait]
pub trait Fido2UserInterface: Send + Sync {
    async fn check_user<'a>(
        &self,
        options: CheckUserOptions,
        hint: UIHint<'a, CipherView>,
    ) -> Result<CheckUserResult>;
    async fn pick_credential_for_authentication(
        &self,
        available_credentials: Vec<CipherView>,
    ) -> Result<CipherView>;
    async fn pick_credential_for_creation(
        &self,
        available_credentials: Vec<CipherView>,
        new_credential: Fido2CredentialNewView,
    ) -> Result<CipherView>;
    async fn is_verification_enabled(&self) -> bool;
}

#[async_trait::async_trait]
pub trait Fido2CredentialStore: Send + Sync {
    async fn find_credentials(
        &self,
        ids: Option<Vec<Vec<u8>>>,
        rip_id: String,
    ) -> Result<Vec<CipherView>>;

    async fn save_credential(&self, cred: Cipher) -> Result<()>;
}

#[derive(Clone)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct CheckUserOptions {
    pub require_presence: bool,
    pub require_verification: Verification,
}

#[derive(Clone)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum Verification {
    Discouraged,
    Preferred,
    Required,
}

pub struct CheckUserResult {
    pub user_present: bool,
    pub user_verified: bool,
}
