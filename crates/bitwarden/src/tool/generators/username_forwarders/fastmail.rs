use std::collections::HashMap;

use reqwest::{header::CONTENT_TYPE, StatusCode};
use serde_json::json;

use crate::error::{Error, Result};
pub async fn generate(
    http: &reqwest::Client,
    api_token: String,
    website: Option<String>,
) -> Result<String> {
    if api_token.is_empty() {
        return Err(Error::Internal("Invalid Fastmail API token"));
    }

    let account_id = get_account_id(http, &api_token).await?;

    let response = http
        .post("https://api.fastmail.com/jmap/api/")
        .header(CONTENT_TYPE, "application/json")
        .bearer_auth(api_token)
        .json(&json!({
            "using": ["https://www.fastmail.com/dev/maskedemail", "urn:ietf:params:jmap:core"],
            "methodCalls": [[
                "MaskedEmail/set", {
                    "accountId": account_id,
                    "create": {
                        "new-masked-email": {
                            "state": "enabled",
                            "description": "",
                            "url": website,
                            "emailPrefix": null,
                        },
                    },
                },
                "0",
            ]],
        }))
        .send()
        .await?;

    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(Error::Internal("Invalid DuckDuckGo API token"));
    }

    // Throw any other errors
    response.error_for_status_ref()?;

    let response: serde_json::Value = response.json().await?;
    let Some(r) = response.get("methodResponses").and_then(|r| r.get(0)) else {
        return Err(Error::Internal("Unknown Fastmail error occurred."));
    };
    let method_response = r.get(0).and_then(|r| r.as_str());
    let response_value = r.get(1);

    if method_response == Some("MaskedEmail/set") {
        if let Some(email) = response_value
            .and_then(|r| r.get("created"))
            .and_then(|r| r.get("new-masked-email"))
            .and_then(|r| r.get("email"))
            .and_then(|r| r.as_str())
        {
            return Ok(email.to_owned());
        };

        if let Some(_error_description) = response_value
            .and_then(|r| r.get("notCreated"))
            .and_then(|r| r.get("new-masked-email"))
            .and_then(|r| r.get("description"))
            .and_then(|r| r.as_str())
        {
            // TODO: Once we have a more flexible type of error, we can return this error_description
            return Err(Error::Internal("Unknown Fastmail error occurred."));
        };
    } else if method_response == Some("error") {
        let _description = response_value
            .and_then(|r| r.get("description"))
            .and_then(|r| r.as_str());

        // TODO: Once we have a more flexible type of error, we can return this error_description
        return Err(Error::Internal("Unknown Fastmail error occurred."));
    }

    Err(Error::Internal("Unknown Fastmail error occurred."))
}

async fn get_account_id(client: &reqwest::Client, api_token: &str) -> Result<String> {
    #[derive(serde::Deserialize)]
    struct Response {
        #[serde(rename = "primaryAccounts")]
        primary_accounts: HashMap<String, String>,
    }
    let mut response: Response = client
        .get("https://api.fastmail.com/.well-known/jmap")
        .bearer_auth(api_token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(response
        .primary_accounts
        .remove("https://www.fastmail.com/dev/maskedemail")
        .unwrap_or_default())
}
