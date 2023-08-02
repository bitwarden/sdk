use bitwarden_api_api::models::GetSecretsRequestModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SecretsResponse;
use crate::{client::Client, error::Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsGetRequest {
    /// IDs of the secrets to retrieve
    pub ids: Vec<Uuid>,
}

pub(crate) async fn get_secrets_by_ids(
    client: &mut Client,
    input: SecretsGetRequest,
) -> Result<SecretsResponse> {
    let request = Some(GetSecretsRequestModel { ids: input.ids });

    let config = client.get_api_configurations().await;

    let res =
        bitwarden_api_api::apis::secrets_api::secrets_get_by_ids_post(&config.api, request).await?;

    let enc = client.get_encryption_settings()?;

    SecretsResponse::process_response(res, enc)
}
