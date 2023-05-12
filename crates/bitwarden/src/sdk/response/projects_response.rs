use bitwarden_api_api::models::{ProjectResponseModel, ProjectResponseModelListResponseModel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::CipherString,
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectResponse {
    pub object: String,
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub creation_date: String,
    pub revision_date: String,
}

impl ProjectResponse {
    pub(crate) fn process_response(
        response: ProjectResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<Self> {
        let organization_id = response.organization_id.ok_or(Error::MissingFields)?;

        use crate::crypto::Decryptable;
        let name = response
            .name
            .ok_or(Error::MissingFields)?
            .parse::<CipherString>()?
            .decrypt(enc, &Some(organization_id))?;

        Ok(ProjectResponse {
            object: "project".to_owned(),

            id: response.id.ok_or(Error::MissingFields)?,
            organization_id,
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
