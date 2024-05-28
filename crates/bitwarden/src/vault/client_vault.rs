use crate::{error::Result, Client};

use super::sync::{sync, SyncRequest, SyncResponse};

pub struct ClientVault<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientVault<'a> {
    pub async fn sync(&mut self, input: &SyncRequest) -> Result<SyncResponse> {
        sync(self.client, input).await
    }
}

impl<'a> Client {
    pub fn vault(&'a mut self) -> ClientVault<'a> {
        ClientVault { client: self }
    }
}
