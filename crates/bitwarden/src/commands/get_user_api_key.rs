use bitwarden_api_api::{
    apis::accounts_api::accounts_api_key_post, models::SecretVerificationRequestModel,
};
use log::{debug, info};

use crate::{
    client::auth_settings::AuthSettings,
    error::{Error, Result},
    sdk::{
        request::secret_verification_request::SecretVerificationRequest,
        response::user_api_key_response::UserApiKeyResponse,
    },
    Client,
};

pub(crate) async fn get_user_api_key(
    client: &mut Client,
    input: &SecretVerificationRequest,
) -> Result<UserApiKeyResponse> {
    info!("Getting Api Key");
    debug!("{:?}", input);

    let auth_settings = get_auth_settings(client)?;
    let request = get_secret_verification_request(auth_settings, input);

    let config = client.get_api_configurations().await;

    let response = accounts_api_key_post(&config.api, Some(request)).await?;
    UserApiKeyResponse::process_response(response)
}

fn get_auth_settings(client: &Client) -> Result<&AuthSettings> {
    if client.is_authed() {
        let auth_settings = client
            .get_auth_settings()
            .as_ref()
            .ok_or(Error::NotAuthenticated)?;
        Ok(auth_settings)
    } else {
        Err(Error::NotAuthenticated)
    }
}

fn get_secret_verification_request(
    auth_settings: &AuthSettings,
    input: &SecretVerificationRequest,
) -> SecretVerificationRequestModel {
    let master_password_hash = input
        .master_password
        .as_ref()
        .map(|p| auth_settings.make_user_password_hash(p));
    SecretVerificationRequestModel {
        master_password_hash,
        otp: input.otp.as_ref().cloned(),
        secret: None,
        auth_request_access_code: None,
    }
}
