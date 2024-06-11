use bitwarden_core::Client;

use crate::{
    sync::{sync, SyncError},
    SyncRequest, SyncResponse,
};

pub struct ClientVault<'a> {
    pub(crate) client: &'a mut Client,
}

impl<'a> ClientVault<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        Self { client }
    }

    pub async fn sync(&mut self, input: &SyncRequest) -> Result<SyncResponse, SyncError> {
        sync(self.client, input).await
    }
}

pub trait ClientVaultExt<'a> {
    fn vault(&'a mut self) -> ClientVault<'a>;
}

impl<'a> ClientVaultExt<'a> for Client {
    fn vault(&'a mut self) -> ClientVault<'a> {
        ClientVault::new(self)
    }
}
