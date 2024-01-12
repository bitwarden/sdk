use bitwarden_api_api::models::{
    BaseSecretResponseModel, BaseSecretResponseModelListResponseModel, SecretResponseModel,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{EncString, KeyDecryptable},
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub project_id: Option<Uuid>,

    pub key: String,
    pub value: String,
    pub note: String,

    pub creation_date: DateTime<Utc>,
    pub revision_date: DateTime<Utc>,
}

impl SecretResponse {
    pub(crate) fn process_response(
        response: SecretResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<SecretResponse> {
        let base = BaseSecretResponseModel {
            object: response.object,
            id: response.id,
            organization_id: response.organization_id,
            key: response.key,
            value: response.value,
            note: response.note,
            creation_date: response.creation_date,
            revision_date: response.revision_date,
            projects: response.projects,
        };
        Self::process_base_response(base, enc)
    }
    pub(crate) fn process_base_response(
        response: BaseSecretResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<SecretResponse> {
        let org_id = response.organization_id.ok_or(Error::MissingFields)?;
        let key = enc.get_org_key(org_id).ok_or(Error::VaultLocked)?;

        let secret_key = response
            .key
            .ok_or(Error::MissingFields)?
            .parse::<EncString>()?
            .decrypt_with_key(key)?;
        let value = response
            .value
            .ok_or(Error::MissingFields)?
            .parse::<EncString>()?
            .decrypt_with_key(key)?;
        let note = response
            .note
            .ok_or(Error::MissingFields)?
            .parse::<EncString>()?
            .decrypt_with_key(key)?;

        let project = response
            .projects
            .and_then(|p| p.into_iter().next())
            .and_then(|p| p.id);

        Ok(SecretResponse {
            id: response.id.ok_or(Error::MissingFields)?,
            organization_id: org_id,
            project_id: project,
            key: secret_key,
            value,
            note,

            creation_date: response
                .creation_date
                .ok_or(Error::MissingFields)?
                .parse()?,
            revision_date: response
                .revision_date
                .ok_or(Error::MissingFields)?
                .parse()?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsResponse {
    pub data: Vec<SecretResponse>,
}

impl SecretsResponse {
    pub(crate) fn process_response(
        response: BaseSecretResponseModelListResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<SecretsResponse> {
        Ok(SecretsResponse {
            data: response
                .data
                .unwrap_or_default()
                .into_iter()
                .map(|r| SecretResponse::process_base_response(r, enc))
                .collect::<Result<_, _>>()?,
        })
    }
}
