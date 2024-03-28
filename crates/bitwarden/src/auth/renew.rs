use chrono::Utc;

#[cfg(feature = "internal")]
use crate::{auth::api::request::ApiTokenRequest, client::UserLoginMethod};
use crate::{
    auth::api::{request::AccessTokenRequest, response::IdentityTokenResponse},
    client::{Client, LoginMethod, ServiceAccountLoginMethod},
    error::{Error, Result},
    secrets_manager::state::{self, ClientState},
};

pub(crate) async fn renew_token(client: &mut Client) -> Result<()> {
    const TOKEN_RENEW_MARGIN_SECONDS: i64 = 5 * 60;

    if let (Some(expires), Some(login_method)) = (&client.token_expires_on, &client.login_method) {
        if Utc::now().timestamp() < expires - TOKEN_RENEW_MARGIN_SECONDS {
            return Ok(());
        }

        let res = match login_method {
            #[cfg(feature = "internal")]
            LoginMethod::User(u) => match u {
                UserLoginMethod::Username { client_id, .. } => {
                    let refresh = client
                        .refresh_token
                        .as_deref()
                        .ok_or(Error::NotAuthenticated)?;

                    crate::auth::api::request::RenewTokenRequest::new(
                        refresh.to_owned(),
                        client_id.to_owned(),
                    )
                    .send(&client.__api_configurations)
                    .await?
                }
                UserLoginMethod::ApiKey {
                    client_id,
                    client_secret,
                    ..
                } => {
                    ApiTokenRequest::new(client_id, client_secret)
                        .send(&client.__api_configurations)
                        .await?
                }
            },
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
                    .send(&client.__api_configurations)
                    .await?;

                    if let (
                        IdentityTokenResponse::Authenticated(r),
                        Some(state_file),
                        Ok(enc_settings),
                    ) = (&result, state_file, client.get_encryption_settings())
                    {
                        if let Some(enc_key) = enc_settings.get_key(&None) {
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
            _ => {
                // We should never get here
                return Err(Error::InvalidResponse);
            }
        }
    }

    Err(Error::NotAuthenticated)
}
