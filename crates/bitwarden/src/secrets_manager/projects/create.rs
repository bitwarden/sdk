use bitwarden_api_api::models::ProjectCreateRequestModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ProjectResponse;
use crate::{
    client::Client,
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectCreateRequest {
    /// Organization where the project will be created
    pub organization_id: Uuid,

    pub name: String,
}

pub(crate) async fn create_project(
    client: &mut Client,
    input: &ProjectCreateRequest,
) -> Result<ProjectResponse> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let org_id = Some(input.organization_id);

    let project = Some(ProjectCreateRequestModel {
        name: enc.encrypt(input.name.as_bytes(), org_id)?.to_string(),
    });

    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::projects_api::organizations_organization_id_projects_post(
        &config.api,
        input.organization_id,
        project,
    )
    .await?;

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    ProjectResponse::process_response(res, enc)
}
