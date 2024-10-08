use std::path::Path;

use bitwarden_core::{Client, Error};
use bitwarden_crypto::{Decryptable, EncString, Encryptable, UsesKey};

use crate::{Send, SendListView, SendView};

pub struct ClientSends<'a> {
    client: &'a Client,
}

impl<'a> ClientSends<'a> {
    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn decrypt(&self, send: Send) -> Result<SendView, Error> {
        let crypto = self.client.internal.get_crypto_service();

        let send_view = crypto.decrypt(&send)?;
        Ok(send_view)
    }

    pub fn decrypt_list(&self, sends: Vec<Send>) -> Result<Vec<SendListView>, Error> {
        let crypto = self.client.internal.get_crypto_service();

        let send_views = crypto.decrypt_list(&sends)?;

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
        let crypto = self.client.internal.get_crypto_service();

        let mut ctx = crypto.context();
        let key = Send::decrypt_key(&mut ctx, &send.key, send.uses_key())?;

        let buf = EncString::from_buffer(encrypted_buffer)?;
        Ok(buf.decrypt(&mut ctx, key)?)
    }

    pub fn encrypt(&self, send_view: SendView) -> Result<Send, Error> {
        let crypto = self.client.internal.get_crypto_service();

        let send = crypto.encrypt(send_view)?;

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
        let crypto = self.client.internal.get_crypto_service();

        let mut ctx = crypto.context();
        let key = Send::decrypt_key(&mut ctx, &send.key, send.uses_key())?;

        let encrypted = buffer.encrypt(&mut ctx, key)?;
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
