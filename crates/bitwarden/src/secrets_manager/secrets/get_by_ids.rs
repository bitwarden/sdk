use bitwarden_api_api::models::{BaseSecretResponseModelListResponseModel, GetSecretsRequestModel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SecretResponse;
use crate::{
    client::encryption_settings::EncryptionSettings,
    error::{Error, Result},
    Client,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsGetRequest {
    /// ID of the secret to retrieve
    pub ids: Vec<Uuid>,
}

pub(crate) async fn get_secrets_by_ids(
    client: &mut Client,
    input: SecretsGetRequest,
) -> Result<SecretsResponse> {
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

    SecretsResponse::process_response(res, enc)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsResponse {
    pub data: Vec<SecretResponse>,
}

impl SecretsResponse {
    pub(crate) fn process_response(
        response: BaseSecretResponseModelListResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<Self> {
        let data = response.data.unwrap_or_default();

        Ok(SecretsResponse {
            data: data
                .into_iter()
                .map(|r| SecretResponse::process_base_response(r, enc))
                .collect::<Result<_, _>>()?,
        })
    }
}
