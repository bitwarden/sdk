use std::sync::Arc;

use crate::Client;

pub mod ciphers;
pub mod collections;
pub mod folders;
pub mod password_history;

#[derive(uniffi::Object)]
pub struct ClientVault(Arc<Client>);

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

    /// Ciphers operations
    pub fn password_history(self: Arc<Self>) -> Arc<password_history::ClientPasswordHistory> {
        Arc::new(password_history::ClientPasswordHistory(self.0.clone()))
    }
}
