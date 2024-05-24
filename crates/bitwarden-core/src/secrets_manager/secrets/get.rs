use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SecretResponse;
use crate::{error::Result, Client};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretGetRequest {
    /// ID of the secret to retrieve
    pub id: Uuid,
}

pub(crate) async fn get_secret(
    client: &mut Client,
    input: &SecretGetRequest,
) -> Result<SecretResponse> {
    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::secrets_api::secrets_id_get(&config.api, input.id).await?;

    let enc = client.get_encryption_settings()?;

    SecretResponse::process_response(res, enc)
}
