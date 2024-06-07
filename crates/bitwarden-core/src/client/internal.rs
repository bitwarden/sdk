use crate::{
    error::{Error, Result},
    DeviceType,
};

use super::{
    encryption_settings::EncryptionSettings,
    flags::Flags,
    login_method::{LoginMethod, ServiceAccountLoginMethod, UserLoginMethod},
};

use bitwarden_crypto::Kdf;
#[cfg(feature = "internal")]
use bitwarden_crypto::{AsymmetricEncString, EncString, MasterKey, SymmetricCryptoKey};
use chrono::Utc;
#[cfg(feature = "internal")]
use uuid::Uuid;

#[derive(Debug)]
pub struct ApiConfigurations {
    pub identity: bitwarden_api_identity::apis::configuration::Configuration,
    pub api: bitwarden_api_api::apis::configuration::Configuration,
    /// Reqwest client useable for external integrations like email forwarders, HIBP.
    #[allow(unused)]
    pub external_client: reqwest::Client,
    pub device_type: DeviceType,
}

#[derive(Debug)]
pub struct InternalClient {
    pub(super) token: Option<String>,
    pub(crate) refresh_token: Option<String>,
    pub(crate) token_expires_on: Option<i64>,
    pub(crate) login_method: Option<LoginMethod>,

    #[cfg(feature = "internal")]
    pub(super) flags: Flags,

    /// Use Client::get_api_configurations() to access this.
    /// It should only be used directly in renew_token
    #[doc(hidden)]
    pub(crate) __api_configurations: ApiConfigurations,

    pub(super) encryption_settings: Option<EncryptionSettings>,
}

impl InternalClient {
    #[cfg(feature = "internal")]
    pub fn load_flags(&mut self, flags: std::collections::HashMap<String, bool>) {
        self.flags = Flags::load_from_map(flags);
    }

    #[cfg(feature = "internal")]
    pub(crate) fn get_flags(&self) -> &Flags {
        &self.flags
    }

    #[cfg(feature = "internal")]
    pub(crate) fn get_login_method(&self) -> &Option<LoginMethod> {
        &self.login_method
    }

    pub fn get_access_token_organization(&self) -> Option<Uuid> {
        match self.login_method {
            Some(LoginMethod::ServiceAccount(ServiceAccountLoginMethod::AccessToken {
                organization_id,
                ..
            })) => Some(organization_id),
            _ => None,
        }
    }

    pub(crate) fn set_login_method(&mut self, login_method: LoginMethod) {
        use log::debug;

        debug! {"setting login method: {:#?}", login_method}
        self.login_method = Some(login_method);
    }

    pub(crate) fn set_tokens(
        &mut self,
        token: String,
        refresh_token: Option<String>,
        expires_in: u64,
    ) {
        self.token = Some(token.clone());
        self.refresh_token = refresh_token;
        self.token_expires_on = Some(Utc::now().timestamp() + expires_in as i64);
        self.__api_configurations.identity.oauth_access_token = Some(token.clone());
        self.__api_configurations.api.oauth_access_token = Some(token);
    }

    #[cfg(feature = "internal")]
    pub fn is_authed(&self) -> bool {
        self.token.is_some() || self.login_method.is_some()
    }

    pub fn get_kdf(&self) -> Result<Kdf> {
        match &self.login_method {
            Some(LoginMethod::User(
                UserLoginMethod::Username { kdf, .. } | UserLoginMethod::ApiKey { kdf, .. },
            )) => Ok(kdf.clone()),
            _ => Err(Error::NotAuthenticated),
        }
    }

    pub async fn get_api_configurations(&mut self) -> &ApiConfigurations {
        // At the moment we ignore the error result from the token renewal, if it fails,
        // the token will end up expiring and the next operation is going to fail anyway.
        // self.auth().renew_token().await.ok();
        &self.__api_configurations
    }

    #[cfg(feature = "internal")]
    pub fn get_http_client(&self) -> &reqwest::Client {
        &self.__api_configurations.external_client
    }

    pub fn get_encryption_settings(&self) -> Result<&EncryptionSettings> {
        self.encryption_settings.as_ref().ok_or(Error::VaultLocked)
    }

    #[cfg(feature = "internal")]
    pub(crate) fn initialize_user_crypto_master_key(
        &mut self,
        master_key: MasterKey,
        user_key: EncString,
        private_key: EncString,
    ) -> Result<&EncryptionSettings> {
        Ok(self.encryption_settings.insert(EncryptionSettings::new(
            master_key,
            user_key,
            private_key,
        )?))
    }

    #[cfg(feature = "internal")]
    pub(crate) fn initialize_user_crypto_decrypted_key(
        &mut self,
        user_key: SymmetricCryptoKey,
        private_key: EncString,
    ) -> Result<&EncryptionSettings> {
        Ok(self
            .encryption_settings
            .insert(EncryptionSettings::new_decrypted_key(
                user_key,
                private_key,
            )?))
    }

    #[cfg(feature = "internal")]
    pub(crate) fn initialize_user_crypto_pin(
        &mut self,
        pin_key: MasterKey,
        pin_protected_user_key: EncString,
        private_key: EncString,
    ) -> Result<&EncryptionSettings> {
        let decrypted_user_key = pin_key.decrypt_user_key(pin_protected_user_key)?;
        self.initialize_user_crypto_decrypted_key(decrypted_user_key, private_key)
    }

    pub(crate) fn initialize_crypto_single_key(
        &mut self,
        key: SymmetricCryptoKey,
    ) -> &EncryptionSettings {
        self.encryption_settings
            .insert(EncryptionSettings::new_single_key(key))
    }

    #[cfg(feature = "internal")]
    pub(crate) fn initialize_org_crypto(
        &mut self,
        org_keys: Vec<(Uuid, AsymmetricEncString)>,
    ) -> Result<&EncryptionSettings> {
        let enc = self
            .encryption_settings
            .as_mut()
            .ok_or(Error::VaultLocked)?;

        enc.set_org_keys(org_keys)?;
        Ok(&*enc)
    }
}
