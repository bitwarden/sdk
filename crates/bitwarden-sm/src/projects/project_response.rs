use bitwarden_api_api::models::ProjectResponseModel;
use bitwarden_crypto::{CryptoError, EncString, KeyDecryptable};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use bitwarden_core::{client::encryption_settings::EncryptionSettings, error::Result};

use crate::require;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub creation_date: DateTime<Utc>,
    pub revision_date: DateTime<Utc>,
}

impl ProjectResponse {
    pub(crate) fn process_response(
        response: ProjectResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<Self> {
        let organization_id = require!(response.organization_id);
        let enc_key = enc
            .get_key(&Some(organization_id))
            .ok_or(CryptoError::MissingKey)?;

        let name = require!(response.name)
            .parse::<EncString>()?
            .decrypt_with_key(enc_key)?;

        Ok(ProjectResponse {
            id: require!(response.id),
            organization_id,
            name,

            creation_date: require!(response.creation_date).parse()?,
            revision_date: require!(response.revision_date).parse()?,
        })
    }
}
