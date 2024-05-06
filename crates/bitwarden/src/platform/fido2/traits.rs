use crate::{
    error::Result,
    vault::{login::Fido2Credential, Cipher, CipherView},
};

#[async_trait::async_trait]
pub trait UserInterface: Send + Sync {
    async fn check_user(
        &self,
        options: CheckUserOptions,
        credential: Option<CipherView>,
    ) -> Result<CheckUserResult>;
    async fn pick_credential_for_authentication(
        &self,
        available_credentials: Vec<Cipher>,
    ) -> Result<CipherView>;
    async fn pick_credential_for_creation(
        &self,
        available_credentials: Vec<Cipher>,
        new_credential: Fido2Credential,
    ) -> Result<CipherView>;
}

#[async_trait::async_trait]
pub trait CredentialStore: Send + Sync {
    async fn find_credentials(
        &self,
        ids: Option<Vec<Vec<u8>>>,
        rip_id: String,
    ) -> Result<Vec<Cipher>>;

    async fn save_credential(&self, cred: Cipher) -> Result<()>;
}

#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum CheckUserOptions {
    RequirePresence(bool),
    RequireVerification(Verification),
}
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
