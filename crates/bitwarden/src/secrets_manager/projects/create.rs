use bitwarden_api_api::models::ProjectCreateRequestModel;
use bitwarden_crypto::KeyEncryptable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ProjectResponse;
use crate::{
    client::Client,
    error::{validate, Error, Result, validate_not_empty},
};
use validator::{Validate};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectCreateRequest {
    /// Organization where the project will be created
    pub organization_id: Uuid,
    #[validate(
        length(max = 500, message = "project name"),
        custom(function = validate_not_empty, message = "project name")
    )]
    pub name: String,
}

pub(crate) async fn create_project(
    client: &mut Client,
    input: &ProjectCreateRequest,
) -> Result<ProjectResponse> {
    validate!(input);

    let key = client
        .get_encryption_settings()?
        .get_key(&Some(input.organization_id))
        .ok_or(Error::VaultLocked)?;

    let project = Some(ProjectCreateRequestModel {
        name: input.name.trim().to_string().clone().encrypt_with_key(key)?.to_string(),
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
