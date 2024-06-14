use bitwarden_api_api::models::ProjectResponseModelListResponseModel;
use bitwarden_core::{
    client::{encryption_settings::EncryptionSettings, Client},
    Error,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ProjectResponse;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectsListRequest {
    /// Organization to retrieve all the projects from
    pub organization_id: Uuid,
}

pub(crate) async fn list_projects(
    client: &mut Client,
    input: &ProjectsListRequest,
) -> Result<ProjectsResponse, Error> {
    let config = client.internal.get_api_configurations().await;
    let res = bitwarden_api_api::apis::projects_api::organizations_organization_id_projects_get(
        &config.api,
        input.organization_id,
    )
    .await?;

    let enc = client.internal.get_encryption_settings()?;

    ProjectsResponse::process_response(res, &enc)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectsResponse {
    pub data: Vec<ProjectResponse>,
}

impl ProjectsResponse {
    pub(crate) fn process_response(
        response: ProjectResponseModelListResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<Self, Error> {
        let data = response.data.unwrap_or_default();

        Ok(ProjectsResponse {
            data: data
                .into_iter()
                .map(|r| ProjectResponse::process_response(r, enc))
                .collect::<Result<_, _>>()?,
        })
    }
}
