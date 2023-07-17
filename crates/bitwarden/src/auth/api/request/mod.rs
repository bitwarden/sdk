mod access_token_request;
#[cfg(feature = "internal")]
mod api_token_request;
#[cfg(feature = "internal")]
mod password_token_request;
#[cfg(feature = "internal")]
mod renew_token_request;

pub(crate) use access_token_request::*;
#[cfg(feature = "internal")]
pub(crate) use api_token_request::*;
#[cfg(feature = "internal")]
pub(crate) use password_token_request::*;
#[cfg(feature = "internal")]
pub(crate) use renew_token_request::*;

use base64::Engine;

use crate::{
    auth::api::response::{parse_identity_response, IdentityTokenResponse},
    client::ApiConfigurations,
    error::Result,
    util::BASE64_ENGINE,
};

async fn send_identity_connect_request(
    configurations: &ApiConfigurations,
    email: Option<&str>,
    body: impl serde::Serialize,
) -> Result<IdentityTokenResponse> {
    let mut request = configurations
        .identity
        .client
        .post(format!(
            "{}/connect/token",
            &configurations.identity.base_path
        ))
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=utf-8",
        )
        .header("Accept", "application/json")
        .header("Device-Type", configurations.device_type as usize);

    if let Some(ref user_agent) = configurations.identity.user_agent {
        request = request.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    if let Some(email) = email {
        request = request.header("Auth-Email", BASE64_ENGINE.encode(email.as_bytes()));
    }

    let response = request
        .body(serde_qs::to_string(&body).unwrap())
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;

    parse_identity_response(status, &text)
}
