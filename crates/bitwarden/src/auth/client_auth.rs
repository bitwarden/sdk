#[cfg(feature = "secrets")]
use crate::auth::login::{login_access_token, AccessTokenLoginRequest, AccessTokenLoginResponse};
use crate::{auth::renew::renew_token, error::Result, Client};
#[cfg(feature = "internal")]
use crate::{
    auth::{
        login::{
            login_api_key, login_password, send_two_factor_email, ApiKeyLoginRequest,
            ApiKeyLoginResponse, PasswordLoginRequest, PasswordLoginResponse,
            TwoFactorEmailRequest,
        },
        password::{password_strength, satisfies_policy, MasterPasswordPolicyOptions},
        register::{make_register_keys, register},
        RegisterKeyResponse, RegisterRequest,
    },
    client::kdf::Kdf,
};

pub struct ClientAuth<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientAuth<'a> {
    #[cfg(feature = "internal")]
    pub async fn password_strength(
        &self,
        password: String,
        email: String,
        additional_inputs: Vec<String>,
    ) -> u8 {
        password_strength(password, email, additional_inputs)
    }

    #[cfg(feature = "internal")]
    pub async fn satisfies_policy(
        &self,
        password: String,
        strength: u8,
        policy: &MasterPasswordPolicyOptions,
    ) -> bool {
        satisfies_policy(password, strength, policy)
    }

    #[cfg(feature = "internal")]
    pub fn make_register_keys(
        &self,
        email: String,
        password: String,
        kdf: Kdf,
    ) -> Result<RegisterKeyResponse> {
        make_register_keys(email, password, kdf)
    }

    pub async fn renew_token(&mut self) -> Result<()> {
        renew_token(self.client).await
    }

    #[cfg(feature = "internal")]
    pub async fn register(&mut self, input: &RegisterRequest) -> Result<()> {
        register(self.client, input).await
    }

    #[cfg(feature = "internal")]
    pub async fn prelogin(&mut self, email: String) -> Result<Kdf> {
        use crate::auth::login::request_prelogin;

        request_prelogin(self.client, email).await?.try_into()
    }

    #[cfg(feature = "internal")]
    pub async fn login_password(
        &mut self,
        input: &PasswordLoginRequest,
    ) -> Result<PasswordLoginResponse> {
        login_password(self.client, input).await
    }

    #[cfg(feature = "internal")]
    pub async fn login_api_key(
        &mut self,
        input: &ApiKeyLoginRequest,
    ) -> Result<ApiKeyLoginResponse> {
        login_api_key(self.client, input).await
    }

    #[cfg(feature = "secrets")]
    pub async fn login_access_token(
        &mut self,
        input: &AccessTokenLoginRequest,
    ) -> Result<AccessTokenLoginResponse> {
        login_access_token(self.client, input).await
    }

    #[cfg(feature = "internal")]
    pub async fn send_two_factor_email(&mut self, tf: &TwoFactorEmailRequest) -> Result<()> {
        send_two_factor_email(self.client, tf).await
    }
}

impl<'a> Client {
    pub fn auth(&'a mut self) -> ClientAuth<'a> {
        ClientAuth { client: self }
    }
}
