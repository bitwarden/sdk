use crate::{
    crypto::{Decryptable, Encryptable},
    error::Result,
    Client,
};

use super::{
    client_vault::ClientVault,
    password_history::{
        PasswordHistoryDecryptListRequest, PasswordHistoryDecryptListResponse,
        PasswordHistoryEncryptRequest, PasswordHistoryEncryptResponse,
    },
};

pub struct ClientPasswordHistory<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientPasswordHistory<'a> {
    pub async fn encrypt(
        &self,
        req: PasswordHistoryEncryptRequest,
    ) -> Result<PasswordHistoryEncryptResponse> {
        let enc = self.client.get_encryption_settings()?;

        let history = req.history.encrypt(enc, &None)?;

        Ok(PasswordHistoryEncryptResponse { history })
    }

    pub async fn decrypt_list(
        &self,
        req: PasswordHistoryDecryptListRequest,
    ) -> Result<PasswordHistoryDecryptListResponse> {
        let enc = self.client.get_encryption_settings()?;

        let history = req.history.decrypt(enc, &None)?;

        Ok(PasswordHistoryDecryptListResponse { history })
    }
}

impl<'a> ClientVault<'a> {
    pub fn password_history(&'a self) -> ClientPasswordHistory<'a> {
        ClientPasswordHistory {
            client: self.client,
        }
    }
}
