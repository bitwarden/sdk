use chrono::Utc;

#[cfg(feature = "secrets")]
use crate::{
    auth::api::request::AccessTokenRequest,
    client::ServiceAccountLoginMethod,
    key_management::SymmetricKeyRef,
    secrets_manager::state::{self, ClientState},
};
use crate::{
    auth::api::{request::ApiTokenRequest, response::IdentityTokenResponse},
    client::{internal::InternalClient, LoginMethod, UserLoginMethod},
    error::{Error, Result},
};

pub(crate) async fn renew_token(client: &InternalClient) -> Result<()> {
    const TOKEN_RENEW_MARGIN_SECONDS: i64 = 5 * 60;

    let tokens = client
        .tokens
        .read()
        .expect("RwLock is not poisoned")
        .clone();
    let login_method = client
        .login_method
        .read()
        .expect("RwLock is not poisoned")
        .clone();

    if let (Some(expires), Some(login_method)) = (tokens.expires_on, login_method) {
        if Utc::now().timestamp() < expires - TOKEN_RENEW_MARGIN_SECONDS {
            return Ok(());
        }

        let config = client
            .__api_configurations
            .read()
            .expect("RwLock is not poisoned")
            .clone();

        let res = match login_method.as_ref() {
            LoginMethod::User(u) => match u {
                UserLoginMethod::Username { client_id, .. } => {
                    let refresh = tokens.refresh_token.ok_or(Error::NotAuthenticated)?;

                    crate::auth::api::request::RenewTokenRequest::new(refresh, client_id.to_owned())
                        .send(&config)
                        .await?
                }
                UserLoginMethod::ApiKey {
                    client_id,
                    client_secret,
                    ..
                } => {
                    ApiTokenRequest::new(client_id, client_secret)
                        .send(&config)
                        .await?
                }
            },
            #[cfg(feature = "secrets")]
            LoginMethod::ServiceAccount(s) => match s {
                ServiceAccountLoginMethod::AccessToken {
                    access_token,
                    state_file,
                    ..
                } => {
                    let result = AccessTokenRequest::new(
                        access_token.access_token_id,
                        &access_token.client_secret,
                    )
                    .send(&config)
                    .await?;

                    if let (IdentityTokenResponse::Payload(r), Some(state_file)) =
                        (&result, state_file)
                    {
                        let ctx = client.get_crypto_service().context();

                        #[allow(deprecated)]
                        if let Ok(enc_key) = ctx.dangerous_get_symmetric_key(SymmetricKeyRef::User)
                        {
                            let state =
                                ClientState::new(r.access_token.clone(), enc_key.to_base64());
                            _ = state::set(state_file, access_token, state);
                        }
                    }

                    result
                }
            },
        };

        match res {
            IdentityTokenResponse::Refreshed(r) => {
                client.set_tokens(r.access_token, r.refresh_token, r.expires_in);
                return Ok(());
            }
            IdentityTokenResponse::Authenticated(r) => {
                client.set_tokens(r.access_token, r.refresh_token, r.expires_in);
                return Ok(());
            }
            IdentityTokenResponse::Payload(r) => {
                client.set_tokens(r.access_token, r.refresh_token, r.expires_in);
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
