use bitwarden_api_api::models::ProjectCreateRequestModel;
use bitwarden_crypto::KeyEncryptable;
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
    let key = client
        .get_encryption_settings()?
        .get_key(&Some(input.organization_id))
        .ok_or(Error::VaultLocked)?;

    let project = Some(ProjectCreateRequestModel {
        name: input.name.clone().encrypt_with_key(key)?.to_string(),
    });

    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::projects_api::organizations_organization_id_projects_post(
        &config.api,
        input.organization_id,
        project,
    )
    .await?;

    let enc = client.get_encryption_settings()?;

    ProjectResponse::process_response(res, enc)
}
