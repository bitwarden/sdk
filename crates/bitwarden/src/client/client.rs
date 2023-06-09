use std::time::{Duration, Instant};

use log::debug;
use reqwest::header::{self};
use uuid::Uuid;

#[allow(unused_imports)]
use crate::{
    client::{
        auth_settings::AuthSettings,
        client_projects::ClientProjects,
        client_secrets::ClientSecrets,
        encryption_settings::{EncryptionSettings, SymmetricCryptoKey},
    },
    commands::{
        access_token_login, api_key_login, generate_fingerprint, get_user_api_key, password_login,
        renew_token, sync,
    },
    crypto::CipherString,
    error::{Error, Result},
    sdk::{
        auth::{
            request::{AccessTokenLoginRequest, ApiKeyLoginRequest, PasswordLoginRequest},
            response::{ApiKeyLoginResponse, PasswordLoginResponse},
        },
        request::{
            client_settings::{ClientSettings, DeviceType},
            fingerprint_request::FingerprintRequest,
            secret_verification_request::SecretVerificationRequest,
            sync_request::SyncRequest,
        },
        response::{sync_response::SyncResponse, user_api_key_response::UserApiKeyResponse},
    },
};
use crate::{commands::send_two_factor_email, sdk::auth::request::TwoFactorEmailRequest};

#[derive(Debug)]
pub(crate) struct ApiConfigurations {
    pub identity: bitwarden_api_identity::apis::configuration::Configuration,
    pub api: bitwarden_api_api::apis::configuration::Configuration,
    pub device_type: DeviceType,
}

#[derive(Debug, Clone)]
pub(crate) enum LoginMethod {
    Username {
        client_id: String,
    },
    ApiKey {
        client_id: String,
        client_secret: String,
    },
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

    auth_settings: Option<AuthSettings>,

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
            auth_settings: None,
            encryption_settings: None,
        }
    }

    pub(crate) async fn get_api_configurations(&mut self) -> &ApiConfigurations {
        // At the moment we ignore the error result from the token renewal, if it fails,
        // the token will end up expiring and the next operation is going to fail anyway.
        self.renew_token().await.ok();
        &self.__api_configurations
    }

    #[cfg(feature = "internal")]
    pub async fn password_login(
        &mut self,
        input: &PasswordLoginRequest,
    ) -> Result<PasswordLoginResponse> {
        password_login(self, input).await
    }

    #[cfg(feature = "internal")]
    pub async fn api_key_login(
        &mut self,
        input: &ApiKeyLoginRequest,
    ) -> Result<ApiKeyLoginResponse> {
        api_key_login(self, input).await
    }

    pub async fn access_token_login(
        &mut self,
        input: &AccessTokenLoginRequest,
    ) -> Result<ApiKeyLoginResponse> {
        access_token_login(self, input).await
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

    pub(crate) fn get_auth_settings(&self) -> &Option<AuthSettings> {
        &self.auth_settings
    }

    pub fn get_access_token_organization(&self) -> Option<Uuid> {
        match &self.login_method {
            Some(LoginMethod::AccessToken {
                organization_id, ..
            }) => Some(*organization_id),
            _ => None,
        }
    }

    pub(crate) fn get_encryption_settings(&self) -> &Option<EncryptionSettings> {
        &self.encryption_settings
    }

    pub(crate) fn set_auth_settings(&mut self, auth_settings: AuthSettings) {
        debug! {"setting auth settings: {:#?}", auth_settings}
        self.auth_settings = Some(auth_settings);
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

    pub async fn renew_token(&mut self) -> Result<()> {
        renew_token(self).await
    }

    pub fn is_authed(&self) -> bool {
        self.token.is_some() || self.auth_settings.is_some()
    }

    pub(crate) fn initialize_user_crypto(
        &mut self,
        password: &str,
        user_key: CipherString,
        private_key: CipherString,
    ) -> Result<&EncryptionSettings> {
        let auth = match &self.auth_settings {
            Some(a) => a,
            None => return Err(Error::NotAuthenticated),
        };

        self.encryption_settings = Some(EncryptionSettings::new(
            auth,
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

    pub(crate) fn initialize_org_crypto(
        &mut self,
        org_keys: Vec<(Uuid, CipherString)>,
    ) -> Result<&EncryptionSettings> {
        let enc = self
            .encryption_settings
            .as_mut()
            .ok_or(Error::VaultLocked)?;

        enc.set_org_keys(org_keys)?;
        Ok(self.encryption_settings.as_ref().unwrap())
    }

    #[cfg(feature = "internal")]
    pub fn fingerprint(&mut self, input: &FingerprintRequest) -> Result<String> {
        generate_fingerprint(input)
    }

    pub async fn send_two_factor_email(&mut self, tf: &TwoFactorEmailRequest) -> Result<()> {
        send_two_factor_email(self, tf).await
    }
}

impl<'a> Client {
    pub fn secrets(&'a mut self) -> ClientSecrets<'a> {
        ClientSecrets { client: self }
    }

    pub fn projects(&'a mut self) -> ClientProjects<'a> {
        ClientProjects { client: self }
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{matchers, Mock, ResponseTemplate};

    use crate::sdk::{auth::request::AccessTokenLoginRequest, request::secrets_request::*};

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
            .access_token_login(&AccessTokenLoginRequest {
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
