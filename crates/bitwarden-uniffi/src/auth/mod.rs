use std::sync::Arc;

use bitwarden::{
    auth::{password::MasterPasswordPolicyOptions, RegisterKeyResponse},
    client::kdf::Kdf,
    crypto::HashPurpose,
};

use crate::{error::Result, Client};

#[derive(uniffi::Object)]
pub struct ClientAuth(pub(crate) Arc<Client>);

#[uniffi::export]
impl ClientAuth {
    /// **API Draft:** Calculate Password Strength
    pub async fn password_strength(
        &self,
        password: String,
        email: String,
        additional_inputs: Vec<String>,
    ) -> u8 {
        self.0
             .0
            .write()
            .await
            .auth()
            .password_strength(password, email, additional_inputs)
            .await
    }

    /// **API Draft:** Evaluate if the provided password satisfies the provided policy
    pub async fn satisfies_policy(
        &self,
        password: String,
        strength: u8,
        policy: MasterPasswordPolicyOptions,
    ) -> bool {
        self.0
             .0
            .write()
            .await
            .auth()
            .satisfies_policy(password, strength, &policy)
            .await
    }

    /// Hash the user password
    pub async fn hash_password(
        &self,
        email: String,
        password: String,
        kdf_params: Kdf,
        purpose: HashPurpose,
    ) -> Result<String> {
        Ok(self
            .0
             .0
            .read()
            .await
            .kdf()
            .hash_password(email, password, kdf_params, purpose)
            .await?)
    }

    /// Generate keys needed for registration process
    pub async fn make_register_keys(
        &self,
        email: String,
        password: String,
        kdf: Kdf,
    ) -> Result<RegisterKeyResponse> {
        Ok(self
            .0
             .0
            .write()
            .await
            .auth()
            .make_register_keys(email, password, kdf)?)
    }

    /// Validate the user password
    pub async fn validate_password(&self, password: String) -> Result<bool> {
        Ok(self
            .0
             .0
            .write()
            .await
            .auth()
            .validate_password(password, "".to_string())
            .await?)
    }
}
