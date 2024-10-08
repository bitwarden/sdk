use std::sync::{Arc, RwLock};

use bitwarden_crypto::service::CryptoService;
#[cfg(any(feature = "internal", feature = "secrets"))]
use bitwarden_crypto::SymmetricCryptoKey;
#[cfg(feature = "internal")]
use bitwarden_crypto::{AsymmetricEncString, EncString, Kdf, MasterKey, PinKey};
use chrono::Utc;
use uuid::Uuid;

#[cfg(feature = "secrets")]
use super::login_method::ServiceAccountLoginMethod;
use crate::{
    auth::renew::renew_token,
    client::{encryption_settings::EncryptionSettings, login_method::LoginMethod},
    error::Result,
    key_management::{AsymmetricKeyRef, SymmetricKeyRef},
    DeviceType,
};
#[cfg(feature = "internal")]
use crate::{
    client::encryption_settings::EncryptionSettingsError,
    client::{flags::Flags, login_method::UserLoginMethod},
    error::Error,
};

#[derive(Debug, Clone)]
pub struct ApiConfigurations {
    pub identity: bitwarden_api_identity::apis::configuration::Configuration,
    pub api: bitwarden_api_api::apis::configuration::Configuration,
    pub device_type: DeviceType,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Tokens {
    // These two fields are always written to, but they are not read
    // from the secrets manager SDK.
    #[cfg_attr(not(feature = "internal"), allow(dead_code))]
    access_token: Option<String>,
    pub(crate) expires_on: Option<i64>,

    #[cfg_attr(not(feature = "internal"), allow(dead_code))]
    pub(crate) refresh_token: Option<String>,
}

#[derive(Debug)]
pub struct InternalClient {
    pub(crate) tokens: RwLock<Tokens>,
    pub(crate) login_method: RwLock<Option<Arc<LoginMethod>>>,

    #[cfg(feature = "internal")]
    pub(super) flags: RwLock<Flags>,

    /// Use Client::get_api_configurations().await to access this.
    /// It should only be used directly in renew_token
    #[doc(hidden)]
    pub(crate) __api_configurations: RwLock<Arc<ApiConfigurations>>,

    /// Reqwest client useable for external integrations like email forwarders, HIBP.
    #[allow(unused)]
    pub(crate) external_client: reqwest::Client,

    pub(super) crypto_service: CryptoService<SymmetricKeyRef, AsymmetricKeyRef>,
}

impl InternalClient {
    #[cfg(feature = "internal")]
    pub fn load_flags(&self, flags: std::collections::HashMap<String, bool>) {
        *self.flags.write().expect("RwLock is not poisoned") = Flags::load_from_map(flags);
    }

    #[cfg(feature = "internal")]
    pub fn get_flags(&self) -> Flags {
        self.flags.read().expect("RwLock is not poisoned").clone()
    }

    #[cfg(feature = "internal")]
    pub(crate) fn get_login_method(&self) -> Option<Arc<LoginMethod>> {
        self.login_method
            .read()
            .expect("RwLock is not poisoned")
            .clone()
    }

    pub fn get_access_token_organization(&self) -> Option<Uuid> {
        match self
            .login_method
            .read()
            .expect("RwLock is not poisoned")
            .as_deref()
        {
            #[cfg(feature = "secrets")]
            Some(LoginMethod::ServiceAccount(ServiceAccountLoginMethod::AccessToken {
                organization_id,
                ..
            })) => Some(*organization_id),
            _ => None,
        }
    }

    #[cfg(any(feature = "internal", feature = "secrets"))]
    pub(crate) fn set_login_method(&self, login_method: LoginMethod) {
        use log::debug;

        debug! {"setting login method: {:#?}", login_method}
        *self.login_method.write().expect("RwLock is not poisoned") = Some(Arc::new(login_method));
    }

    pub(crate) fn set_tokens(&self, token: String, refresh_token: Option<String>, expires_in: u64) {
        *self.tokens.write().expect("RwLock is not poisoned") = Tokens {
            access_token: Some(token.clone()),
            expires_on: Some(Utc::now().timestamp() + expires_in as i64),
            refresh_token,
        };
        let mut guard = self
            .__api_configurations
            .write()
            .expect("RwLock is not poisoned");

        let inner = Arc::make_mut(&mut guard);
        inner.identity.oauth_access_token = Some(token.clone());
        inner.api.oauth_access_token = Some(token);
    }

    #[cfg(feature = "internal")]
    pub fn is_authed(&self) -> bool {
        let is_token_set = self
            .tokens
            .read()
            .expect("RwLock is not poisoned")
            .access_token
            .is_some();
        let is_login_method_set = self
            .login_method
            .read()
            .expect("RwLock is not poisoned")
            .is_some();

        is_token_set || is_login_method_set
    }

    #[cfg(feature = "internal")]
    pub fn get_kdf(&self) -> Result<Kdf> {
        match self
            .login_method
            .read()
            .expect("RwLock is not poisoned")
            .as_deref()
        {
            Some(LoginMethod::User(
                UserLoginMethod::Username { kdf, .. } | UserLoginMethod::ApiKey { kdf, .. },
            )) => Ok(kdf.clone()),
            _ => Err(Error::NotAuthenticated),
        }
    }

    pub async fn get_api_configurations(&self) -> Arc<ApiConfigurations> {
        // At the moment we ignore the error result from the token renewal, if it fails,
        // the token will end up expiring and the next operation is going to fail anyway.
        renew_token(self).await.ok();
        self.__api_configurations
            .read()
            .expect("RwLock is not poisoned")
            .clone()
    }

    #[cfg(feature = "internal")]
    pub fn get_http_client(&self) -> &reqwest::Client {
        &self.external_client
    }

    pub fn get_crypto_service(&self) -> &CryptoService<SymmetricKeyRef, AsymmetricKeyRef> {
        &self.crypto_service
    }

    #[cfg(feature = "internal")]
    pub(crate) fn initialize_user_crypto_master_key(
        &self,
        master_key: MasterKey,
        user_key: EncString,
        private_key: EncString,
    ) -> Result<(), EncryptionSettingsError> {
        let user_key = master_key.decrypt_user_key(user_key)?;
        EncryptionSettings::new_decrypted_key(user_key, private_key, &self.crypto_service)?;

        Ok(())
    }

    #[cfg(feature = "internal")]
    pub(crate) fn initialize_user_crypto_decrypted_key(
        &self,
        user_key: SymmetricCryptoKey,
        private_key: EncString,
    ) -> Result<(), EncryptionSettingsError> {
        EncryptionSettings::new_decrypted_key(user_key, private_key, &self.crypto_service)?;

        Ok(())
    }

    #[cfg(feature = "internal")]
    pub(crate) fn initialize_user_crypto_pin(
        &self,
        pin_key: PinKey,
        pin_protected_user_key: EncString,
        private_key: EncString,
    ) -> Result<(), EncryptionSettingsError> {
        let decrypted_user_key = pin_key.decrypt_user_key(pin_protected_user_key)?;
        self.initialize_user_crypto_decrypted_key(decrypted_user_key, private_key)
    }

    #[cfg(feature = "secrets")]
    pub(crate) fn initialize_crypto_single_key(&self, key: SymmetricCryptoKey) {
        EncryptionSettings::new_single_key(key, &self.crypto_service);
    }

    #[cfg(feature = "internal")]
    pub fn initialize_org_crypto(
        &self,
        org_keys: Vec<(Uuid, AsymmetricEncString)>,
    ) -> Result<(), EncryptionSettingsError> {
        EncryptionSettings::set_org_keys(org_keys, &self.crypto_service)?;
        Ok(())
    }
}
