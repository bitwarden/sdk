#[cfg(feature = "internal")]
use bitwarden_crypto::{AsymmetricEncString, EncString};

use super::crypto::{derive_key_connector, DeriveKeyConnectorRequest};
use crate::{client::encryption_settings::EncryptionSettingsError, Client};
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
    pub async fn initialize_user_crypto(
        &self,
        req: InitUserCryptoRequest,
    ) -> Result<(), EncryptionSettingsError> {
        initialize_user_crypto(self.client, req).await
    }

    pub async fn initialize_org_crypto(
        &self,
        req: InitOrgCryptoRequest,
    ) -> Result<(), EncryptionSettingsError> {
        initialize_org_crypto(self.client, req).await
    }

    pub async fn get_user_encryption_key(&self) -> Result<String> {
        get_user_encryption_key(self.client).await
    }

    pub fn update_password(&self, new_password: String) -> Result<UpdatePasswordResponse> {
        update_password(self.client, new_password)
    }

    pub fn derive_pin_key(&self, pin: String) -> Result<DerivePinKeyResponse> {
        derive_pin_key(self.client, pin)
    }

    pub fn derive_pin_user_key(&self, encrypted_pin: EncString) -> Result<EncString> {
        derive_pin_user_key(self.client, encrypted_pin)
    }

    pub fn enroll_admin_password_reset(&self, public_key: String) -> Result<AsymmetricEncString> {
        enroll_admin_password_reset(self.client, public_key)
    }

    /// Derive the master key for migrating to the key connector
    pub fn derive_key_connector(&self, request: DeriveKeyConnectorRequest) -> Result<String> {
        derive_key_connector(request)
    }
}

impl<'a> Client {
    pub fn crypto(&'a self) -> ClientCrypto<'a> {
        ClientCrypto { client: self }
    }
}
