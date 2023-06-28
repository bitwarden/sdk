use bitwarden_api_api::models::{
    BulkDeleteResponseModel, BulkDeleteResponseModelListResponseModel,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::Client,
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectsDeleteRequest {
    /// IDs of the projects to delete
    pub ids: Vec<Uuid>,
}

pub(crate) async fn delete_projects(
    client: &mut Client,
    input: ProjectsDeleteRequest,
) -> Result<ProjectsDeleteResponse> {
    let config = client.get_api_configurations().await;
    let res =
        bitwarden_api_api::apis::projects_api::projects_delete_post(&config.api, Some(input.ids))
            .await?;

    ProjectsDeleteResponse::process_response(res)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectsDeleteResponse {
    pub data: Vec<ProjectDeleteResponse>,
}

impl ProjectsDeleteResponse {
    pub(crate) fn process_response(
        response: BulkDeleteResponseModelListResponseModel,
    ) -> Result<ProjectsDeleteResponse> {
        Ok(ProjectsDeleteResponse {
            data: response
                .data
                .unwrap_or_default()
                .into_iter()
                .map(ProjectDeleteResponse::process_response)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectDeleteResponse {
    pub id: Uuid,
    pub error: Option<String>,
}

impl ProjectDeleteResponse {
    pub(crate) fn process_response(
        response: BulkDeleteResponseModel,
    ) -> Result<ProjectDeleteResponse> {
        Ok(ProjectDeleteResponse {
            id: response.id.ok_or(Error::MissingFields)?,
            error: response.error,
        })
    }
}
