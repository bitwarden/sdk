use std::path::Path;

use bitwarden_crypto::{EncString, KeyDecryptable, KeyEncryptable, LocateKey};

use super::client_vault::ClientVault;
use crate::{
    error::{Error, Result},
    vault::{
        Attachment, AttachmentEncryptResult, AttachmentFile, AttachmentFileView, AttachmentView,
        Cipher,
    },
    Client,
};

pub struct ClientAttachments<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientAttachments<'a> {
    pub async fn encrypt_buffer(
        &self,
        cipher: Cipher,
        attachment: AttachmentView,
        buffer: &[u8],
    ) -> Result<AttachmentEncryptResult> {
        let enc = self.client.get_encryption_settings()?;
        let key = cipher.locate_key(enc, &None).ok_or(Error::VaultLocked)?;

        Ok(AttachmentFileView {
            cipher,
            attachment,
            contents: buffer,
        }
        .encrypt_with_key(key)?)
    }
    pub async fn encrypt_file(
        &self,
        cipher: Cipher,
        attachment: AttachmentView,
        decrypted_file_path: &Path,
        encrypted_file_path: &Path,
    ) -> Result<Attachment> {
        let data = std::fs::read(decrypted_file_path).unwrap();
        let AttachmentEncryptResult {
            attachment,
            contents,
        } = self.encrypt_buffer(cipher, attachment, &data).await?;
        std::fs::write(encrypted_file_path, contents)?;
        Ok(attachment)
    }

    pub async fn decrypt_buffer(
        &self,
        cipher: Cipher,
        attachment: Attachment,
        encrypted_buffer: &[u8],
    ) -> Result<Vec<u8>> {
        let enc = self.client.get_encryption_settings()?;
        let key = cipher.locate_key(enc, &None).ok_or(Error::VaultLocked)?;

        AttachmentFile {
            cipher,
            attachment,
            contents: EncString::from_buffer(encrypted_buffer)?,
        }
        .decrypt_with_key(key)
        .map_err(Error::Crypto)
    }
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
}

impl<'a> ClientVault<'a> {
    pub fn attachments(&'a self) -> ClientAttachments<'a> {
        ClientAttachments {
            client: self.client,
        }
    }
}
