use bitwarden_core::{Client, Error};

use crate::{ClientVault, PasswordHistory, PasswordHistoryView};

pub struct ClientPasswordHistory<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientPasswordHistory<'a> {
    pub fn encrypt(&self, history_view: PasswordHistoryView) -> Result<PasswordHistory, Error> {
        let history = self
            .client
            .internal
            .get_crypto_service()
            .encrypt(history_view)?;

        Ok(history)
    }

    pub fn decrypt_list(
        &self,
        history: Vec<PasswordHistory>,
    ) -> Result<Vec<PasswordHistoryView>, Error> {
        let history_view = self
            .client
            .internal
            .get_crypto_service()
            .decrypt_list(&history)?;

        Ok(history_view)
    }
}

impl<'a> ClientVault<'a> {
    pub fn password_history(&'a self) -> ClientPasswordHistory<'a> {
        ClientPasswordHistory {
            client: self.client,
        }
    }
}
