use bitwarden_api_api::models::ProjectResponseModel;
use bitwarden_core::{
    key_management::{AsymmetricKeyRef, SymmetricKeyRef},
    require, Error,
};
use bitwarden_crypto::{service::CryptoServiceContext, Decryptable, EncString};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
    ) -> Result<Self, Error> {
        let organization_id = require!(response.organization_id);
        let enc_key = SymmetricKeyRef::Organization(organization_id);

        let name = require!(response.name)
            .parse::<EncString>()?
            .decrypt(ctx, enc_key)?;

        Ok(ProjectResponse {
            id: require!(response.id),
            organization_id,
            name,

            creation_date: require!(response.creation_date).parse()?,
            revision_date: require!(response.revision_date).parse()?,
        })
    }
}
