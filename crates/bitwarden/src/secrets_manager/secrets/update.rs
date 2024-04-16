use bitwarden_api_api::models::SecretUpdateRequestModel;
use bitwarden_crypto::KeyEncryptable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SecretResponse;
use crate::{
    client::Client,
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretPutRequest {
    /// ID of the secret to modify
    pub id: Uuid,
    /// Organization ID of the secret to modify
    pub organization_id: Uuid,

    pub key: String,
    pub value: String,
    pub note: String,
    pub project_ids: Option<Vec<Uuid>>,
}

pub(crate) async fn update_secret(
    client: &mut Client,
    input: &SecretPutRequest,
) -> Result<SecretResponse> {
    let key = client
        .get_encryption_settings()?
        .get_key(&Some(input.organization_id))
        .ok_or(Error::VaultLocked)?;

    let secret = Some(SecretUpdateRequestModel {
        key: input.key.clone().encrypt_with_key(key)?.to_string(),
        value: input.value.clone().encrypt_with_key(key)?.to_string(),
        note: input.note.clone().encrypt_with_key(key)?.to_string(),
        project_ids: input.project_ids.clone(),
    });

    let config = client.get_api_configurations().await;
    let res =
        bitwarden_api_api::apis::secrets_api::secrets_id_put(&config.api, input.id, secret).await?;

    let enc = client.get_encryption_settings()?;

    SecretResponse::process_response(res, enc)
}
