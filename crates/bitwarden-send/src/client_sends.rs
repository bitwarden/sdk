use std::path::Path;

use bitwarden_core::{Client, Error};
use bitwarden_crypto::{EncString, KeyDecryptable, KeyEncryptable};

use crate::{Send, SendListView, SendView};

pub struct ClientSends<'a> {
    client: &'a Client,
}

impl<'a> ClientSends<'a> {
    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn decrypt(&self, send: Send) -> Result<SendView, Error> {
        let enc = self.client.internal.get_encryption_settings()?;
        let key = enc.get_key(&None)?;

        let send_view = send.decrypt_with_key(key)?;

        Ok(send_view)
    }

    pub fn decrypt_list(&self, sends: Vec<Send>) -> Result<Vec<SendListView>, Error> {
        let enc = self.client.internal.get_encryption_settings()?;
        let key = enc.get_key(&None)?;

        let send_views = sends.decrypt_with_key(key)?;

        Ok(send_views)
    }

    pub fn decrypt_file(
        &self,
        send: Send,
        encrypted_file_path: &Path,
        decrypted_file_path: &Path,
    ) -> Result<(), Error> {
        let data = std::fs::read(encrypted_file_path)?;
        let decrypted = self.decrypt_buffer(send, &data)?;
        std::fs::write(decrypted_file_path, decrypted)?;
        Ok(())
    }

    pub fn decrypt_buffer(&self, send: Send, encrypted_buffer: &[u8]) -> Result<Vec<u8>, Error> {
        let enc = self.client.internal.get_encryption_settings()?;
        let key = enc.get_key(&None)?;
        let key = Send::get_key(&send.key, key)?;

        let buf = EncString::from_buffer(encrypted_buffer)?;
        Ok(buf.decrypt_with_key(&key)?)
    }

    pub fn encrypt(&self, send_view: SendView) -> Result<Send, Error> {
        let enc = self.client.internal.get_encryption_settings()?;
        let key = enc.get_key(&None)?;

        let send = send_view.encrypt_with_key(key)?;

        Ok(send)
    }

    pub fn encrypt_file(
        &self,
        send: Send,
        decrypted_file_path: &Path,
        encrypted_file_path: &Path,
    ) -> Result<(), Error> {
        let data = std::fs::read(decrypted_file_path)?;
        let encrypted = self.encrypt_buffer(send, &data)?;
        std::fs::write(encrypted_file_path, encrypted)?;
        Ok(())
    }

    pub fn encrypt_buffer(&self, send: Send, buffer: &[u8]) -> Result<Vec<u8>, Error> {
        let enc = self.client.internal.get_encryption_settings()?;
        let key = enc.get_key(&None)?;
        let key = Send::get_key(&send.key, key)?;

        let encrypted = buffer.encrypt_with_key(&key)?;
        Ok(encrypted.to_buffer()?)
    }
}

pub trait ClientSendsExt<'a> {
    fn sends(&'a self) -> ClientSends<'a>;
}

impl<'a> ClientSendsExt<'a> for Client {
    fn sends(&'a self) -> ClientSends<'a> {
        ClientSends::new(self)
    }
}
