use std::sync::Arc;

use bitwarden::vault::{PasswordHistory, PasswordHistoryView};

use crate::{Client, Result};

#[derive(uniffi::Object)]
pub struct ClientPasswordHistory(pub Arc<Client>);

#[uniffi::export]
impl ClientPasswordHistory {
    /// Encrypt password history
    pub fn encrypt(&self, password_history: PasswordHistoryView) -> Result<PasswordHistory> {
        Ok(self
            .0
             .0
            .vault()
            .password_history()
            .encrypt(password_history)?)
    }

    /// Decrypt password history
    pub fn decrypt_list(&self, list: Vec<PasswordHistory>) -> Result<Vec<PasswordHistoryView>> {
        Ok(self.0 .0.vault().password_history().decrypt_list(list)?)
    }
}
