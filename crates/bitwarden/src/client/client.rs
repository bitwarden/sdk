use std::time::{Duration, Instant};

use reqwest::header::{self};
use uuid::Uuid;

#[cfg(feature = "secrets")]
use crate::auth::login::{AccessTokenLoginRequest, AccessTokenLoginResponse};
#[cfg(feature = "internal")]
use crate::{
    client::kdf::Kdf,
    crypto::EncString,
    platform::{
        generate_fingerprint, get_user_api_key, sync, FingerprintRequest, FingerprintResponse,
        SecretVerificationRequest, SyncRequest, SyncResponse, UserApiKeyResponse,
    },
};
use crate::{
    client::{
        client_settings::{ClientSettings, DeviceType},
        encryption_settings::EncryptionSettings,
    },
    crypto::SymmetricCryptoKey,
    error::{Error, Result},
};

#[derive(Debug)]
pub(crate) struct ApiConfigurations {
    pub identity: bitwarden_api_identity::apis::configuration::Configuration,
    pub api: bitwarden_api_api::apis::configuration::Configuration,
    pub device_type: DeviceType,
}

#[derive(Debug, Clone)]
pub(crate) enum LoginMethod {
    #[cfg(feature = "internal")]
    User(UserLoginMethod),
    // TODO: Organizations supports api key
    // Organization(OrganizationLoginMethod),
    ServiceAccount(ServiceAccountLoginMethod),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub(crate) enum ServiceAccountLoginMethod {
    AccessToken {
        service_account_id: Uuid,
        client_secret: String,
        organization_id: Uuid,
    },
}

#[derive(Debug)]
pub struct Client {
    token: Option<String>,
    pub(crate) refresh_token: Option<String>,
    pub(crate) token_expires_in: Option<Instant>,
    pub(crate) login_method: Option<LoginMethod>,

    /// Use Client::get_api_configurations() to access this.
    /// It should only be used directly in renew_token
    #[doc(hidden)]
    pub(crate) __api_configurations: ApiConfigurations,

    encryption_settings: Option<EncryptionSettings>,
}

impl Client {
    pub fn new(settings_input: Option<ClientSettings>) -> Self {
        let settings = settings_input.unwrap_or_default();

        let headers = header::HeaderMap::new();

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

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
            token_expires_in: None,
            login_method: None,
            __api_configurations: ApiConfigurations {
                identity,
                api,
                device_type: settings.device_type,
            },
            encryption_settings: None,
        }
    }

    pub(crate) async fn get_api_configurations(&mut self) -> &ApiConfigurations {
        // At the moment we ignore the error result from the token renewal, if it fails,
        // the token will end up expiring and the next operation is going to fail anyway.
        self.auth().renew_token().await.ok();
        &self.__api_configurations
    }

    #[cfg(feature = "secrets")]
    #[deprecated(note = "Use auth().login_access_token() instead")]
    pub async fn access_token_login(
        &mut self,
        input: &AccessTokenLoginRequest,
    ) -> Result<AccessTokenLoginResponse> {
        self.auth().login_access_token(input).await
    }

    #[cfg(feature = "internal")]
    pub async fn sync(&mut self, input: &SyncRequest) -> Result<SyncResponse> {
        sync(self, input).await
    }

    #[cfg(feature = "internal")]
    pub async fn get_user_api_key(
        &mut self,
        input: &SecretVerificationRequest,
    ) -> Result<UserApiKeyResponse> {
        get_user_api_key(self, input).await
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

    pub(crate) fn get_encryption_settings(&self) -> Result<&EncryptionSettings> {
        self.encryption_settings.as_ref().ok_or(Error::VaultLocked)
    }

    #[cfg(feature = "mobile")]
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
        login_method: LoginMethod,
    ) {
        self.token = Some(token.clone());
        self.refresh_token = refresh_token;
        self.token_expires_in = Some(Instant::now() + Duration::from_secs(expires_in));
        self.login_method = Some(login_method);
        self.__api_configurations.identity.oauth_access_token = Some(token.clone());
        self.__api_configurations.api.oauth_access_token = Some(token);
    }

    #[cfg(feature = "internal")]
    pub fn is_authed(&self) -> bool {
        self.token.is_some() || self.login_method.is_some()
    }

    #[cfg(feature = "internal")]
    pub(crate) fn initialize_user_crypto(
        &mut self,
        password: &str,
        user_key: EncString,
        private_key: EncString,
    ) -> Result<&EncryptionSettings> {
        let login_method = match &self.login_method {
            Some(LoginMethod::User(u)) => u,
            _ => return Err(Error::NotAuthenticated),
        };

        self.encryption_settings = Some(EncryptionSettings::new(
            login_method,
            password,
            user_key,
            private_key,
        )?);
        Ok(self.encryption_settings.as_ref().unwrap())
    }

    #[cfg(feature = "mobile")]
    pub(crate) fn initialize_user_crypto_decrypted_key(
        &mut self,
        decrypted_user_key: &str,
        private_key: EncString,
    ) -> Result<&EncryptionSettings> {
        let user_key = decrypted_user_key.parse::<SymmetricCryptoKey>()?;
        self.encryption_settings = Some(EncryptionSettings::new_decrypted_key(
            user_key,
            private_key,
        )?);
        Ok(self.encryption_settings.as_ref().unwrap())
    }

    pub(crate) fn initialize_crypto_single_key(
        &mut self,
        key: SymmetricCryptoKey,
    ) -> &EncryptionSettings {
        self.encryption_settings = Some(EncryptionSettings::new_single_key(key));
        self.encryption_settings.as_ref().unwrap()
    }

    #[cfg(feature = "internal")]
    pub(crate) fn initialize_org_crypto(
        &mut self,
        org_keys: Vec<(Uuid, EncString)>,
    ) -> Result<&EncryptionSettings> {
        let enc = self
            .encryption_settings
            .as_mut()
            .ok_or(Error::VaultLocked)?;

        enc.set_org_keys(org_keys)?;
        Ok(self.encryption_settings.as_ref().unwrap())
    }

    #[cfg(feature = "internal")]
    pub fn fingerprint(&mut self, input: &FingerprintRequest) -> Result<FingerprintResponse> {
        generate_fingerprint(input)
    }
}
