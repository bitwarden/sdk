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

    pub fn trust_device(&self) -> Result<TrustDeviceResponse> {
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
