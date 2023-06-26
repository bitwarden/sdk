use bitwarden_api_api::models::{
    BulkDeleteResponseModel, BulkDeleteResponseModelListResponseModel,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::Client,
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsDeleteRequest {
    /// IDs of the secrets to delete
    pub ids: Vec<Uuid>,
}

pub(crate) async fn delete_secrets(
    client: &mut Client,
    input: SecretsDeleteRequest,
) -> Result<SecretsDeleteResponse> {
    let config = client.get_api_configurations().await;
    let res =
        bitwarden_api_api::apis::secrets_api::secrets_delete_post(&config.api, Some(input.ids))
            .await?;

    SecretsDeleteResponse::process_response(res)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsDeleteResponse {
    pub data: Vec<SecretDeleteResponse>,
}

impl SecretsDeleteResponse {
    pub(crate) fn process_response(
        response: BulkDeleteResponseModelListResponseModel,
    ) -> Result<SecretsDeleteResponse> {
        Ok(SecretsDeleteResponse {
            data: response
                .data
                .unwrap_or_default()
                .into_iter()
                .map(SecretDeleteResponse::process_response)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretDeleteResponse {
    pub id: Uuid,
    pub error: Option<String>,
}

impl SecretDeleteResponse {
    pub(crate) fn process_response(
        response: BulkDeleteResponseModel,
    ) -> Result<SecretDeleteResponse> {
        Ok(SecretDeleteResponse {
            id: response.id.ok_or(Error::MissingFields)?,
            error: response.error,
        })
    }
}
