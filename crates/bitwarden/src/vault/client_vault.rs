use super::sync::{sync, SyncRequest, SyncResponse};
use crate::{error::Result, Client};

pub struct ClientVault<'a> {
    pub(crate) client: &'a crate::Client,
}

impl<'a> ClientVault<'a> {
    pub async fn sync(&self, input: &SyncRequest) -> Result<SyncResponse> {
        sync(self.client, input).await
    }
}

impl<'a> Client {
    pub fn vault(&'a self) -> ClientVault<'a> {
        ClientVault { client: self }
    }
}
