use std::{path::Path, sync::Arc};

use bitwarden_vault::{
    Attachment, AttachmentEncryptResult, AttachmentView, Cipher, ClientVaultExt,
};

use crate::{Client, Result};

#[derive(uniffi::Object)]
pub struct ClientAttachments(pub Arc<Client>);

#[uniffi::export]
impl ClientAttachments {
    /// Encrypt an attachment file in memory
    pub fn encrypt_buffer(
        &self,
        cipher: Cipher,
        attachment: AttachmentView,
        buffer: Vec<u8>,
    ) -> Result<AttachmentEncryptResult> {
        Ok(self
            .0
             .0
            .vault()
            .attachments()
            .encrypt_buffer(cipher, attachment, &buffer)?)
    }

    /// Encrypt an attachment file located in the file system
    pub fn encrypt_file(
        &self,
        cipher: Cipher,
        attachment: AttachmentView,
        decrypted_file_path: String,
        encrypted_file_path: String,
    ) -> Result<Attachment> {
        Ok(self.0 .0.vault().attachments().encrypt_file(
            cipher,
            attachment,
            Path::new(&decrypted_file_path),
            Path::new(&encrypted_file_path),
        )?)
    }
    /// Decrypt an attachment file in memory
    pub fn decrypt_buffer(
        &self,
        cipher: Cipher,
        attachment: Attachment,
        buffer: Vec<u8>,
    ) -> Result<Vec<u8>> {
        Ok(self
            .0
             .0
            .vault()
            .attachments()
            .decrypt_buffer(cipher, attachment, &buffer)?)
    }

    /// Decrypt an attachment file located in the file system
    pub fn decrypt_file(
        &self,
        cipher: Cipher,
        attachment: Attachment,
        encrypted_file_path: String,
        decrypted_file_path: String,
    ) -> Result<()> {
        Ok(self.0 .0.vault().attachments().decrypt_file(
            cipher,
            attachment,
            Path::new(&encrypted_file_path),
            Path::new(&decrypted_file_path),
        )?)
    }
}
