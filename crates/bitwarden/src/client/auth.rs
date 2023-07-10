use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::{
    client::{auth_settings::AuthSettings, LoginMethod},
    error::Result,
    state::state_service::ServiceDefinition,
    Client,
};

const AUTH_SERVICE: ServiceDefinition<Auth> = ServiceDefinition::new("auth");

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_expiration: Option<chrono::DateTime<Utc>>,
    pub login_method: Option<LoginMethod>,

    pub kdf: Option<AuthSettings>,
}

impl Auth {
    pub(crate) async fn get(client: &Client) -> Auth {
        client.get_state_service(AUTH_SERVICE).get().await
    }

    pub(crate) async fn set_tokens(
        client: &Client,
        token: String,
        refresh_token: Option<String>,
        expires_in: u64,
        login_method: LoginMethod,
    ) -> Result<()> {
        client
            .get_state_service(AUTH_SERVICE)
            .modify(move |auth| {
                auth.access_token = token.clone();
                auth.refresh_token = refresh_token;
                auth.token_expiration =
                    Some(Utc::now() + chrono::Duration::seconds(expires_in as i64));
                auth.login_method = Some(login_method);
                Ok(())
            })
            .await
    }

    pub(crate) async fn set_kdf(client: &Client, kdf: AuthSettings) -> Result<()> {
        client
            .get_state_service(AUTH_SERVICE)
            .modify(move |auth| {
                auth.kdf = Some(kdf);
                Ok(())
            })
            .await
    }
}
