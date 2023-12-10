use std::str::FromStr;

use base64::Engine;
use chrono::Utc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{
        api::{request::AccessTokenRequest, response::IdentityTokenResponse},
        login::{response::two_factor::TwoFactorProviders, PasswordLoginResponse},
        JWTToken,
    },
    client::{AccessToken, ClientState, LoginMethod, ServiceAccountLoginMethod},
    crypto::{EncString, KeyDecryptable, SymmetricCryptoKey},
    error::{Error, Result},
    util::BASE64_ENGINE,
    Client,
};

pub(crate) async fn login_access_token_from_state(
    client: &mut Client,
    state: ClientState,
) -> Result<()> {
    let time_till_expiration = state.token_expiry_timestamp - Utc::now().timestamp();

    if time_till_expiration < 0 {
        return Err(Error::Internal("Expired token error."));
    }

    client.set_tokens(
        state.token,
        state.refresh_token,
        time_till_expiration as u64,
        LoginMethod::ServiceAccount(ServiceAccountLoginMethod::AccessToken {
            service_account_id: state.access_token.service_account_id,
            client_secret: state.access_token.client_secret,
            organization_id: state.access_token.organization_id,
        }),
    );

    let encryption_key = match SymmetricCryptoKey::from_str(&state.encryption_key) {
        Ok(t) => t,
        Err(_) => panic!("Failure to convert encryption key into SymmetricCryptoKey"),
    };

    client.initialize_crypto_single_key(encryption_key);

    Ok(())
}

pub(crate) async fn login_access_token(
    client: &mut Client,
    input: &AccessTokenLoginRequest,
) -> Result<AccessTokenLoginResponse> {
    //info!("api key logging in");
    //debug!("{:#?}, {:#?}", client, input);

    let access_token: AccessToken = input.access_token.parse()?;

    let response = request_access_token(client, &access_token).await?;

    if let IdentityTokenResponse::Payload(r) = &response {
        // Extract the encrypted payload and use the access token encryption key to decrypt it
        let payload: EncString = r.encrypted_payload.parse()?;

        let decrypted_payload: Vec<u8> = payload.decrypt_with_key(&access_token.encryption_key)?;

        // Once decrypted, we have to JSON decode to extract the organization encryption key
        #[derive(serde::Deserialize)]
        struct Payload {
            #[serde(rename = "encryptionKey")]
            encryption_key: String,
        }

        let payload: Payload = serde_json::from_slice(&decrypted_payload)?;
        let encryption_key = BASE64_ENGINE.decode(payload.encryption_key)?;
        let encryption_key = SymmetricCryptoKey::try_from(encryption_key.as_slice())?;

        let access_token_obj: JWTToken = r.access_token.parse()?;

        // This should always be Some() when logging in with an access token
        let organization_id = access_token_obj
            .organization
            .ok_or(Error::MissingFields)?
            .parse()
            .map_err(|_| Error::InvalidResponse)?;

        client.set_tokens(
            r.access_token.clone(),
            r.refresh_token.clone(),
            r.expires_in,
            LoginMethod::ServiceAccount(ServiceAccountLoginMethod::AccessToken {
                access_token_id: access_token.access_token_id,
                client_secret: access_token.client_secret,
                organization_id,
            }),
        );

        client.initialize_crypto_single_key(encryption_key);
    }

    AccessTokenLoginResponse::process_response(response)
}

async fn request_access_token(
    client: &mut Client,
    input: &AccessToken,
) -> Result<IdentityTokenResponse> {
    let config = client.get_api_configurations().await;
    AccessTokenRequest::new(input.access_token_id, &input.client_secret)
        .send(config)
        .await
}

/// Login to Bitwarden with access token
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccessTokenLoginRequest {
    /// Bitwarden service API access token
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccessTokenLoginResponse {
    pub authenticated: bool,
    /// TODO: What does this do?
    pub reset_master_password: bool,
    /// Whether or not the user is required to update their master password
    pub force_password_reset: bool,
    two_factor: Option<TwoFactorProviders>,
}

impl AccessTokenLoginResponse {
    pub(crate) fn process_response(
        response: IdentityTokenResponse,
    ) -> Result<AccessTokenLoginResponse> {
        let password_response = PasswordLoginResponse::process_response(response)?;

        Ok(AccessTokenLoginResponse {
            authenticated: password_response.authenticated,
            reset_master_password: password_response.reset_master_password,
            force_password_reset: password_response.force_password_reset,
            two_factor: password_response.two_factor,
        })
    }
}
