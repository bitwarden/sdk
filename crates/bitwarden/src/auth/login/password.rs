use log::{debug, info};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{
    auth::{
        api::{request::PasswordTokenRequest, response::IdentityTokenResponse},
        login::determine_password_hash,
        response::PasswordLoginResponse,
    },
    client::LoginMethod,
    crypto::CipherString,
    error::Result,
    Client,
};

pub(crate) async fn password_login(
    client: &mut Client,
    input: &PasswordLoginRequest,
) -> Result<PasswordLoginResponse> {
    info!("password logging in");
    debug!("{:#?}, {:#?}", client, input);

    let password_hash = determine_password_hash(client, &input.email, &input.password).await?;
    let response = request_identity_tokens(client, input, &password_hash).await?;

    if let IdentityTokenResponse::Authenticated(r) = &response {
        client.set_tokens(
            r.access_token.clone(),
            r.refresh_token.clone(),
            r.expires_in,
            LoginMethod::Username {
                client_id: "web".to_owned(),
            },
        );

        let user_key = CipherString::from_str(r.key.as_deref().unwrap()).unwrap();
        let private_key = CipherString::from_str(r.private_key.as_deref().unwrap()).unwrap();

        client.initialize_user_crypto(&input.password, user_key, private_key)?;
    }

    PasswordLoginResponse::process_response(response)
}

async fn request_identity_tokens(
    client: &mut Client,
    input: &PasswordLoginRequest,
    password_hash: &String,
) -> Result<IdentityTokenResponse> {
    let config = client.get_api_configurations().await;
    PasswordTokenRequest::new(&input.email, password_hash)
        .send(config)
        .await
}

/// Login to Bitwarden with Username and Password
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PasswordLoginRequest {
    /// Bitwarden account email address
    pub email: String,
    /// Bitwarden account master password
    pub password: String,
}
