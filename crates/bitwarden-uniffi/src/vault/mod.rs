use std::sync::Arc;

use bitwarden::{
    error::Error,
    vault::{ClientVaultExt, TotpResponse},
};
use bitwarden_vault::CipherListView;
use chrono::{DateTime, Utc};

use crate::{error::Result, Client};

pub mod attachments;
pub mod ciphers;
pub mod collections;
pub mod folders;
pub mod password_history;

#[derive(uniffi::Object)]
pub struct ClientVault(pub(crate) Arc<Client>);

#[uniffi::export]
impl ClientVault {
    /// Folder operations
    pub fn folders(self: Arc<Self>) -> Arc<folders::ClientFolders> {
        Arc::new(folders::ClientFolders(self.0.clone()))
    }

    /// Collections operations
    pub fn collections(self: Arc<Self>) -> Arc<collections::ClientCollections> {
        Arc::new(collections::ClientCollections(self.0.clone()))
    }

    /// Ciphers operations
    pub fn ciphers(self: Arc<Self>) -> Arc<ciphers::ClientCiphers> {
        Arc::new(ciphers::ClientCiphers(self.0.clone()))
    }

    /// Password history operations
    pub fn password_history(self: Arc<Self>) -> Arc<password_history::ClientPasswordHistory> {
        Arc::new(password_history::ClientPasswordHistory(self.0.clone()))
    }

    /// Attachment file operations
    pub fn attachments(self: Arc<Self>) -> Arc<attachments::ClientAttachments> {
        Arc::new(attachments::ClientAttachments(self.0.clone()))
    }

    /// Generate a TOTP code from a provided key.
    ///
    /// The key can be either:
    /// - A base32 encoded string
    /// - OTP Auth URI
    /// - Steam URI
    pub fn generate_totp(&self, key: String, time: Option<DateTime<Utc>>) -> Result<TotpResponse> {
        Ok(self
            .0
             .0
            .vault()
            .generate_totp(key, time)
            .map_err(Error::Totp)?)
    }

    /// Generate a TOTP code from a provided cipher list view.
    pub fn generate_totp_cipher_view(
        &self,
        view: CipherListView,
        time: Option<DateTime<Utc>>,
    ) -> Result<TotpResponse> {
        Ok(self
            .0
             .0
            .vault()
            .generate_totp_cipher_view(view, time)
            .map_err(Error::Totp)?)
    }
}
