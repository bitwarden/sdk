use std::path::Path;

use super::client_vault::ClientVault;
use crate::{
    crypto::{Decryptable, EncString, Encryptable},
    error::Result,
    vault::{Send, SendListView, SendView},
    Client,
};

pub struct ClientSends<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientSends<'a> {
    pub async fn decrypt(&self, send: Send) -> Result<SendView> {
        let enc = self.client.get_encryption_settings()?;

        let send_view = send.decrypt(enc, &None)?;

        Ok(send_view)
    }

    pub async fn decrypt_list(&self, sends: Vec<Send>) -> Result<Vec<SendListView>> {
        let enc = self.client.get_encryption_settings()?;

        let send_views = sends.decrypt(enc, &None)?;

        Ok(send_views)
    }

    pub async fn decrypt_file(
        &self,
        send: Send,
        encrypted_file_path: &Path,
        decrypted_file_path: &Path,
    ) -> Result<()> {
        let enc = self.client.get_encryption_settings()?;
        let key = Send::get_key(&send.key, enc, &None)?;

        crate::crypto::decrypt_file(key, encrypted_file_path, decrypted_file_path)?;
        Ok(())
    }

    pub async fn decrypt_buffer(&self, send: Send, encrypted_buffer: &[u8]) -> Result<Vec<u8>> {
        let enc = self.client.get_encryption_settings()?;
        let enc = Send::get_encryption(&send.key, enc, &None)?;

        let buf = EncString::from_buffer(encrypted_buffer)?;

        enc.decrypt_bytes(&buf, &None)
    }

    pub async fn encrypt(&self, send_view: SendView) -> Result<Send> {
        let enc = self.client.get_encryption_settings()?;

        let send = send_view.encrypt(enc, &None)?;

        Ok(send)
    }

    pub async fn encrypt_file(
        &self,
        send: Send,
        decrypted_file_path: &Path,
        encrypted_file_path: &Path,
    ) -> Result<()> {
        let data = std::fs::read(decrypted_file_path).unwrap();
        let encrypted = self.encrypt_buffer(send, &data).await?;
        std::fs::write(encrypted_file_path, encrypted)?;
        Ok(())
    }

    pub async fn encrypt_buffer(&self, send: Send, buffer: &[u8]) -> Result<Vec<u8>> {
        let enc = self.client.get_encryption_settings()?;
        let enc = Send::get_encryption(&send.key, enc, &None)?;

        let enc = enc.encrypt(buffer, &None)?;
        enc.to_buffer()
    }
}

impl<'a> ClientVault<'a> {
    pub fn sends(&'a self) -> ClientSends<'a> {
        ClientSends {
            client: self.client,
        }
    }
}
