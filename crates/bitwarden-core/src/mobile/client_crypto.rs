#[cfg(feature = "internal")]
use bitwarden_crypto::{AsymmetricEncString, EncString};

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
    pub(crate) client: &'a crate::Client,
}

impl<'a> ClientCrypto<'a> {
    #[cfg(feature = "internal")]
    pub async fn initialize_user_crypto(&self, req: InitUserCryptoRequest) -> Result<()> {
        initialize_user_crypto(self.client, req).await
    }

    #[cfg(feature = "internal")]
    pub async fn initialize_org_crypto(&self, req: InitOrgCryptoRequest) -> Result<()> {
        initialize_org_crypto(self.client, req).await
    }

    #[cfg(feature = "internal")]
    pub async fn get_user_encryption_key(&self) -> Result<String> {
        get_user_encryption_key(self.client).await
    }

    #[cfg(feature = "internal")]
    pub fn update_password(&self, new_password: String) -> Result<UpdatePasswordResponse> {
        update_password(self.client, new_password)
    }

    #[cfg(feature = "internal")]
    pub fn derive_pin_key(&self, pin: String) -> Result<DerivePinKeyResponse> {
        derive_pin_key(self.client, pin)
    }

    #[cfg(feature = "internal")]
    pub fn derive_pin_user_key(&self, encrypted_pin: EncString) -> Result<EncString> {
        derive_pin_user_key(self.client, encrypted_pin)
    }

    #[cfg(feature = "internal")]
    pub fn enroll_admin_password_reset(&self, public_key: String) -> Result<AsymmetricEncString> {
        enroll_admin_password_reset(self.client, public_key)
    }
}

impl<'a> Client {
    pub fn crypto(&'a self) -> ClientCrypto<'a> {
        ClientCrypto { client: self }
    }
}
