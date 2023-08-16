use std::sync::Arc;

use bitwarden::mobile::vault::{
    CipherDecryptListRequest, CipherDecryptListResponse, CipherDecryptRequest,
    CipherDecryptResponse, CipherEncryptRequest, CipherEncryptResponse,
};

use crate::{Client, Result};

#[derive(uniffi::Object)]
pub struct ClientCiphers(pub Arc<Client>);

#[uniffi::export]
impl ClientCiphers {
    /// Encrypt cipher
    pub async fn encrypt(&self, req: CipherEncryptRequest) -> Result<CipherEncryptResponse> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .ciphers()
            .encrypt(req)
            .await?)
    }

    /// Decrypt cipher
    pub async fn decrypt(&self, req: CipherDecryptRequest) -> Result<CipherDecryptResponse> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .ciphers()
            .decrypt(req)
            .await?)
    }

    /// Decrypt cipher list
    pub async fn decrypt_list(
        &self,
        req: CipherDecryptListRequest,
    ) -> Result<CipherDecryptListResponse> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .ciphers()
            .decrypt_list(req)
            .await?)
    }
}
