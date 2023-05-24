use bitwarden_api_api::models::{ProjectCreateRequestModel, ProjectUpdateRequestModel};

use crate::{
    client::Client,
    error::{Error, Result},
    sdk::{
        request::projects_request::{
            ProjectCreateRequest, ProjectGetRequest, ProjectPutRequest, ProjectsDeleteRequest,
            ProjectsListRequest,
        },
        response::projects_response::{ProjectResponse, ProjectsDeleteResponse, ProjectsResponse},
    },
};

pub(crate) async fn get_project(
    client: &mut Client,
    input: &ProjectGetRequest,
) -> Result<ProjectResponse> {
    let config = client.get_api_configurations().await;

    let res = bitwarden_api_api::apis::projects_api::projects_id_get(&config.api, input.id).await?;

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    ProjectResponse::process_response(res, enc)
}

pub(crate) async fn create_project(
    client: &mut Client,
    input: &ProjectCreateRequest,
) -> Result<ProjectResponse> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let org_id = Some(input.organization_id.as_str());

    let project = Some(ProjectCreateRequestModel {
        name: enc.encrypt(input.name.as_bytes(), org_id)?.to_string(),
    });

    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::projects_api::organizations_organization_id_projects_post(
        &config.api,
        &input.organization_id,
        project,
    )
    .await?;

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    ProjectResponse::process_response(res, enc)
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

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    ProjectsResponse::process_response(res, enc)
}

pub(crate) async fn update_project(
    client: &mut Client,
    input: &ProjectPutRequest,
) -> Result<ProjectResponse> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let org_id = Some(input.organization_id.as_str());

    let project = Some(ProjectUpdateRequestModel {
        name: enc.encrypt(input.name.as_bytes(), org_id)?.to_string(),
    });

    let config = client.get_api_configurations().await;
    let res =
        bitwarden_api_api::apis::projects_api::projects_id_put(&config.api, &input.id, project)
            .await?;

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    ProjectResponse::process_response(res, enc)
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
