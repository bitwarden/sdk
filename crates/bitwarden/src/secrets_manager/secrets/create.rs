use bitwarden_api_api::models::SecretCreateRequestModel;
use bitwarden_crypto::KeyEncryptable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SecretResponse;
use crate::{
    Client,
    error::{validate, Error, Result, validate_not_empty},
};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretCreateRequest {
    /// Organization where the secret will be created
    pub organization_id: Uuid,
    #[validate(
        length(max = 500, message = "secret key"),
        custom(function = validate_not_empty, message = "secret key")
    )]
    pub key: String,
    #[validate(length(max = 25_000, message = "secret value"))]
    pub value: String,
    #[validate(
        length(max = 7_000, message = "secret note"),
        custom(function = validate_not_empty, message = "secret note")
    )]
    pub note: String,

    /// IDs of the projects that this secret will belong to
    pub project_ids: Option<Vec<Uuid>>,
}

pub(crate) async fn create_secret(
    client: &mut Client,
    input: &SecretCreateRequest,
) -> Result<SecretResponse> {
    validate!(input);

    let key = client
        .get_encryption_settings()?
        .get_key(&Some(input.organization_id))
        .ok_or(Error::VaultLocked)?;

    let secret = Some(SecretCreateRequestModel {
        key: input.key.trim().to_string().clone().encrypt_with_key(key)?.to_string(),
        value: input.value.clone().encrypt_with_key(key)?.to_string(),
        note: input.note.trim().to_string().clone().encrypt_with_key(key)?.to_string(),
        project_ids: input.project_ids.clone(),
    });

    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::secrets_api::organizations_organization_id_secrets_post(
        &config.api,
        input.organization_id,
        secret,
    )
    .await?;

    let enc = client.get_encryption_settings()?;

    SecretResponse::process_response(res, enc)
}
