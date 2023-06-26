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
pub struct ProjectGetRequest {
    /// ID of the project to retrieve
    pub id: Uuid,
}

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
