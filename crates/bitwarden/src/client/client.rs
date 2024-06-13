use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use bitwarden_core::VaultLocked;
#[cfg(feature = "internal")]
pub use bitwarden_crypto::Kdf;
use bitwarden_crypto::SymmetricCryptoKey;
#[cfg(feature = "internal")]
use bitwarden_crypto::{AsymmetricEncString, EncString, MasterKey};
use chrono::Utc;
use reqwest::header::{self, HeaderValue};
use uuid::Uuid;

#[cfg(feature = "internal")]
use crate::client::flags::Flags;
use crate::{
    auth::AccessToken,
    client::{
        client_settings::{ClientSettings, DeviceType},
        encryption_settings::EncryptionSettings,
    },
    error::Result,
};

#[derive(Debug, Clone)]
pub(crate) struct ApiConfigurations {
    pub identity: bitwarden_api_identity::apis::configuration::Configuration,
    pub api: bitwarden_api_api::apis::configuration::Configuration,
    pub device_type: DeviceType,
}

#[derive(Debug)]
pub(crate) enum LoginMethod {
    #[cfg(feature = "internal")]
    User(UserLoginMethod),
    // TODO: Organizations supports api key
    // Organization(OrganizationLoginMethod),
    ServiceAccount(ServiceAccountLoginMethod),
}

#[derive(Debug)]
#[cfg(feature = "internal")]
pub(crate) enum UserLoginMethod {
    Username {
        client_id: String,
        email: String,
        kdf: Kdf,
    },
    ApiKey {
        client_id: String,
        client_secret: String,

        email: String,
        kdf: Kdf,
    },
}

#[derive(Debug)]
pub(crate) enum ServiceAccountLoginMethod {
    AccessToken {
        access_token: AccessToken,
        organization_id: Uuid,
        state_file: Option<PathBuf>,
    },
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

/// The main struct to interact with the Bitwarden SDK.
#[derive(Debug)]
pub struct Client {
    pub(crate) tokens: RwLock<Tokens>,
    pub(crate) login_method: RwLock<Option<Arc<LoginMethod>>>,

    #[cfg(feature = "internal")]
    flags: RwLock<Flags>,

    /// Use Client::get_api_configurations() to access this.
    /// It should only be used directly in renew_token
    #[doc(hidden)]
    pub(crate) __api_configurations: RwLock<Arc<ApiConfigurations>>,

    /// Reqwest client useable for external integrations like email forwarders, HIBP.
    #[allow(unused)]
    pub(crate) external_client: reqwest::Client,

    encryption_settings: RwLock<Option<Arc<EncryptionSettings>>>,
}

impl Client {
    pub fn new(settings_input: Option<ClientSettings>) -> Self {
        let settings = settings_input.unwrap_or_default();

        fn new_client_builder() -> reqwest::ClientBuilder {
            #[allow(unused_mut)]
            let mut client_builder = reqwest::Client::builder();

            #[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))]
            {
                client_builder =
                    client_builder.use_preconfigured_tls(rustls_platform_verifier::tls_config());
            }

            client_builder
        }

        let external_client = new_client_builder().build().expect("Build should not fail");

        let mut headers = header::HeaderMap::new();
        headers.append(
            "Device-Type",
            HeaderValue::from_str(&(settings.device_type as u8).to_string())
                .expect("All numbers are valid ASCII"),
        );
        let client_builder = new_client_builder().default_headers(headers);

        let client = client_builder.build().expect("Build should not fail");

        let identity = bitwarden_api_identity::apis::configuration::Configuration {
            base_path: settings.identity_url,
            user_agent: Some(settings.user_agent.clone()),
            client: client.clone(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        };

        let api = bitwarden_api_api::apis::configuration::Configuration {
            base_path: settings.api_url,
            user_agent: Some(settings.user_agent),
            client,
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        };

        Self {
            tokens: RwLock::new(Tokens::default()),
            login_method: RwLock::new(None),
            #[cfg(feature = "internal")]
            flags: RwLock::new(Flags::default()),
            __api_configurations: RwLock::new(Arc::new(ApiConfigurations {
                identity,
                api,
                device_type: settings.device_type,
            })),
            external_client,
            encryption_settings: RwLock::new(None),
        }
    }

    #[cfg(feature = "internal")]
    pub fn load_flags(&self, flags: std::collections::HashMap<String, bool>) {
        *self.flags.write().expect("RwLock is not poisoned") = Flags::load_from_map(flags);
    }

    #[cfg(feature = "internal")]
    pub(crate) fn get_flags(&self) -> Flags {
        self.flags.read().expect("RwLock is not poisoned").clone()
    }

    pub(crate) async fn get_api_configurations(&self) -> Arc<ApiConfigurations> {
        // At the moment we ignore the error result from the token renewal, if it fails,
        // the token will end up expiring and the next operation is going to fail anyway.
        self.auth().renew_token().await.ok();
        self.__api_configurations
            .read()
            .expect("RwLock is not poisoned")
            .clone()
    }

    #[cfg(feature = "internal")]
    pub(crate) fn get_http_client(&self) -> &reqwest::Client {
        &self.external_client
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
            Some(LoginMethod::ServiceAccount(ServiceAccountLoginMethod::AccessToken {
                organization_id,
                ..
            })) => Some(*organization_id),
            _ => None,
        }
    }

    pub(crate) fn get_encryption_settings(&self) -> Result<Arc<EncryptionSettings>, VaultLocked> {
        self.encryption_settings
            .read()
            .expect("RwLock is not poisoned")
            .clone()
            .ok_or(VaultLocked)
    }

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

        let mut inner: ApiConfigurations = guard.as_ref().clone();
        inner.identity.oauth_access_token = Some(token.clone());
        inner.api.oauth_access_token = Some(token);

        *guard = Arc::new(inner);
    }

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
    pub(crate) fn initialize_user_crypto_master_key(
        &self,
        master_key: MasterKey,
        user_key: EncString,
        private_key: EncString,
    ) -> Result<()> {
        *self
            .encryption_settings
            .write()
            .expect("RwLock is not poisoned") = Some(Arc::new(EncryptionSettings::new(
            master_key,
            user_key,
            private_key,
        )?));

        Ok(())
    }

    #[cfg(feature = "internal")]
    pub(crate) fn initialize_user_crypto_decrypted_key(
        &self,
        user_key: SymmetricCryptoKey,
        private_key: EncString,
    ) -> Result<()> {
        *self
            .encryption_settings
            .write()
            .expect("RwLock is not poisoned") = Some(Arc::new(
            EncryptionSettings::new_decrypted_key(user_key, private_key)?,
        ));

        Ok(())
    }

    #[cfg(feature = "internal")]
    pub(crate) fn initialize_user_crypto_pin(
        &self,
        pin_key: MasterKey,
        pin_protected_user_key: EncString,
        private_key: EncString,
    ) -> Result<()> {
        let decrypted_user_key = pin_key.decrypt_user_key(pin_protected_user_key)?;
        self.initialize_user_crypto_decrypted_key(decrypted_user_key, private_key)
    }

    pub(crate) fn initialize_crypto_single_key(&self, key: SymmetricCryptoKey) {
        *self
            .encryption_settings
            .write()
            .expect("RwLock is not poisoned") =
            Some(Arc::new(EncryptionSettings::new_single_key(key)));
    }

    #[cfg(feature = "internal")]
    pub(crate) fn initialize_org_crypto(
        &self,
        org_keys: Vec<(Uuid, AsymmetricEncString)>,
    ) -> Result<Arc<EncryptionSettings>> {
        let mut guard = self
            .encryption_settings
            .write()
            .expect("RwLock is not poisoned");

        let Some(enc) = guard.as_mut() else {
            return Err(VaultLocked.into());
        };

        let mut enc: EncryptionSettings = enc.as_ref().clone();
        enc.set_org_keys(org_keys)?;
        let enc = Arc::new(enc);

        *guard = Some(enc.clone());

        Ok(enc)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reqwest_rustls_platform_verifier_are_compatible() {
        // rustls-platform-verifier is generating a rustls::ClientConfig,
        // which reqwest accepts as a &dyn Any and then downcasts it to a
        // rustls::ClientConfig.

        // This means that if the rustls version of the two crates don't match,
        // the downcast will fail and we will get a runtime error.

        // This tests is added to ensure that it doesn't happen.

        let _ = reqwest::ClientBuilder::new()
            .use_preconfigured_tls(rustls_platform_verifier::tls_config())
            .build()
            .unwrap();
    }
}
