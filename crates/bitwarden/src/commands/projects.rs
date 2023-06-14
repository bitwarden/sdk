use crate::{
    client::Client,
    error::Result,
    sdk::{
        request::projects_request::{ProjectGetRequest, ProjectsListRequest},
        response::projects_response::{ProjectResponse, ProjectsResponse},
    },
};

pub(crate) async fn get_project(
    client: &mut Client,
    input: &ProjectGetRequest,
) -> Result<ProjectResponse> {
    let config = client.get_api_configurations().await;

    let enc = client.get_encryption_settings()?;

    let res = bitwarden_api_api::apis::projects_api::projects_id_get(&config.api, input.id).await?;

    ProjectResponse::process_response(res, &enc)
}

pub(crate) async fn list_projects(
    client: &mut Client,
    input: &ProjectsListRequest,
) -> Result<ProjectsResponse> {
    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::projects_api::organizations_organization_id_projects_get(
        &config.api,
        input.organization_id,
    )
    .await?;

    let enc = client.get_encryption_settings()?;

    ProjectsResponse::process_response(res, &enc)
}
