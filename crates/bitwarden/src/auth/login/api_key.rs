use bitwarden_crypto::EncString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{
        api::{request::ApiTokenRequest, response::IdentityTokenResponse},
        login::{response::two_factor::TwoFactorProviders, PasswordLoginResponse},
        JWTToken,
    },
    client::{LoginMethod, UserLoginMethod},
    error::Result,
    Client,
};

pub(crate) async fn login_api_key(
    client: &mut Client,
    input: &ApiKeyLoginRequest,
) -> Result<ApiKeyLoginResponse> {
    //info!("api key logging in");
    //debug!("{:#?}, {:#?}", client, input);

    let response = request_api_identity_tokens(client, input).await?;

    if let IdentityTokenResponse::Authenticated(r) = &response {
        let access_token_obj: JWTToken = r.access_token.parse()?;

        // This should always be Some() when logging in with an api key
        let email = access_token_obj
            .email
            .ok_or("Access token doesn't contain email")?;

        let kdf = client.auth().prelogin(email.clone()).await?;

        client.set_tokens(
            r.access_token.clone(),
            r.refresh_token.clone(),
            r.expires_in,
        );
        client.set_login_method(LoginMethod::User(UserLoginMethod::ApiKey {
            client_id: input.client_id.to_owned(),
            client_secret: input.client_secret.to_owned(),
            email,
            kdf,
        }));

        let user_key: EncString = r.key.as_deref().unwrap().parse().unwrap();
        let private_key: EncString = r.private_key.as_deref().unwrap().parse().unwrap();

        client.initialize_user_crypto(&input.password, user_key, private_key)?;
    }

    ApiKeyLoginResponse::process_response(response)
}

async fn request_api_identity_tokens(
    client: &mut Client,
    input: &ApiKeyLoginRequest,
) -> Result<IdentityTokenResponse> {
    let config = client.get_api_configurations().await;
    ApiTokenRequest::new(&input.client_id, &input.client_secret)
        .send(config)
        .await
}

/// Login to Bitwarden with Api Key
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ApiKeyLoginRequest {
    /// Bitwarden account client_id
    pub client_id: String,
    /// Bitwarden account client_secret
    pub client_secret: String,

    /// Bitwarden account master password
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ApiKeyLoginResponse {
    pub authenticated: bool,
    /// TODO: What does this do?
    pub reset_master_password: bool,
    /// Whether or not the user is required to update their master password
    pub force_password_reset: bool,
    two_factor: Option<TwoFactorProviders>,
}

impl ApiKeyLoginResponse {
    pub(crate) fn process_response(response: IdentityTokenResponse) -> Result<ApiKeyLoginResponse> {
        let password_response = PasswordLoginResponse::process_response(response)?;

        Ok(ApiKeyLoginResponse {
            authenticated: password_response.authenticated,
            reset_master_password: password_response.reset_master_password,
            force_password_reset: password_response.force_password_reset,
            two_factor: password_response.two_factor,
        })
    }
}
