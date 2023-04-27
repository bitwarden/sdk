use bitwarden_api_api::models::{ProjectResponseModel, ProjectResponseModelListResponseModel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    client::encryption_settings::EncryptionSettings,
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectResponse {
    pub object: String,
    pub id: String,
    pub organization_id: String,
    pub name: String,
    pub creation_date: String,
    pub revision_date: String,
}

impl ProjectResponse {
    pub(crate) fn process_response(
        response: ProjectResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<Self> {
        let name = enc.decrypt_str(
            &response.name.ok_or(Error::MissingFields)?,
            response.organization_id.as_deref(),
        )?;

        Ok(ProjectResponse {
            object: "project".to_owned(),

            id: response.id.ok_or(Error::MissingFields)?,
            organization_id: response.organization_id.ok_or(Error::MissingFields)?,
            name,

            creation_date: response.creation_date.ok_or(Error::MissingFields)?,
            revision_date: response.revision_date.ok_or(Error::MissingFields)?,
        })
    }
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
    ) -> Result<Self> {
        let data = response.data.unwrap_or_default();

        Ok(ProjectsResponse {
            data: data
                .into_iter()
                .map(|r| ProjectResponse::process_response(r, enc))
                .collect::<Result<_, _>>()?,
        })
    }
}
