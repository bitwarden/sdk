use bitwarden_api_api::models::ProjectResponseModelListResponseModel;
use bitwarden_core::{
    client::Client,
    key_management::{AsymmetricKeyRef, SymmetricKeyRef},
    Error,
};
use bitwarden_crypto::service::CryptoServiceContext;
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
    client: &Client,
    input: &ProjectsListRequest,
) -> Result<ProjectsResponse, Error> {
    let config = client.internal.get_api_configurations().await;
    let res = bitwarden_api_api::apis::projects_api::organizations_organization_id_projects_get(
        &config.api,
        input.organization_id,
    )
    .await?;

    let mut ctx = client.internal.get_crypto_service().context();

    ProjectsResponse::process_response(res, &mut ctx)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectsResponse {
    pub data: Vec<ProjectResponse>,
}

impl ProjectsResponse {
    pub(crate) fn process_response(
        response: ProjectResponseModelListResponseModel,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
    ) -> Result<Self, Error> {
        let data = response.data.unwrap_or_default();

        Ok(ProjectsResponse {
            data: data
                .into_iter()
                .map(|r| ProjectResponse::process_response(r, ctx))
                .collect::<Result<_, _>>()?,
        })
    }
}
