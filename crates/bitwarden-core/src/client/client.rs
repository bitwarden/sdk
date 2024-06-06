use std::path::PathBuf;

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
    error::{Error, Result},
};

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

/// The main struct to interact with the Bitwarden SDK.
#[derive(Debug)]
pub struct Client {
    token: Option<String>,
    pub(crate) refresh_token: Option<String>,
    pub(crate) token_expires_on: Option<i64>,
    pub(crate) login_method: Option<LoginMethod>,

    #[cfg(feature = "internal")]
    flags: Flags,

    /// Use Client::get_api_configurations() to access this.
    /// It should only be used directly in renew_token
    #[doc(hidden)]
    pub(crate) __api_configurations: ApiConfigurations,

    encryption_settings: Option<EncryptionSettings>,
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
            token: None,
            refresh_token: None,
            token_expires_on: None,
            login_method: None,
            #[cfg(feature = "internal")]
            flags: Flags::default(),
            __api_configurations: ApiConfigurations {
                identity,
                api,
                external_client,
                device_type: settings.device_type,
            },
            encryption_settings: None,
        }
    }

    #[cfg(feature = "internal")]
    pub fn load_flags(&mut self, flags: std::collections::HashMap<String, bool>) {
        self.flags = Flags::load_from_map(flags);
    }

    #[cfg(feature = "internal")]
    pub(crate) fn get_flags(&self) -> &Flags {
        &self.flags
    }

    pub async fn get_api_configurations(&mut self) -> &ApiConfigurations {
        // At the moment we ignore the error result from the token renewal, if it fails,
        // the token will end up expiring and the next operation is going to fail anyway.
        self.auth().renew_token().await.ok();
        &self.__api_configurations
    }

    #[cfg(feature = "internal")]
    pub fn get_http_client(&self) -> &reqwest::Client {
        &self.__api_configurations.external_client
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

    pub fn get_encryption_settings(&self) -> Result<&EncryptionSettings> {
        self.encryption_settings.as_ref().ok_or(Error::VaultLocked)
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

    pub fn get_kdf(&self) -> Result<Kdf> {
        match &self.login_method {
            Some(LoginMethod::User(
                UserLoginMethod::Username { kdf, .. } | UserLoginMethod::ApiKey { kdf, .. },
            )) => Ok(kdf.clone()),
            _ => Err(Error::NotAuthenticated),
        }
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
