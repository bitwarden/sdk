use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::api::response::IdentityTokenResponse, client::ApiConfigurations, error::Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenRequest {
    scope: String,
    client_id: String,
    client_secret: String,
    grant_type: String,
}

impl AccessTokenRequest {
    pub fn new(access_token_id: Uuid, client_secret: &String) -> Self {
        let obj = Self {
            scope: "api.secrets".to_string(),
            client_id: access_token_id.to_string(),
            client_secret: client_secret.to_string(),
            grant_type: "client_credentials".to_string(),
        };
        debug!("initializing {:?}", obj);
        obj
    }

    pub(crate) async fn send(
        &self,
        configurations: &ApiConfigurations,
    ) -> Result<IdentityTokenResponse> {
        super::send_identity_connect_request(configurations, None, &self).await
    }
}
