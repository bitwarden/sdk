use crate::{error::Result, Client};

use super::kdf::{get_user_password_hash, set_kdf_params, KdfParamRequest, PasswordHashRequest};

pub struct ClientKdf<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientKdf<'a> {
    pub async fn set_kdf_params(&mut self, req: KdfParamRequest) -> Result<()> {
        set_kdf_params(self.client, req).await
    }

    pub async fn get_user_password_hash(&mut self, req: PasswordHashRequest) -> Result<String> {
        get_user_password_hash(self.client, req).await
    }
}

impl<'a> Client {
    pub fn kdf(&'a mut self) -> ClientKdf<'a> {
        ClientKdf { client: self }
    }
}
