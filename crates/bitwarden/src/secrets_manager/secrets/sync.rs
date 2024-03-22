use bitwarden_api_api::models::SecretsSyncResponseModel;
use chrono::{DateTime, Utc};
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
pub struct SecretsSyncRequest {
    /// Organization to sync secrets from
    pub organization_id: Uuid,
    /// Optional date time a sync last occurred
    pub last_synced_date: Option<DateTime<Utc>>,
}

pub(crate) async fn sync_secrets(
    client: &mut Client,
    input: &SecretsSyncRequest,
) -> Result<SecretsSyncResponse> {
    let config = client.get_api_configurations().await;
    let last_synced_date = input.last_synced_date.map(|date| date.to_rfc3339());

    let res = bitwarden_api_api::apis::secrets_api::organizations_organization_id_secrets_sync_get(
        &config.api,
        input.organization_id,
        last_synced_date,
    )
    .await?;

    let enc = client.get_encryption_settings()?;

    SecretsSyncResponse::process_response(res, enc)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsSyncResponse {
    pub has_changes: bool,
    pub secrets: Option<Vec<SecretResponse>>,
}

impl SecretsSyncResponse {
    pub(crate) fn process_response(
        response: SecretsSyncResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<SecretsSyncResponse> {
        let has_changes = response.has_changes.ok_or(Error::MissingFields)?;

        if has_changes && response.secrets.is_some() {
            let secrets = response
                .secrets
                .unwrap()
                .data
                .unwrap_or_default()
                .into_iter()
                .map(|r| SecretResponse::process_base_response(r, enc))
                .collect::<Result<_, _>>()?;

            return Ok(SecretsSyncResponse {
                has_changes,
                secrets: Some(secrets),
            });
        } else if has_changes && response.secrets.is_none() {
            return Err(Error::MissingFields);
        }

        return Ok(SecretsSyncResponse {
            has_changes: false,
            secrets: None,
        });
    }
}
