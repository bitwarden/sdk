use bitwarden_core::Client;

#[cfg(feature = "state")]
use crate::repository::CipherRepository;
use crate::{
    sync::{sync, SyncError},
    SyncRequest, SyncResponse,
};

pub struct ClientVault<'a> {
    pub(crate) client: &'a Client,
    #[cfg(feature = "state")]
    pub cipher_repository: CipherRepository,
}

impl<'a> ClientVault<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            #[cfg(feature = "state")]
            cipher_repository: CipherRepository::new(client.internal.db.clone()),
        }
    }

    pub async fn sync(&self, input: &SyncRequest) -> Result<SyncResponse, SyncError> {
        sync(self.client, input).await
    }
}

pub trait ClientVaultExt<'a> {
    fn vault(&'a self) -> ClientVault<'a>;
}

impl<'a> ClientVaultExt<'a> for Client {
    fn vault(&'a self) -> ClientVault<'a> {
        ClientVault::new(self)
    }
}
