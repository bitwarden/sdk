use crate::{error::Result, Client};

use super::kdf::{hash_password, set_kdf_params, KdfParamRequest, PasswordHashRequest};

pub struct ClientKdf<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientKdf<'a> {
    pub async fn set_kdf_params(&mut self, req: KdfParamRequest) -> Result<()> {
        set_kdf_params(self.client, req).await
    }

    pub async fn hash_password(&mut self, req: PasswordHashRequest) -> Result<String> {
        hash_password(self.client, req).await
    }
}

impl<'a> Client {
    pub fn kdf(&'a mut self) -> ClientKdf<'a> {
        ClientKdf { client: self }
    }
}
