use bitwarden_api_api::models::{BaseSecretResponseModelListResponseModel, GetSecretsRequestModel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::{encryption_settings::EncryptionSettings, Client},
    error::{Error, Result},
};

use super::SecretResponse;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsGetRequest {
    /// IDs of the secrets to retrieve
    pub ids: Vec<Uuid>,
}

pub(crate) async fn get_secrets_by_ids(
    client: &mut Client,
    input: SecretsGetRequest,
) -> Result<SecretsGetByIdsResponse> {
    let config = client.get_api_configurations().await;

    let request = Some(GetSecretsRequestModel {
        ids: input.ids.clone(),
    });

    let res =
        bitwarden_api_api::apis::secrets_api::secrets_get_by_ids_post(&config.api, request).await?;

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    SecretsGetByIdsResponse::process_response(res, enc)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsGetByIdsResponse {
    pub data: Vec<SecretResponse>,
}

impl SecretsGetByIdsResponse {
    pub(crate) fn process_response(
        response: BaseSecretResponseModelListResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<Self> {
        let data = response.data.unwrap_or_default();

        Ok(SecretsGetByIdsResponse {
            data: data
                .into_iter()
                .map(|r| SecretResponse::process_base_response(r, enc))
                .collect::<Result<_, _>>()?,
        })
    }
}
