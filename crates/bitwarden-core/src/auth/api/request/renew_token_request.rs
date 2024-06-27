use serde::{Deserialize, Serialize};

use crate::{auth::api::response::IdentityTokenResponse, client::ApiConfigurations, error::Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct RenewTokenRequest {
    grant_type: String,
    refresh_token: String,
    client_id: String,
}

impl RenewTokenRequest {
    pub fn new(refresh_token: String, client_id: String) -> Self {
        Self {
            refresh_token,
            client_id,
            grant_type: "refresh_token".to_string(),
        }
    }

    pub(crate) async fn send(
        &self,
        configurations: &ApiConfigurations,
    ) -> Result<IdentityTokenResponse> {
        super::send_identity_connect_request(configurations, None, &self).await
    }
}
