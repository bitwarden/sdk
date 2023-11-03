use std::{path::Path, sync::Arc};

use bitwarden::vault::{self, SendListView, SendView};

use crate::{Client, Result};

#[derive(uniffi::Object)]
pub struct ClientSends(pub Arc<Client>);

#[uniffi::export]
impl ClientSends {
    /// Encrypt send
    pub async fn encrypt(&self, send: SendView) -> Result<vault::Send> {
        Ok(self.0 .0.read().await.vault().sends().encrypt(send).await?)
    }

    /// Encrypt a send file in memory
    pub async fn encrypt_buffer(&self, send: vault::Send, buffer: Vec<u8>) -> Result<Vec<u8>> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .sends()
            .encrypt_buffer(send, &buffer)
            .await?)
    }

    /// Encrypt a send file located in the file system
    pub async fn encrypt_file(
        &self,
        send: vault::Send,
        decrypted_file_path: String,
        encrypted_file_path: String,
    ) -> Result<()> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .sends()
            .encrypt_file(
                send,
                Path::new(&decrypted_file_path),
                Path::new(&encrypted_file_path),
            )
            .await?)
    }

    /// Decrypt send
    pub async fn decrypt(&self, send: vault::Send) -> Result<SendView> {
        Ok(self.0 .0.read().await.vault().sends().decrypt(send).await?)
    }

    /// Decrypt send list
    pub async fn decrypt_list(&self, sends: Vec<vault::Send>) -> Result<Vec<SendListView>> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .sends()
            .decrypt_list(sends)
            .await?)
    }

    /// Decrypt a send file in memory
    pub async fn decrypt_buffer(&self, send: vault::Send, buffer: Vec<u8>) -> Result<Vec<u8>> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .sends()
            .decrypt_buffer(send, &buffer)
            .await?)
    }

    /// Decrypt a send file located in the file system
    pub async fn decrypt_file(
        &self,
        send: vault::Send,
        encrypted_file_path: String,
        decrypted_file_path: String,
    ) -> Result<()> {
        Ok(self
            .0
             .0
            .read()
            .await
            .vault()
            .sends()
            .decrypt_file(
                send,
                Path::new(&encrypted_file_path),
                Path::new(&decrypted_file_path),
            )
            .await?)
    }
}
