use crate::{ClientVault, PasswordHistory, PasswordHistoryView};
use bitwarden_crypto::{CryptoError, KeyDecryptable, KeyEncryptable};

use bitwarden_core::{Client, Error};

pub struct ClientPasswordHistory<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientPasswordHistory<'a> {
    pub async fn encrypt(
        &self,
        history_view: PasswordHistoryView,
    ) -> Result<PasswordHistory, Error> {
        let enc = self.client.internal.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(CryptoError::MissingKey)?;

        let history = history_view.encrypt_with_key(key)?;

        Ok(history)
    }

    pub async fn decrypt_list(
        &self,
        history: Vec<PasswordHistory>,
    ) -> Result<Vec<PasswordHistoryView>, Error> {
        let enc = self.client.internal.get_encryption_settings()?;
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
