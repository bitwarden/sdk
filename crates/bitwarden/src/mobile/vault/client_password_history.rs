use crate::{crypto::Encryptable, error::Result, Client};

use super::{
    client_vault::ClientVault,
    password_history::{PasswordHistoryEncryptRequest, PasswordHistoryEncryptResponse},
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
}

impl<'a> ClientVault<'a> {
    pub fn password_history(&'a self) -> ClientPasswordHistory<'a> {
        ClientPasswordHistory {
            client: self.client,
        }
    }
}
