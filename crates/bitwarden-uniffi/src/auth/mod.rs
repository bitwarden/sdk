use std::sync::Arc;

use bitwarden::auth::password::MasterPasswordPolicyOptions;

use crate::Client;

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
            .read()
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
            .read()
            .await
            .auth()
            .satisfies_policy(password, strength, &policy)
            .await
    }
}
