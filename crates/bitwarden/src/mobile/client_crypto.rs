#[cfg(feature = "internal")]
use bitwarden_crypto::{AsymmetricEncString, EncString, SensitiveString};

use crate::Client;
#[cfg(feature = "internal")]
use crate::{
    error::Result,
    mobile::crypto::{
        derive_pin_key, derive_pin_user_key, enroll_admin_password_reset, get_user_encryption_key,
        initialize_org_crypto, initialize_user_crypto, update_password, DerivePinKeyResponse,
        InitOrgCryptoRequest, InitUserCryptoRequest, UpdatePasswordResponse,
    },
};

pub struct ClientCrypto<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientCrypto<'a> {
    #[cfg(feature = "internal")]
    pub async fn initialize_user_crypto(&mut self, req: InitUserCryptoRequest) -> Result<()> {
        initialize_user_crypto(self.client, req).await
    }

    #[cfg(feature = "internal")]
    pub async fn initialize_org_crypto(&mut self, req: InitOrgCryptoRequest) -> Result<()> {
        initialize_org_crypto(self.client, req).await
    }

    #[cfg(feature = "internal")]
    pub async fn get_user_encryption_key(&mut self) -> Result<SensitiveString> {
        get_user_encryption_key(self.client).await
    }

    #[cfg(feature = "internal")]
    pub async fn update_password(
        &mut self,
        new_password: String,
    ) -> Result<UpdatePasswordResponse> {
        update_password(self.client, new_password)
    }

    #[cfg(feature = "internal")]
    pub async fn derive_pin_key(&mut self, pin: String) -> Result<DerivePinKeyResponse> {
        derive_pin_key(self.client, pin)
    }

    #[cfg(feature = "internal")]
    pub async fn derive_pin_user_key(&mut self, encrypted_pin: EncString) -> Result<EncString> {
        derive_pin_user_key(self.client, encrypted_pin)
    }

    #[cfg(feature = "internal")]
    pub fn enroll_admin_password_reset(
        &mut self,
        public_key: String,
    ) -> Result<AsymmetricEncString> {
        enroll_admin_password_reset(self.client, public_key)
    }
}

impl<'a> Client {
    pub fn crypto(&'a mut self) -> ClientCrypto<'a> {
        ClientCrypto { client: self }
    }
}
