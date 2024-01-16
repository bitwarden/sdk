use bitwarden_api_api::{
    apis::accounts_api::accounts_api_key_post,
    models::{ApiKeyResponseModel, SecretVerificationRequestModel},
};
use bitwarden_crypto::{HashPurpose, MasterKey};
use log::{debug, info};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::SecretVerificationRequest;
use crate::{
    client::{LoginMethod, UserLoginMethod},
    error::{Error, Result},
    Client,
};

pub(crate) async fn get_user_api_key(
    client: &mut Client,
    input: &SecretVerificationRequest,
) -> Result<UserApiKeyResponse> {
    info!("Getting Api Key");
    debug!("{:?}", input);

    let auth_settings = get_login_method(client)?;
    let request = get_secret_verification_request(auth_settings, input)?;

    let config = client.get_api_configurations().await;

    let response = accounts_api_key_post(&config.api, Some(request)).await?;
    UserApiKeyResponse::process_response(response)
}

fn get_login_method(client: &Client) -> Result<&LoginMethod> {
    if client.is_authed() {
        client
            .get_login_method()
            .as_ref()
            .ok_or(Error::NotAuthenticated)
    } else {
        Err(Error::NotAuthenticated)
    }
}

fn get_secret_verification_request(
    login_method: &LoginMethod,
    input: &SecretVerificationRequest,
) -> Result<SecretVerificationRequestModel> {
    if let LoginMethod::User(UserLoginMethod::Username { email, kdf, .. }) = login_method {
        let master_password_hash = input
            .master_password
            .as_ref()
            .map(|p| {
                let master_key = MasterKey::derive(p.as_bytes(), email.as_bytes(), kdf)?;

                master_key.derive_master_key_hash(p.as_bytes(), HashPurpose::ServerAuthorization)
            })
            .transpose()?;
        Ok(SecretVerificationRequestModel {
            master_password_hash,
            otp: input.otp.as_ref().cloned(),
            secret: None,
            auth_request_access_code: None,
        })
    } else {
        Err("Unsupported login method".into())
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserApiKeyResponse {
    /// The user's API key, which represents the client_secret portion of an oauth request.
    api_key: String,
}

impl UserApiKeyResponse {
    pub(crate) fn process_response(response: ApiKeyResponseModel) -> Result<UserApiKeyResponse> {
        match response.api_key {
            Some(api_key) => Ok(UserApiKeyResponse { api_key }),
            None => Err(Error::MissingFields),
        }
    }
}
