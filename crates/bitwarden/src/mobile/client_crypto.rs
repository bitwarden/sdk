use crate::{error::Result, Client};

use super::crypto::{
    initialize_org_crypto, initialize_user_crypto, InitOrgCryptoRequest, InitUserCryptoRequest,
};

pub struct ClientCrypto<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientCrypto<'a> {
    pub async fn initialize_user_crypto(&mut self, req: InitUserCryptoRequest) -> Result<()> {
        initialize_user_crypto(self.client, req).await
    }

    pub async fn initialize_org_crypto(&mut self, req: InitOrgCryptoRequest) -> Result<()> {
        initialize_org_crypto(self.client, req).await
    }
}

impl<'a> Client {
    pub fn crypto(&'a mut self) -> ClientCrypto<'a> {
        ClientCrypto { client: self }
    }
}
