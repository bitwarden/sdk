use bitwarden_api_api::models::SecretCreateRequestModel;
use bitwarden_core::{Client, Error, VaultLocked};
use bitwarden_crypto::KeyEncryptable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SecretResponse;

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
    client: &Client,
    input: &SecretCreateRequest,
) -> Result<SecretResponse, Error> {
    let enc = client.internal.get_encryption_settings()?;
    let key = enc
        .get_key(&Some(input.organization_id))
        .ok_or(VaultLocked)?;

    let secret = Some(SecretCreateRequestModel {
        key: input.key.clone().encrypt_with_key(key)?.to_string(),
        value: input.value.clone().encrypt_with_key(key)?.to_string(),
        note: input.note.clone().encrypt_with_key(key)?.to_string(),
        project_ids: input.project_ids.clone(),
    });

    let config = client.internal.get_api_configurations().await;
    let res = bitwarden_api_api::apis::secrets_api::organizations_organization_id_secrets_post(
        &config.api,
        input.organization_id,
        secret,
    )
    .await?;

    SecretResponse::process_response(res, &enc)
}
