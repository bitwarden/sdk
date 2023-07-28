use bitwarden_api_api::models::SecretUpdateRequestModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    SecretResponse, SECRET_KEY_MAX_LENGTH, SECRET_NOTE_MAX_LENGTH, SECRET_VALUE_MAX_LENGTH,
};

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
    validate(input)?;

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let org_id = Some(input.organization_id);

    let secret = Some(SecretUpdateRequestModel {
        key: enc.encrypt(input.key.as_bytes(), &org_id)?.to_string(),
        value: enc.encrypt(input.value.as_bytes(), &org_id)?.to_string(),
        note: enc.encrypt(input.note.as_bytes(), &org_id)?.to_string(),
        project_ids: input.project_ids.clone(),
    });

    let config = client.get_api_configurations().await;
    let res =
        bitwarden_api_api::apis::secrets_api::secrets_id_put(&config.api, input.id, secret).await?;

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    SecretResponse::process_response(res, enc)
}

fn validate(input: &SecretPutRequest) -> Result<()> {
    if input.key.len() > SECRET_KEY_MAX_LENGTH {
        return Err(Error::FieldLengthExceeded {
            field_name: "key",
            maximum_length: SECRET_KEY_MAX_LENGTH,
        });
    }

    if input.value.len() > SECRET_VALUE_MAX_LENGTH {
        return Err(Error::FieldLengthExceeded {
            field_name: "value",
            maximum_length: SECRET_VALUE_MAX_LENGTH,
        });
    }

    if input.note.len() > SECRET_NOTE_MAX_LENGTH {
        return Err(Error::FieldLengthExceeded {
            field_name: "note",
            maximum_length: SECRET_NOTE_MAX_LENGTH,
        });
    }

    Ok(())
}
