use std::sync::Arc;

use bitwarden::mobile::vault::{
    PasswordHistoryDecryptListRequest, PasswordHistoryDecryptListResponse,
    PasswordHistoryEncryptRequest, PasswordHistoryEncryptResponse,
};

use crate::{Client, Result};

#[derive(uniffi::Object)]
pub struct ClientPasswordHistory(pub Arc<Client>);

#[uniffi::export]
impl ClientPasswordHistory {
    /// Encrypt password history
    pub async fn encrypt(
        &self,
        req: PasswordHistoryEncryptRequest,
    ) -> Result<PasswordHistoryEncryptResponse> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .password_history()
            .encrypt(req)
            .await?)
    }

    /// Decrypt password history
    pub async fn decrypt_list(
        &self,
        req: PasswordHistoryDecryptListRequest,
    ) -> Result<PasswordHistoryDecryptListResponse> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .password_history()
            .decrypt_list(req)
            .await?)
    }
}
