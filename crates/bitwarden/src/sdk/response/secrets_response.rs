use bitwarden_api_api::models::{
    BulkDeleteResponseModel, BulkDeleteResponseModelListResponseModel, SecretResponseModel,
    SecretWithProjectsListResponseModel, SecretsWithProjectsInnerSecret,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretResponse {
    pub object: String,
    pub id: Uuid,
    pub organization_id: Uuid,
    pub project_id: Option<Uuid>,

    pub key: String,
    pub value: String,
    pub note: String,

    pub creation_date: String,
    pub revision_date: String,
}

impl SecretResponse {
    pub(crate) fn process_response(
        response: SecretResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<SecretResponse> {
        let org_id = response.organization_id;

        let key = enc.decrypt_str(&response.key.ok_or(Error::MissingFields)?, org_id)?;
        let value = enc.decrypt_str(&response.value.ok_or(Error::MissingFields)?, org_id)?;
        let note = enc.decrypt_str(&response.note.ok_or(Error::MissingFields)?, org_id)?;

        let project = response
            .projects
            .and_then(|p| p.into_iter().next())
            .and_then(|p| p.id);

        Ok(SecretResponse {
            object: "secret".to_owned(),
            id: response.id.ok_or(Error::MissingFields)?,
            organization_id: org_id.ok_or(Error::MissingFields)?,
            project_id: project,
            key,
            value,
            note,

            creation_date: response.creation_date.ok_or(Error::MissingFields)?,
            revision_date: response.revision_date.ok_or(Error::MissingFields)?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretIdentifiersResponse {
    pub data: Vec<SecretIdentifierResponse>,
}

impl SecretIdentifiersResponse {
    pub(crate) fn process_response(
        response: SecretWithProjectsListResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<SecretIdentifiersResponse> {
        Ok(SecretIdentifiersResponse {
            data: response
                .secrets
                .unwrap_or_default()
                .into_iter()
                .map(|r| SecretIdentifierResponse::process_response(r, enc))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretIdentifierResponse {
    pub id: Uuid,
    pub organization_id: Uuid,

    pub key: String,
}

impl SecretIdentifierResponse {
    pub(crate) fn process_response(
        response: SecretsWithProjectsInnerSecret,
        enc: &EncryptionSettings,
    ) -> Result<SecretIdentifierResponse> {
        let organization_id = response.organization_id.ok_or(Error::MissingFields)?;

        let key = enc.decrypt_str(
            &response.key.ok_or(Error::MissingFields)?,
            Some(organization_id),
        )?;

        Ok(SecretIdentifierResponse {
            id: response.id.ok_or(Error::MissingFields)?,
            organization_id,
            key,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsDeleteResponse {
    pub data: Vec<SecretDeleteResponse>,
}

impl SecretsDeleteResponse {
    pub(crate) fn process_response(
        response: BulkDeleteResponseModelListResponseModel,
    ) -> Result<SecretsDeleteResponse> {
        Ok(SecretsDeleteResponse {
            data: response
                .data
                .unwrap_or_default()
                .into_iter()
                .map(SecretDeleteResponse::process_response)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretDeleteResponse {
    pub id: Uuid,
    pub error: Option<String>,
}

impl SecretDeleteResponse {
    pub(crate) fn process_response(
        response: BulkDeleteResponseModel,
    ) -> Result<SecretDeleteResponse> {
        Ok(SecretDeleteResponse {
            id: response.id.ok_or(Error::MissingFields)?,
            error: response.error,
        })
    }
}
