use super::{
    password::{password_strength, satisfies_policy, MasterPasswordPolicyOptions},
    register::{make_register_keys, register},
    renew::renew_token,
    RegisterKeyResponse, RegisterRequest,
};
#[cfg(feature = "secrets")]
use crate::auth::login::{access_token_login, AccessTokenLoginRequest, AccessTokenLoginResponse};
#[cfg(feature = "internal")]
use crate::auth::login::{
    api_key_login, password_login, send_two_factor_email, ApiKeyLoginRequest, ApiKeyLoginResponse,
    PasswordLoginRequest, PasswordLoginResponse, TwoFactorEmailRequest,
};
use crate::{client::kdf::Kdf, error::Result, Client};

pub struct ClientAuth<'a> {
    pub(crate) client: &'a mut crate::Client,
}

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
    pub async fn password_login(
        &mut self,
        input: &PasswordLoginRequest,
    ) -> Result<PasswordLoginResponse> {
        password_login(self.client, input).await
    }

    #[cfg(feature = "internal")]
    pub async fn api_key_login(
        &mut self,
        input: &ApiKeyLoginRequest,
    ) -> Result<ApiKeyLoginResponse> {
        api_key_login(self.client, input).await
    }

    #[cfg(feature = "secrets")]
    pub async fn access_token_login(
        &mut self,
        input: &AccessTokenLoginRequest,
    ) -> Result<AccessTokenLoginResponse> {
        access_token_login(self.client, input).await
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
