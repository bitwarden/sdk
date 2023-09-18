use std::time::{Duration, Instant};

#[cfg(feature = "internal")]
use crate::auth::api::request::ApiTokenRequest;
use crate::{
    auth::api::{request::AccessTokenRequest, response::IdentityTokenResponse},
    client::{Client, LoginMethod, ServiceAccountLoginMethod, UserLoginMethod},
    error::{Error, Result},
};

pub(crate) async fn renew_token(client: &mut Client) -> Result<()> {
    const TOKEN_RENEW_MARGIN: Duration = Duration::from_secs(5 * 60);

    if let (Some(expires), Some(login_method)) = (&client.token_expires_in, &client.login_method) {
        if expires > &(Instant::now() + TOKEN_RENEW_MARGIN) {
            return Ok(());
        }

        let res = match login_method {
            #[cfg(feature = "internal")]
            LoginMethod::User(u) => match u {
                UserLoginMethod::Username { client_id } => {
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
                } => {
                    ApiTokenRequest::new(client_id, client_secret)
                        .send(&client.__api_configurations)
                        .await?
                }
            },
            LoginMethod::ServiceAccount(s) => match s {
                ServiceAccountLoginMethod::AccessToken {
                    service_account_id,
                    client_secret,
                    ..
                } => {
                    AccessTokenRequest::new(*service_account_id, client_secret)
                        .send(&client.__api_configurations)
                        .await?
                }
            },
        };

        match res {
            IdentityTokenResponse::Refreshed(r) => {
                let login_method = login_method.to_owned();
                client.set_tokens(r.access_token, r.refresh_token, r.expires_in, login_method);
                return Ok(());
            }
            IdentityTokenResponse::Authenticated(r) => {
                let login_method = login_method.to_owned();
                client.set_tokens(r.access_token, r.refresh_token, r.expires_in, login_method);
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
