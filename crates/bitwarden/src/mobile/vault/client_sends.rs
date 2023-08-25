use std::path::Path;

use crate::{
    crypto::{CipherString, Decryptable},
    error::Result,
    vault::{Send, SendView},
    Client,
};

use super::client_vault::ClientVault;

pub struct ClientSends<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientSends<'a> {
    pub async fn decrypt(&self, send: Send) -> Result<SendView> {
        let enc = self.client.get_encryption_settings()?;

        let send_view = send.decrypt(enc, &None)?;

        Ok(send_view)
    }

    pub async fn decrypt_file(
        &self,
        send: Send,
        encrypted_file_path: &Path,
        decrypted_file_path: &Path,
    ) -> Result<()> {
        let data = std::fs::read(encrypted_file_path).unwrap();
        let decrypted = self.decrypt_buffer(send, &data).await?;
        std::fs::write(decrypted_file_path, decrypted)?;
        Ok(())
    }

    pub async fn decrypt_buffer(&self, send: Send, encrypted_buffer: &[u8]) -> Result<Vec<u8>> {
        let enc = self.client.get_encryption_settings()?;
        let enc = send.get_send_encryption(enc, &None)?;

        let cs = CipherString::from_buffer(encrypted_buffer)?;

        enc.decrypt_bytes(&cs, &None)
    }
}

impl<'a> ClientVault<'a> {
    pub fn sends(&'a self) -> ClientSends<'a> {
        ClientSends {
            client: self.client,
        }
    }
}
