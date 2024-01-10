use std::path::Path;

use super::client_vault::ClientVault;
use crate::{
    crypto::{EncString, KeyDecryptable, KeyEncryptable, LocateKey},
    error::{Error, Result},
    vault::{Attachment, Cipher},
    Client,
};

pub struct ClientAttachments<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientAttachments<'a> {
    pub async fn decrypt_file(
        &self,
        cipher: Cipher,
        attachment: Attachment,
        encrypted_file_path: &Path,
        decrypted_file_path: &Path,
    ) -> Result<()> {
        let data = std::fs::read(encrypted_file_path).unwrap();
        let decrypted = self.decrypt_buffer(cipher, attachment, &data).await?;
        std::fs::write(decrypted_file_path, decrypted)?;
        Ok(())
    }

    pub async fn decrypt_buffer(
        &self,
        cipher: Cipher,
        attachment: Attachment,
        encrypted_buffer: &[u8],
    ) -> Result<Vec<u8>> {
        let enc = self.client.get_encryption_settings()?;

        let key = cipher.locate_key(enc, &None).ok_or(Error::VaultLocked)?;
        let attachment_key = attachment.get_attachment_file_key(key, &cipher)?;

        let buf = EncString::from_buffer(encrypted_buffer)?;
        buf.decrypt_with_key(&attachment_key)
    }

    pub async fn encrypt_file(
        &self,
        cipher: Cipher,
        attachment: Attachment,
        decrypted_file_path: &Path,
        encrypted_file_path: &Path,
    ) -> Result<()> {
        let data = std::fs::read(decrypted_file_path).unwrap();
        let encrypted = self.encrypt_buffer(cipher, attachment, &data).await?;
        std::fs::write(encrypted_file_path, encrypted)?;
        Ok(())
    }

    pub async fn encrypt_buffer(
        &self,
        cipher: Cipher,
        attachment: Attachment,
        buffer: &[u8],
    ) -> Result<Vec<u8>> {
        let enc = self.client.get_encryption_settings()?;

        let key = cipher.locate_key(enc, &None).ok_or(Error::VaultLocked)?;
        let attachment_key = attachment.get_attachment_file_key(key, &cipher)?;

        let enc = buffer.encrypt_with_key(&attachment_key)?;
        enc.to_buffer()
    }
}

impl<'a> ClientVault<'a> {
    pub fn attachments(&'a self) -> ClientAttachments<'a> {
        ClientAttachments {
            client: self.client,
        }
    }
}
