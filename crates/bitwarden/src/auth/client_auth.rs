#[cfg(feature = "internal")]
use bitwarden_crypto::{AsymmetricEncString, DeviceKey, TrustDeviceResponse};

#[cfg(feature = "mobile")]
use crate::auth::login::NewAuthRequestResponse;
#[cfg(feature = "secrets")]
use crate::auth::login::{login_access_token, AccessTokenLoginRequest, AccessTokenLoginResponse};
use crate::{auth::renew::renew_token, error::Result, Client};
#[cfg(feature = "internal")]
use crate::{
    auth::{
        auth_request::{approve_auth_request, new_auth_request},
        login::{
            login_api_key, login_password, send_two_factor_email, ApiKeyLoginRequest,
            ApiKeyLoginResponse, PasswordLoginRequest, PasswordLoginResponse,
            TwoFactorEmailRequest,
        },
        password::{
            password_strength, satisfies_policy, validate_password, validate_password_user_key,
            MasterPasswordPolicyOptions,
        },
        register::{make_register_keys, register},
        AuthRequestResponse, RegisterKeyResponse, RegisterRequest,
    },
    client::Kdf,
    error::Error,
};

pub struct ClientAuth<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientAuth<'a> {
    pub async fn renew_token(&mut self) -> Result<()> {
        renew_token(self.client).await
    }

    #[cfg(feature = "secrets")]
    pub async fn login_access_token(
        &mut self,
        input: &AccessTokenLoginRequest,
    ) -> Result<AccessTokenLoginResponse> {
        login_access_token(self.client, input).await
    }
}

#[cfg(feature = "internal")]
impl<'a> ClientAuth<'a> {
    pub async fn password_strength(
        &self,
        password: String,
        email: String,
        additional_inputs: Vec<String>,
    ) -> u8 {
        password_strength(password, email, additional_inputs)
    }

    pub async fn satisfies_policy(
        &self,
        password: String,
        strength: u8,
        policy: &MasterPasswordPolicyOptions,
    ) -> bool {
        satisfies_policy(password, strength, policy)
    }

    pub fn make_register_keys(
        &self,
        email: String,
        password: String,
        kdf: Kdf,
    ) -> Result<RegisterKeyResponse> {
        make_register_keys(email, password, kdf)
    }

    pub async fn register(&mut self, input: &RegisterRequest) -> Result<()> {
        register(self.client, input).await
    }

    pub async fn prelogin(&mut self, email: String) -> Result<Kdf> {
        use crate::auth::login::{parse_prelogin, request_prelogin};

        let response = request_prelogin(self.client, email).await?;
        parse_prelogin(response)
    }

    pub async fn login_password(
        &mut self,
        input: &PasswordLoginRequest,
    ) -> Result<PasswordLoginResponse> {
        login_password(self.client, input).await
    }

    pub async fn login_api_key(
        &mut self,
        input: &ApiKeyLoginRequest,
    ) -> Result<ApiKeyLoginResponse> {
        login_api_key(self.client, input).await
    }

    pub async fn send_two_factor_email(&mut self, tf: &TwoFactorEmailRequest) -> Result<()> {
        send_two_factor_email(self.client, tf).await
    }

    pub fn validate_password(&self, password: String, password_hash: String) -> Result<bool> {
        validate_password(self.client, password, password_hash)
    }

    pub fn validate_password_user_key(
        &self,
        password: String,
        encrypted_user_key: String,
    ) -> Result<String> {
        validate_password_user_key(self.client, password, encrypted_user_key)
    }

    pub fn new_auth_request(&self, email: &str) -> Result<AuthRequestResponse> {
        new_auth_request(email)
    }

    pub fn approve_auth_request(&mut self, public_key: String) -> Result<AsymmetricEncString> {
        approve_auth_request(self.client, public_key)
    }

    pub async fn trust_device(&self) -> Result<TrustDeviceResponse> {
        trust_device(self.client)
    }
}

#[cfg(feature = "mobile")]
impl<'a> ClientAuth<'a> {
    pub async fn login_device(
        &mut self,
        email: String,
        device_identifier: String,
    ) -> Result<NewAuthRequestResponse> {
        use crate::auth::login::send_new_auth_request;

        send_new_auth_request(self.client, email, device_identifier).await
    }

    pub async fn login_device_complete(&mut self, auth_req: NewAuthRequestResponse) -> Result<()> {
        use crate::auth::login::complete_auth_request;

        complete_auth_request(self.client, auth_req).await
    }
}

#[cfg(feature = "internal")]
fn trust_device(client: &Client) -> Result<TrustDeviceResponse> {
    let enc = client.get_encryption_settings()?;

    let user_key = enc.get_key(&None).ok_or(Error::VaultLocked)?;

    Ok(DeviceKey::trust_device(user_key)?)
}

impl<'a> Client {
    pub fn auth(&'a mut self) -> ClientAuth<'a> {
        ClientAuth { client: self }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "secrets")]
    #[tokio::test]
    async fn test_access_token_login() {
        use wiremock::{matchers, Mock, ResponseTemplate};

        use crate::{auth::login::AccessTokenLoginRequest, secrets_manager::secrets::*};

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
                state_file: None,
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
