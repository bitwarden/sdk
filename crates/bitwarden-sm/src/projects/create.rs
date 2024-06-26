use bitwarden_api_api::models::ProjectCreateRequestModel;
use bitwarden_core::{Client, Error, VaultLocked};
use bitwarden_crypto::KeyEncryptable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ProjectResponse;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectCreateRequest {
    /// Organization where the project will be created
    pub organization_id: Uuid,

    pub name: String,
}

pub(crate) async fn create_project(
    client: &Client,
    input: &ProjectCreateRequest,
) -> Result<ProjectResponse, Error> {
    let enc = client.internal.get_encryption_settings()?;
    let key = enc
        .get_key(&Some(input.organization_id))
        .ok_or(VaultLocked)?;

    let project = Some(ProjectCreateRequestModel {
        name: input.name.clone().encrypt_with_key(key)?.to_string(),
    });

    let config = client.internal.get_api_configurations().await;
    let res = bitwarden_api_api::apis::projects_api::organizations_organization_id_projects_post(
        &config.api,
        input.organization_id,
        project,
    )
    .await?;

    ProjectResponse::process_response(res, &enc)
}
