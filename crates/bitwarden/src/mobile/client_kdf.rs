use crate::{client::kdf::Kdf, error::Result, mobile::kdf::hash_password, Client};

pub struct ClientKdf<'a> {
    pub(crate) client: &'a crate::Client,
}

impl<'a> ClientKdf<'a> {
    pub async fn hash_password(
        &self,
        email: String,
        password: String,
        kdf_params: Kdf,
    ) -> Result<String> {
        hash_password(self.client, email, password, kdf_params).await
    }
}

impl<'a> Client {
    pub fn kdf(&'a self) -> ClientKdf<'a> {
        ClientKdf { client: self }
    }
}
