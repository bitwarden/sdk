use super::client_vault::ClientVault;
use crate::{
    crypto::{Decryptable, Encryptable},
    error::Result,
    vault::{PasswordHistory, PasswordHistoryView},
    Client,
};

pub struct ClientPasswordHistory<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientPasswordHistory<'a> {
    pub async fn encrypt(&self, history_view: PasswordHistoryView) -> Result<PasswordHistory> {
        let enc = self.client.get_encryption_settings()?;

        let history = history_view.encrypt(enc)?;

        Ok(history)
    }

    pub async fn decrypt_list(
        &self,
        history: Vec<PasswordHistory>,
    ) -> Result<Vec<PasswordHistoryView>> {
        let enc = self.client.get_encryption_settings()?;

        let history_view: Result<_> = history.into_iter().map(|h| h.decrypt(enc)).collect();

        history_view
    }
}

impl<'a> ClientVault<'a> {
    pub fn password_history(&'a self) -> ClientPasswordHistory<'a> {
        ClientPasswordHistory {
            client: self.client,
        }
    }
}
