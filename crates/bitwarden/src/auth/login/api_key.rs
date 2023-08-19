use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{
    auth::{
        api::{request::ApiTokenRequest, response::IdentityTokenResponse},
        login::determine_password_hash,
        response::ApiKeyLoginResponse,
    },
    client::LoginMethod,
    crypto::CipherString,
    error::{Error, Result},
    util::decode_token,
    Client,
};

pub(crate) async fn api_key_login(
    client: &mut Client,
    input: &ApiKeyLoginRequest,
) -> Result<ApiKeyLoginResponse> {
    //info!("api key logging in");
    //debug!("{:#?}, {:#?}", client, input);

    let response = request_api_identity_tokens(client, input).await?;

    if let IdentityTokenResponse::Authenticated(r) = &response {
        client.set_tokens(
            r.access_token.clone(),
            r.refresh_token.clone(),
            r.expires_in,
            LoginMethod::ApiKey {
                client_id: input.client_id.to_owned(),
                client_secret: input.client_secret.to_owned(),
            },
        );

        let access_token_obj = decode_token(&r.access_token)?;

        // This should always be Some() when logging in with an api key
        let email = access_token_obj
            .email
            .ok_or(Error::Internal("Access token doesn't contain email"))?;

        let _ = determine_password_hash(client, &email, &input.password).await?;

        let user_key = CipherString::from_str(r.key.as_deref().unwrap()).unwrap();
        let private_key = CipherString::from_str(r.private_key.as_deref().unwrap()).unwrap();

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
