use std::path::Path;

use bitwarden_core::{Client, Error};
use bitwarden_crypto::EncString;

use crate::{
    Attachment, AttachmentEncryptResult, AttachmentFile, AttachmentFileView, AttachmentView,
    Cipher, ClientVault,
};

pub struct ClientAttachments<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientAttachments<'a> {
    pub fn encrypt_buffer(
        &self,
        cipher: Cipher,
        attachment: AttachmentView,
        buffer: &[u8],
    ) -> Result<AttachmentEncryptResult, Error> {
        let crypto = self.client.internal.get_crypto_service();

        Ok(crypto.encrypt(AttachmentFileView {
            cipher,
            attachment,
            contents: buffer,
        })?)
    }
    pub fn encrypt_file(
        &self,
        cipher: Cipher,
        attachment: AttachmentView,
        decrypted_file_path: &Path,
        encrypted_file_path: &Path,
    ) -> Result<Attachment, Error> {
        let data = std::fs::read(decrypted_file_path)?;
        let AttachmentEncryptResult {
            attachment,
            contents,
        } = self.encrypt_buffer(cipher, attachment, &data)?;
        std::fs::write(encrypted_file_path, contents)?;
        Ok(attachment)
    }

    pub fn decrypt_buffer(
        &self,
        cipher: Cipher,
        attachment: Attachment,
        encrypted_buffer: &[u8],
    ) -> Result<Vec<u8>, Error> {
        let crypto = self.client.internal.get_crypto_service();

        Ok(crypto.decrypt(&AttachmentFile {
            cipher,
            attachment,
            contents: EncString::from_buffer(encrypted_buffer)?,
        })?)
    }
    pub fn decrypt_file(
        &self,
        cipher: Cipher,
        attachment: Attachment,
        encrypted_file_path: &Path,
        decrypted_file_path: &Path,
    ) -> Result<(), Error> {
        let data = std::fs::read(encrypted_file_path)?;
        let decrypted = self.decrypt_buffer(cipher, attachment, &data)?;
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
