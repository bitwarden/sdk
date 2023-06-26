use bitwarden_api_api::models::ProjectResponseModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
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

        let name = enc.decrypt_str(
            &response.name.ok_or(Error::MissingFields)?,
            Some(organization_id),
        )?;

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
