use bitwarden_crypto::{CryptoError, KeyDecryptable, KeyEncryptable};
use bitwarden_vault::{PasswordHistory, PasswordHistoryView};

use crate::{error::Result, vault::ClientVault, Client};

pub struct ClientPasswordHistory<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientPasswordHistory<'a> {
    pub fn encrypt(&self, history_view: PasswordHistoryView) -> Result<PasswordHistory> {
        let enc = self.client.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;

        let history = history_view.encrypt_with_key(key)?;

        Ok(history)
    }

    pub fn decrypt_list(&self, history: Vec<PasswordHistory>) -> Result<Vec<PasswordHistoryView>> {
        let enc = self.client.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;

        let history_view = history.decrypt_with_key(key)?;

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
