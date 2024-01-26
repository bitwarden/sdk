use std::collections::HashMap;

use reqwest::{header::CONTENT_TYPE, StatusCode};
use serde_json::json;

use crate::username::UsernameError;

pub async fn generate(
    http: &reqwest::Client,
    api_token: String,
    website: Option<String>,
) -> Result<String, UsernameError> {
    generate_with_api_url(http, api_token, website, "https://api.fastmail.com".into()).await
}

pub async fn generate_with_api_url(
    http: &reqwest::Client,
    api_token: String,
    website: Option<String>,
    api_url: String,
) -> Result<String, UsernameError> {
    let account_id = get_account_id(http, &api_token, &api_url).await?;

    let response = http
        .post(format!("{api_url}/jmap/api/"))
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

    let status_code = response.status();
    if status_code == StatusCode::UNAUTHORIZED {
        return Err(UsernameError::InvalidApiKey);
    }

    let response_json: serde_json::Value = response.json().await?;
    let Some(r) = response_json.get("methodResponses").and_then(|r| r.get(0)) else {
        return Err(UsernameError::Unknown);
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

        let error_description = response_value
            .and_then(|r| r.get("notCreated"))
            .and_then(|r| r.get("new-masked-email"))
            .and_then(|r| r.get("description"))
            .and_then(|r| r.as_str())
            .unwrap_or("Unknown error");

        return Err(UsernameError::ResponseContent {
            status: status_code,
            message: error_description.to_owned(),
        });
    } else if method_response == Some("error") {
        let error_description = response_value
            .and_then(|r| r.get("description"))
            .and_then(|r| r.as_str())
            .unwrap_or("Unknown error");

        return Err(UsernameError::ResponseContent {
            status: status_code,
            message: error_description.to_owned(),
        });
    }

    Err(UsernameError::Unknown)
}

async fn get_account_id(
    client: &reqwest::Client,
    api_token: &str,
    api_url: &str,
) -> Result<String, UsernameError> {
    #[derive(serde::Deserialize)]
    struct Response {
        #[serde(rename = "primaryAccounts")]
        primary_accounts: HashMap<String, String>,
    }
    let response = client
        .get(format!("{api_url}/.well-known/jmap"))
        .bearer_auth(api_token)
        .send()
        .await?;

    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(UsernameError::InvalidApiKey);
    }

    response.error_for_status_ref()?;
    let mut response: Response = response.json().await?;

    Ok(response
        .primary_accounts
        .remove("https://www.fastmail.com/dev/maskedemail")
        .unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::username::UsernameError;
    #[tokio::test]
    async fn test_mock_server() {
        use wiremock::{matchers, Mock, ResponseTemplate};

        let server = wiremock::MockServer::start().await;

        // Mock a valid request to FastMail API
        server.register(Mock::given(matchers::path("/.well-known/jmap"))
                .and(matchers::method("GET"))
                .and(matchers::header("Authorization", "Bearer MY_TOKEN"))
                .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                    "primaryAccounts": {
                        "https://www.fastmail.com/dev/maskedemail": "ca0a4e09-c266-4f6f-845c-958db5090f09"
                    }
                })))
                .expect(1)).await;

        server.register(Mock::given(matchers::path("/jmap/api/"))
                .and(matchers::method("POST"))
                .and(matchers::header("Content-Type", "application/json"))
                .and(matchers::header("Authorization", "Bearer MY_TOKEN"))
                .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                    "methodResponses": [
                        ["MaskedEmail/set", {"created": {"new-masked-email": {"email": "9f823dq23d123ds@mydomain.com"}}}]
                    ]
                })))
                .expect(1)).await;

        // Mock an invalid token request
        server
            .register(
                Mock::given(matchers::path("/.well-known/jmap"))
                    .and(matchers::method("GET"))
                    .and(matchers::header("Authorization", "Bearer MY_FAKE_TOKEN"))
                    .respond_with(ResponseTemplate::new(401))
                    .expect(1),
            )
            .await;

        server
            .register(
                Mock::given(matchers::path("/jmap/api/"))
                    .and(matchers::method("POST"))
                    .and(matchers::header("Content-Type", "application/json"))
                    .and(matchers::header("Authorization", "Bearer MY_FAKE_TOKEN"))
                    .respond_with(ResponseTemplate::new(201))
                    .expect(0),
            )
            .await;

        let address = super::generate_with_api_url(
            &reqwest::Client::new(),
            "MY_TOKEN".into(),
            Some("example.com".into()),
            format!("http://{}", server.address()),
        )
        .await
        .unwrap();
        assert_eq!(address, "9f823dq23d123ds@mydomain.com");

        let fake_token_error = super::generate_with_api_url(
            &reqwest::Client::new(),
            "MY_FAKE_TOKEN".into(),
            Some("example.com".into()),
            format!("http://{}", server.address()),
        )
        .await
        .unwrap_err();

        assert_eq!(
            fake_token_error.to_string(),
            UsernameError::InvalidApiKey.to_string()
        );

        server.verify().await;
    }
}
