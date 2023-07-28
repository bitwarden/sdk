use bitwarden_api_api::models::ProjectUpdateRequestModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{ProjectResponse, PROJECT_NAME_MAX_LENGTH};

use crate::{
    client::Client,
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectPutRequest {
    /// ID of the project to modify
    pub id: Uuid,
    /// Organization ID of the project to modify
    pub organization_id: Uuid,

    pub name: String,
}

pub(crate) async fn update_project(
    client: &mut Client,
    input: &ProjectPutRequest,
) -> Result<ProjectResponse> {
    validate(input)?;

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let org_id = Some(input.organization_id);

    let project = Some(ProjectUpdateRequestModel {
        name: enc.encrypt(input.name.as_bytes(), &org_id)?.to_string(),
    });

    let config = client.get_api_configurations().await;
    let res =
        bitwarden_api_api::apis::projects_api::projects_id_put(&config.api, input.id, project)
            .await?;

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    ProjectResponse::process_response(res, enc)
}

fn validate(input: &ProjectPutRequest) -> Result<()> {
    if input.name.len() > PROJECT_NAME_MAX_LENGTH {
        return Err(Error::FieldLengthExceeded {
            field_name: "name",
            maximum_length: PROJECT_NAME_MAX_LENGTH,
        });
    }

    Ok(())
}
