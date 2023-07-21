use crate::{error::Result, Client};

use super::kdf::{hash_password, PasswordHashRequest};

pub struct ClientKdf<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientKdf<'a> {
    pub async fn hash_password(&mut self, req: PasswordHashRequest) -> Result<String> {
        hash_password(self.client, req).await
    }
}

impl<'a> Client {
    pub fn kdf(&'a mut self) -> ClientKdf<'a> {
        ClientKdf { client: self }
    }
}
