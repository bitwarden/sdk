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

#[cfg(test)]
mod tests {
    use wiremock::{matchers, Mock, ResponseTemplate};

    use crate::{auth::login::AccessTokenLoginRequest, secrets_manager::secrets::*};

    #[tokio::test]
    async fn test_access_token_login() {
        // Create the mock server with the necessary routes for this test
        let (_server, mut client) = crate::util::start_mock(vec![
            Mock::given(matchers::path("/identity/connect/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({
                    "access_token":"eyJhbGciOiJSUzI1NiIsImtpZCI6IjMwMURENkE1MEU4NEUxRDA5MUM4MUQzQjAwQkY5MDEwQzg1REJEOUFSUzI1NiIsInR5cCI6\
                    ImF0K2p3dCIsIng1dCI6Ik1CM1dwUTZFNGRDUnlCMDdBTC1RRU1oZHZabyJ9.eyJuYmYiOjE2NzUxMDM3ODEsImV4cCI6MTY3NTEwNzM4MSwiaXNzIjo\
                    iaHR0cDovL2xvY2FsaG9zdCIsImNsaWVudF9pZCI6ImVjMmMxZDQ2LTZhNGItNDc1MS1hMzEwLWFmOTYwMTMxN2YyZCIsInN1YiI6ImQzNDgwNGNhLTR\
                    mNmMtNDM5Mi04NmI3LWFmOTYwMTMxNzVkMCIsIm9yZ2FuaXphdGlvbiI6ImY0ZTQ0YTdmLTExOTAtNDMyYS05ZDRhLWFmOTYwMTMxMjdjYiIsImp0aSI\
                    6IjU3QUU0NzQ0MzIwNzk1RThGQkQ4MUIxNDA2RDQyNTQyIiwiaWF0IjoxNjc1MTAzNzgxLCJzY29wZSI6WyJhcGkuc2VjcmV0cyJdfQ.GRKYzqgJZHEE\
                    ZHsJkhVZH8zjYhY3hUvM4rhdV3FU10WlCteZdKHrPIadCUh-Oz9DxIAA2HfALLhj1chL4JgwPmZgPcVS2G8gk8XeBmZXowpVWJ11TXS1gYrM9syXbv9j\
                    0JUCdpeshH7e56WnlpVynyUwIum9hmYGZ_XJUfmGtlKLuNjYnawTwLEeR005uEjxq3qI1kti-WFnw8ciL4a6HLNulgiFw1dAvs4c7J0souShMfrnFO3g\
                    SOHff5kKD3hBB9ynDBnJQSFYJ7dFWHIjhqs0Vj-9h0yXXCcHvu7dVGpaiNjNPxbh6YeXnY6UWcmHLDtFYsG2BWcNvVD4-VgGxXt3cMhrn7l3fSYuo32Z\
                    Yk4Wop73XuxqF2fmfmBdZqGI1BafhENCcZw_bpPSfK2uHipfztrgYnrzwvzedz0rjFKbhDyrjzuRauX5dqVJ4ntPeT9g_I5n71gLxiP7eClyAx5RxdF6\
                    He87NwC8i-hLBhugIvLTiDj-Sk9HvMth6zaD0ebxd56wDjq8-CMG_WcgusDqNzKFHqWNDHBXt8MLeTgZAR2rQMIMFZqFgsJlRflbig8YewmNUA9wAU74\
                    TfxLY1foO7Xpg49vceB7C-PlvGi1VtX6F2i0tc_67lA5kWXnnKBPBUyspoIrmAUCwfms5nTTqA9xXAojMhRHAos_OdM",
                    "expires_in":3600,
                    "token_type":"Bearer",
                    "scope":"api.secrets",
                    "encrypted_payload":"2.E9fE8+M/VWMfhhim1KlCbQ==|eLsHR484S/tJbIkM6spnG/HP65tj9A6Tba7kAAvUp+rYuQmGLixiOCfMsqt5OvBctDfvvr/Aes\
                    Bu7cZimPLyOEhqEAjn52jF0eaI38XZfeOG2VJl0LOf60Wkfh3ryAMvfvLj3G4ZCNYU8sNgoC2+IQ==|lNApuCQ4Pyakfo/wwuuajWNaEX/2MW8/3rjXB/V7n+k="})
            )),
            Mock::given(matchers::path("/api/organizations/f4e44a7f-1190-432a-9d4a-af96013127cb/secrets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({
                    "secrets":[{
                            "id":"15744a66-341a-4c62-af50-af960166b6bc",
                            "organizationId":"f4e44a7f-1190-432a-9d4a-af96013127cb",
                            "key":"2.pMS6/icTQABtulw52pq2lg==|XXbxKxDTh+mWiN1HjH2N1w==|Q6PkuT+KX/axrgN9ubD5Ajk2YNwxQkgs3WJM0S0wtG8=",
                            "creationDate":"2023-01-26T21:46:02.2182556Z",
                            "revisionDate":"2023-01-26T21:46:02.2182557Z"
                    }],
                    "projects":[],
                    "object":"SecretsWithProjectsList"
                })
            )),
            Mock::given(matchers::path("/api/secrets/15744a66-341a-4c62-af50-af960166b6bc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({
                    "id":"15744a66-341a-4c62-af50-af960166b6bc",
                    "organizationId":"f4e44a7f-1190-432a-9d4a-af96013127cb",
                    "key":"2.pMS6/icTQABtulw52pq2lg==|XXbxKxDTh+mWiN1HjH2N1w==|Q6PkuT+KX/axrgN9ubD5Ajk2YNwxQkgs3WJM0S0wtG8=",
                    "value":"2.Gl34n9JYABC7V21qHcBzHg==|c1Ds244pob7i+8+MXe4++w==|Shimz/qKMYZmzSFWdeBzFb9dFz7oF6Uv9oqkws7rEe0=",
                    "note":"2.Cn9ABJy7+WfR4uUHwdYepg==|+nbJyU/6hSknoa5dcEJEUg==|1DTp/ZbwGO3L3RN+VMsCHz8XDr8egn/M5iSitGGysPA=",
                    "creationDate":"2023-01-26T21:46:02.2182556Z",
                    "revisionDate":"2023-01-26T21:46:02.2182557Z",
                    "object":"secret"
                })
            ))
        ]).await;

        // Test the login is correct and we store the returned organization ID correctly
        let res = client
            .auth()
            .login_access_token(&AccessTokenLoginRequest {
                access_token: "0.ec2c1d46-6a4b-4751-a310-af9601317f2d.C2IgxjjLF7qSshsbwe8JGcbM075YXw:X8vbvA0bduihIDe/qrzIQQ==".into(),
            })
            .await
            .unwrap();
        assert!(res.authenticated);
        let organization_id = client.get_access_token_organization().unwrap();
        assert_eq!(
            organization_id.to_string(),
            "f4e44a7f-1190-432a-9d4a-af96013127cb"
        );

        // Test that we can retrieve the list of secrets correctly
        let mut res = client
            .secrets()
            .list(&SecretIdentifiersRequest { organization_id })
            .await
            .unwrap();
        assert_eq!(res.data.len(), 1);

        // Test that given a secret ID we can get it's data
        let res = client
            .secrets()
            .get(&SecretGetRequest {
                id: res.data.remove(0).id,
            })
            .await
            .unwrap();
        assert_eq!(res.key, "TEST");
        assert_eq!(res.note, "TEST");
        assert_eq!(res.value, "TEST");
    }
}
