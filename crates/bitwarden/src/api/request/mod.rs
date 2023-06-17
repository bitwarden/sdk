mod access_token_request;
mod api_token_request;
mod password_token_request;
mod renew_token_request;

pub(crate) use access_token_request::*;
pub(crate) use api_token_request::*;
use base64::Engine;
pub(crate) use password_token_request::*;
pub(crate) use renew_token_request::*;

use crate::{
    api::response::{parse_identity_response, IdentityTokenResponse},
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
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded; charset=utf-8",
        )
        .header(reqwest::header::ACCEPT, "application/json")
        .header("Device-Type", configurations.device_type as usize);

    if let Some(ref user_agent) = configurations.identity.user_agent {
        request = request.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    if let Some(email) = email {
        request = request.header("Auth-Email", BASE64_ENGINE.encode(email.as_bytes()));
    }

    let raw_response = request
        .body(serde_qs::to_string(&body).unwrap())
        .send()
        .await?
        .text()
        .await?;

    parse_identity_response(&raw_response)
}
