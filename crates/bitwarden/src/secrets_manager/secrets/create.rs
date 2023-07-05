use bitwarden_api_api::models::SecretCreateRequestModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SecretResponse;
use crate::{
    error::{Error, Result},
    Client,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretCreateRequest {
    /// Organization where the secret will be created
    pub organization_id: Uuid,

    pub key: String,
    pub value: String,
    pub note: String,

    /// IDs of the projects that this secret will belong to
    pub project_ids: Option<Vec<Uuid>>,
}

pub(crate) async fn create_secret(
    client: &mut Client,
    input: &SecretCreateRequest,
) -> Result<SecretResponse> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let org_id = Some(input.organization_id);

    let secret = Some(SecretCreateRequestModel {
        key: enc.encrypt(input.key.as_bytes(), org_id)?.to_string(),
        value: enc.encrypt(input.value.as_bytes(), org_id)?.to_string(),
        note: enc.encrypt(input.note.as_bytes(), org_id)?.to_string(),
        project_ids: input.project_ids.clone(),
    });

    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::secrets_api::organizations_organization_id_secrets_post(
        &config.api,
        input.organization_id,
        secret,
    )
    .await?;

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    SecretResponse::process_response(res, enc)
}
