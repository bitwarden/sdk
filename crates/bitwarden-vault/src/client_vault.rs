use bitwarden_core::Client;

use crate::{
    sync::{sync, SyncError},
    SyncRequest, SyncResponse,
};

pub struct ClientVault<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientVault<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
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
