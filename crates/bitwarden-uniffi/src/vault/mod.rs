use std::sync::Arc;

use bitwarden::vault::TotpResponse;
use chrono::{DateTime, Utc};

use crate::Client;

pub mod ciphers;
pub mod collections;
pub mod folders;
pub mod password_history;
pub mod sends;

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

    /// Sends operations
    pub fn sends(self: Arc<Self>) -> Arc<sends::ClientSends> {
        Arc::new(sends::ClientSends(self.0.clone()))
    }

    /// Generate a TOTP code from a provided key.
    ///
    /// The key can be either:
    /// - A base32 encoded string
    /// - OTP Auth URI
    /// - Steam URI
    pub async fn generate_totp(&self, key: String, time: Option<DateTime<Utc>>) -> TotpResponse {
        self.0 .0.read().await.vault().generate_totp(key, time).await
    }
}
