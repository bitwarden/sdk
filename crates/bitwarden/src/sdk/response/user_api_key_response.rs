use bitwarden_api_api::models::ApiKeyResponseModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserApiKeyResponse {
    /// The user's API key, which represents the client_secret portion of an oauth request.
    api_key: String,
}

impl UserApiKeyResponse {
    pub(crate) fn process_response(response: ApiKeyResponseModel) -> Result<UserApiKeyResponse> {
        match response.api_key {
            Some(api_key) => Ok(UserApiKeyResponse { api_key }),
            None => Err(Error::MissingFields),
        }
    }
}
