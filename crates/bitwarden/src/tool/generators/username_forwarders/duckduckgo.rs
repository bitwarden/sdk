use reqwest::{header::CONTENT_TYPE, StatusCode};

use crate::error::{Error, Result};
pub async fn generate(http: &reqwest::Client, token: String) -> Result<String> {
    if token.is_empty() {
        return Err(Error::Internal("Invalid DuckDuckGo API token"));
    }

    let response = http
        .post("https://quack.duckduckgo.com/api/email/addresses")
        .header(CONTENT_TYPE, "application/json")
        .bearer_auth(token)
        .send()
        .await?;

    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(Error::Internal("Invalid DuckDuckGo API token"));
    }

    // Throw any other errors
    response.error_for_status_ref()?;

    #[derive(serde::Deserialize)]
    struct Response {
        address: String,
    }
    let response: Response = response.json().await?;

    Ok(format!("{}@duck.com", response.address))
}
