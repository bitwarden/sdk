use std::path::Path;

use bitwarden_core::VaultLocked;
use bitwarden_crypto::{EncString, KeyDecryptable, KeyEncryptable, LocateKey};
use bitwarden_vault::{
    Attachment, AttachmentEncryptResult, AttachmentFile, AttachmentFileView, AttachmentView, Cipher,
};
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    vault::ClientVault,
    Client,
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
    ) -> Result<AttachmentEncryptResult> {
        let enc = self.client.get_encryption_settings()?;
        let key = cipher.locate_key(&enc, &None).ok_or(VaultLocked)?;

        Ok(AttachmentFileView {
            cipher,
            attachment,
            contents: buffer,
        }
        .encrypt_with_key(key)?)
    }

    /// Temporary method used for re-encrypting attachments using the attachment share function when
    /// moving a cipher between organizations
    pub fn encrypt_buffer_org_id(
        &self,
        cipher: Cipher,
        attachment: AttachmentView,
        new_org_id: Uuid,
        buffer: &[u8],
    ) -> Result<AttachmentEncryptResult> {
        let enc = self.client.get_encryption_settings()?;
        let key = cipher
            .locate_key(&enc, &Some(new_org_id))
            .ok_or(VaultLocked)?;

        Ok(AttachmentFileView {
            cipher,
            attachment,
            contents: buffer,
        }
        .encrypt_with_key(key)?)
    }

    pub fn encrypt_file(
        &self,
        cipher: Cipher,
        attachment: AttachmentView,
        decrypted_file_path: &Path,
        encrypted_file_path: &Path,
    ) -> Result<Attachment> {
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
    ) -> Result<Vec<u8>> {
        let enc = self.client.get_encryption_settings()?;
        let key = cipher.locate_key(&enc, &None).ok_or(VaultLocked)?;

        AttachmentFile {
            cipher,
            attachment,
            contents: EncString::from_buffer(encrypted_buffer)?,
        }
        .decrypt_with_key(key)
        .map_err(Error::Crypto)
    }
    pub fn decrypt_file(
        &self,
        cipher: Cipher,
        attachment: Attachment,
        encrypted_file_path: &Path,
        decrypted_file_path: &Path,
    ) -> Result<()> {
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
