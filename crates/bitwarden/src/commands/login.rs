use std::str::FromStr;

use base64::Engine;
use bitwarden_api_identity::{
    apis::accounts_api::accounts_prelogin_post,
    models::{PreloginRequestModel, PreloginResponseModel},
};
use chrono::Utc;
use log::{debug, info};

use crate::{
    api::{
        request::{AccessTokenRequest, ApiTokenRequest, PasswordTokenRequest},
        response::IdentityTokenResponse,
    },
    client::{
        access_token::AccessToken,
        auth_settings::AuthSettings,
        encryption_settings::{decrypt, SymmetricCryptoKey},
        Client, LoginMethod,
    },
    crypto::CipherString,
    error::{Error, Result},
    sdk::{
        auth::{
            request::{
                AccessTokenLoginRequest, ApiKeyLoginRequest, PasswordLoginRequest,
                SessionLoginRequest,
            },
            response::{ApiKeyLoginResponse, PasswordLoginResponse},
        },
        model::state_service::{AUTH_SERVICE, KEYS_SERVICE, PROFILE_SERVICE},
    },
    util::{decode_token, BASE64_ENGINE},
};

#[allow(dead_code)]
pub(crate) async fn password_login(
    client: &mut Client,
    input: &PasswordLoginRequest,
) -> Result<PasswordLoginResponse> {
    info!("password logging in");
    debug!("{:#?}, {:#?}", client, input);

    let password_hash = determine_password_hash(client, &input.email, &input.password).await?;
    let response = request_identity_tokens(client, input, &password_hash).await?;

    if let IdentityTokenResponse::Authenticated(r) = &response {
        client
            .set_tokens(
                r.access_token.clone(),
                r.refresh_token.clone(),
                r.expires_in,
                LoginMethod::Username {
                    client_id: "web".to_owned(),
                },
            )
            .await?;

        let user_key = CipherString::from_str(r.key.as_deref().unwrap()).unwrap();
        let private_key = CipherString::from_str(r.private_key.as_deref().unwrap()).unwrap();

        client
            .initialize_user_crypto(&input.password, user_key, private_key)
            .await?;
    }

    PasswordLoginResponse::process_response(response)
}

#[allow(dead_code)]
pub(crate) async fn api_key_login(
    client: &mut Client,
    input: &ApiKeyLoginRequest,
) -> Result<ApiKeyLoginResponse> {
    //info!("api key logging in");
    //debug!("{:#?}, {:#?}", client, input);

    let response = request_api_identity_tokens(client, input).await?;

    if let IdentityTokenResponse::Authenticated(r) = &response {
        client
            .set_tokens(
                r.access_token.clone(),
                r.refresh_token.clone(),
                r.expires_in,
                LoginMethod::ApiKey {
                    client_id: input.client_id.to_owned(),
                    client_secret: input.client_secret.to_owned(),
                },
            )
            .await?;

        let access_token_obj = decode_token(&r.access_token)?;

        // This should always be Some() when logging in with an api key
        let email = access_token_obj
            .email
            .ok_or(Error::Internal("Access token doesn't contain email"))?;

        let _ = determine_password_hash(client, &email, &input.password).await?;

        let user_key = CipherString::from_str(r.key.as_deref().unwrap()).unwrap();
        let private_key = CipherString::from_str(r.private_key.as_deref().unwrap()).unwrap();

        client
            .initialize_user_crypto(&input.password, user_key, private_key)
            .await?;
    }

    ApiKeyLoginResponse::process_response(response)
}

pub(crate) async fn access_token_login(
    client: &mut Client,
    input: &AccessTokenLoginRequest,
) -> Result<ApiKeyLoginResponse> {
    //info!("api key logging in");
    //debug!("{:#?}, {:#?}", client, input);

    let access_token = AccessToken::from_str(&input.access_token)?;

    let response = request_access_token(client, &access_token).await?;

    if let IdentityTokenResponse::Payload(r) = &response {
        // Extract the encrypted payload and use the access token encryption key to decrypt it
        let payload = CipherString::from_str(&r.encrypted_payload)?;

        let decrypted_payload = decrypt(&payload, &access_token.encryption_key)?;

        // Once decrypted, we have to JSON decode to extract the organization encryption key
        #[derive(serde::Deserialize)]
        struct Payload {
            #[serde(rename = "encryptionKey")]
            encryption_key: String,
        }

        let payload: Payload = serde_json::from_slice(&decrypted_payload)?;

        let encryption_key = BASE64_ENGINE.decode(&payload.encryption_key)?;

        let encryption_key = SymmetricCryptoKey::try_from(encryption_key.as_slice())?;

        let access_token_obj = decode_token(&r.access_token)?;

        // This should always be Some() when logging in with an access token
        let organization_id = access_token_obj
            .organization
            .ok_or(Error::MissingFields)?
            .parse()
            .map_err(|_| Error::InvalidResponse)?;

        client
            .set_tokens(
                r.access_token.clone(),
                r.refresh_token.clone(),
                r.expires_in,
                LoginMethod::AccessToken {
                    service_account_id: access_token.service_account_id,
                    client_secret: access_token.client_secret,
                    organization_id,
                },
            )
            .await?;

        client.initialize_crypto_single_key(encryption_key);
    }

    ApiKeyLoginResponse::process_response(response)
}

pub(crate) async fn session_login(client: &mut Client, input: &SessionLoginRequest) -> Result<()> {
    client.state.load_account(input.user_id).await?;

    let Some(profile) = client.get_state_service(PROFILE_SERVICE).get().await else {return Err(Error::NotAuthenticated)};

    let auth = client.get_state_service(AUTH_SERVICE).get().await;

    let Some(expires) = auth.token_expiration else {return Err(Error::VaultLocked)};
    let Some(login_method) = auth.login_method else {return Err(Error::VaultLocked)};
    let expires_seconds = (expires.timestamp() - Utc::now().timestamp()) as u64;

    client
        .set_tokens(
            auth.access_token,
            auth.refresh_token,
            expires_seconds,
            login_method,
        )
        .await?;

    let _ = determine_password_hash(client, &profile.email, &input.password).await?;

    let Some(keys) = client.get_state_service(KEYS_SERVICE).get().await else {return Err(Error::VaultLocked)};

    client
        .initialize_user_crypto(&input.password, keys.crypto_symmetric_key, keys.private_key)
        .await?;

    client.initialize_org_crypto().await?;

    Ok(())
}

async fn determine_password_hash(
    client: &mut Client,
    email: &str,
    password: &str,
) -> Result<String> {
    let pre_login = request_prelogin(client, email.to_owned()).await?;
    let auth_settings = AuthSettings::new(pre_login, email.to_owned());
    let password_hash = auth_settings.make_user_password_hash(password);
    client.set_auth_settings(auth_settings).await?;

    Ok(password_hash)
}

async fn request_prelogin(client: &mut Client, email: String) -> Result<PreloginResponseModel> {
    let request_model = PreloginRequestModel::new(email);
    let config = client.get_api_configurations().await;
    Ok(accounts_prelogin_post(&config.identity, Some(request_model)).await?)
}

async fn request_identity_tokens(
    client: &mut Client,
    input: &PasswordLoginRequest,
    password_hash: &String,
) -> Result<IdentityTokenResponse> {
    let config = client.get_api_configurations().await;
    PasswordTokenRequest::new(&input.email, password_hash)
        .send(&config)
        .await
}

#[allow(dead_code)]
async fn request_api_identity_tokens(
    client: &mut Client,
    input: &ApiKeyLoginRequest,
) -> Result<IdentityTokenResponse> {
    let config = client.get_api_configurations().await;
    ApiTokenRequest::new(&input.client_id, &input.client_secret)
        .send(&config)
        .await
}

async fn request_access_token(
    client: &mut Client,
    input: &AccessToken,
) -> Result<IdentityTokenResponse> {
    let config = client.get_api_configurations().await;
    AccessTokenRequest::new(input.service_account_id, &input.client_secret)
        .send(&config)
        .await
}

pub(crate) async fn renew_token(client: &mut Client) -> Result<()> {
    let token_renew_margin = chrono::Duration::seconds(5 * 60);

    let auth = client.get_state_service(AUTH_SERVICE).get().await;

    if let (Some(expires), Some(login_method)) = (&auth.token_expiration, &auth.login_method) {
        if expires > &(Utc::now() + token_renew_margin) {
            return Ok(());
        }

        let res = match login_method {
            LoginMethod::Username { client_id } => {
                let refresh = auth
                    .refresh_token
                    .as_deref()
                    .ok_or(Error::NotAuthenticated)?;

                crate::api::request::RenewTokenRequest::new(
                    refresh.to_owned(),
                    client_id.to_owned(),
                )
                .send(&client.__api_configurations)
                .await?
            }
            LoginMethod::ApiKey {
                client_id,
                client_secret,
            } => {
                ApiTokenRequest::new(client_id, client_secret)
                    .send(&client.__api_configurations)
                    .await?
            }
            LoginMethod::AccessToken {
                service_account_id,
                client_secret,
                ..
            } => {
                AccessTokenRequest::new(*service_account_id, &client_secret)
                    .send(&client.__api_configurations)
                    .await?
            }
        };

        match res {
            IdentityTokenResponse::Refreshed(r) => {
                let login_method = login_method.to_owned();
                client
                    .set_tokens(r.access_token, r.refresh_token, r.expires_in, login_method)
                    .await?;
                return Ok(());
            }
            IdentityTokenResponse::Authenticated(r) => {
                let login_method = login_method.to_owned();
                client
                    .set_tokens(r.access_token, r.refresh_token, r.expires_in, login_method)
                    .await?;
                return Ok(());
            }
            _ => {
                // We should never get here
                return Err(Error::InvalidResponse);
            }
        }
    }

    Err(Error::NotAuthenticated)
}
