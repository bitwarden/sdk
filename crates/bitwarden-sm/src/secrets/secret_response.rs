use bitwarden_api_api::models::{
    BaseSecretResponseModel, BaseSecretResponseModelListResponseModel, SecretResponseModel,
};
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
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
    ) -> Result<SecretResponse, Error> {
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
        Self::process_base_response(base, ctx)
    }
    pub(crate) fn process_base_response(
        response: BaseSecretResponseModel,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
    ) -> Result<SecretResponse, Error> {
        let organization_id = require!(response.organization_id);
        let enc_key = SymmetricKeyRef::Organization(organization_id);

        let key = require!(response.key)
            .parse::<EncString>()?
            .decrypt(ctx, enc_key)?;
        let value = require!(response.value)
            .parse::<EncString>()?
            .decrypt(ctx, enc_key)?;
        let note = require!(response.note)
            .parse::<EncString>()?
            .decrypt(ctx, enc_key)?;

        let project = response
            .projects
            .and_then(|p| p.into_iter().next())
            .and_then(|p| p.id);

        Ok(SecretResponse {
            id: require!(response.id),
            organization_id,
            project_id: project,
            key,
            value,
            note,

            creation_date: require!(response.creation_date).parse()?,
            revision_date: require!(response.revision_date).parse()?,
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
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
    ) -> Result<SecretsResponse, Error> {
        Ok(SecretsResponse {
            data: response
                .data
                .unwrap_or_default()
                .into_iter()
                .map(|r| SecretResponse::process_base_response(r, ctx))
                .collect::<Result<_, _>>()?,
        })
    }
}
