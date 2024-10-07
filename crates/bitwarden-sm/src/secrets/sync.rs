use bitwarden_api_api::models::SecretsSyncResponseModel;
use bitwarden_core::{
    key_management::{AsymmetricKeyRef, SymmetricKeyRef},
    require, Client, Error,
};
use bitwarden_crypto::service::CryptoServiceContext;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SecretResponse;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsSyncRequest {
    /// Organization to sync secrets from
    pub organization_id: Uuid,
    /// Optional date time a sync last occurred
    pub last_synced_date: Option<DateTime<Utc>>,
}

pub(crate) async fn sync_secrets(
    client: &Client,
    input: &SecretsSyncRequest,
) -> Result<SecretsSyncResponse, Error> {
    let config = client.internal.get_api_configurations().await;
    let last_synced_date = input.last_synced_date.map(|date| date.to_rfc3339());

    let res = bitwarden_api_api::apis::secrets_api::organizations_organization_id_secrets_sync_get(
        &config.api,
        input.organization_id,
        last_synced_date,
    )
    .await?;

    let mut ctx = client.internal.get_crypto_service().context();

    SecretsSyncResponse::process_response(res, &mut ctx)
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
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
    ) -> Result<SecretsSyncResponse, Error> {
        let has_changes = require!(response.has_changes);

        if has_changes {
            let secrets = require!(response.secrets)
                .data
                .unwrap_or_default()
                .into_iter()
                .map(|r| SecretResponse::process_base_response(r, ctx))
                .collect::<Result<_, _>>()?;
            return Ok(SecretsSyncResponse {
                has_changes,
                secrets: Some(secrets),
            });
        }

        Ok(SecretsSyncResponse {
            has_changes: false,
            secrets: None,
        })
    }
}
