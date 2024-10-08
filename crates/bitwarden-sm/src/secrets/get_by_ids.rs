use bitwarden_api_api::models::GetSecretsRequestModel;
use bitwarden_core::{client::Client, Error};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SecretsResponse;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsGetRequest {
    /// IDs of the secrets to retrieve
    pub ids: Vec<Uuid>,
}

pub(crate) async fn get_secrets_by_ids(
    client: &Client,
    input: SecretsGetRequest,
) -> Result<SecretsResponse, Error> {
    let request = Some(GetSecretsRequestModel { ids: input.ids });

    let config = client.internal.get_api_configurations().await;

    let res =
        bitwarden_api_api::apis::secrets_api::secrets_get_by_ids_post(&config.api, request).await?;

    let mut ctx = client.internal.get_crypto_service().context();

    SecretsResponse::process_response(res, &mut ctx)
}
