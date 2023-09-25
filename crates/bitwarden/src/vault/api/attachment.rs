use reqwest::Response;

use crate::error::Result;

pub(crate) async fn attachment_get(url: &str) -> Result<Response> {
    reqwest::get(url).await.map_err(|e| e.into())
}
