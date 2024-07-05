use bitwarden_vault::{Cipher, CipherView, Fido2CredentialNewView};
use passkey::authenticator::UIHint;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Fido2CallbackError {
    #[error("The operation requires user interaction")]
    UserInterfaceRequired,

    #[error("The operation was cancelled by the user")]
    OperationCancelled,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[async_trait::async_trait]
pub trait Fido2UserInterface: Send + Sync {
    async fn check_user<'a>(
        &self,
        options: CheckUserOptions,
        hint: UIHint<'a, CipherView>,
    ) -> Result<CheckUserResult, Fido2CallbackError>;
    async fn pick_credential_for_authentication(
        &self,
        available_credentials: Vec<CipherView>,
    ) -> Result<CipherView, Fido2CallbackError>;
    async fn check_user_and_pick_credential_for_creation(
        &self,
        options: CheckUserOptions,
        new_credential: Fido2CredentialNewView,
    ) -> Result<(CipherView, CheckUserResult), Fido2CallbackError>;
    async fn is_verification_enabled(&self) -> bool;
}

#[async_trait::async_trait]
pub trait Fido2CredentialStore: Send + Sync {
    async fn find_credentials(
        &self,
        ids: Option<Vec<Vec<u8>>>,
        rip_id: String,
    ) -> Result<Vec<CipherView>, Fido2CallbackError>;

    async fn all_credentials(&self) -> Result<Vec<CipherView>, Fido2CallbackError>;

    async fn save_credential(&self, cred: Cipher) -> Result<(), Fido2CallbackError>;
}

#[derive(Clone)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CheckUserOptions {
    pub require_presence: bool,
    pub require_verification: Verification,
}

#[derive(Clone)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum Verification {
    Discouraged,
    Preferred,
    Required,
}

pub struct CheckUserResult {
    pub user_present: bool,
    pub user_verified: bool,
}
