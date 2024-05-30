use super::{
    repository::CipherSqliteRepository,
    sync::{sync, SyncRequest, SyncResponse},
};
use crate::{error::Result, vault::cipher::repository::CipherRepository, Client};

pub struct ClientVault<'a> {
    pub(crate) client: &'a mut crate::Client,
    pub cipher_repository: Box<dyn CipherRepository + Send>,
}

impl<'a> ClientVault<'a> {
    pub async fn sync(&mut self, input: &SyncRequest) -> Result<SyncResponse> {
        sync(self.client, input).await
    }
}

impl<'a> Client {
    pub fn vault(&'a mut self) -> ClientVault<'a> {
        let t = self.sqlite_conn.clone();
        ClientVault {
            client: self,
            cipher_repository: Box::new(CipherSqliteRepository::new(t)),
        }
    }
}

pub struct ClientRepositories {}

impl std::fmt::Debug for ClientRepositories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientRepositories").finish()
    }
}
