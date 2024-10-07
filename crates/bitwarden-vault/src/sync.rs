use bitwarden_api_api::models::{
    DomainsResponseModel, ProfileOrganizationResponseModel, ProfileResponseModel, SyncResponseModel,
};
use bitwarden_core::{
    key_management::{AsymmetricKeyRef, SymmetricKeyRef},
    require, Client, Error, MissingFieldError,
};
use bitwarden_crypto::service::CryptoServiceContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{Cipher, Collection, Folder, GlobalDomains, VaultParseError};

#[derive(Debug, Error)]
pub enum SyncError {
    #[error(transparent)]
    Core(#[from] bitwarden_core::Error),

    #[error(transparent)]
    MissingFieldError(#[from] MissingFieldError),

    #[error(transparent)]
    VaultParse(#[from] VaultParseError),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SyncRequest {
    /// Exclude the subdomains from the response, defaults to false
    pub exclude_subdomains: Option<bool>,
}

pub(crate) async fn sync(client: &Client, input: &SyncRequest) -> Result<SyncResponse, SyncError> {
    let config = client.internal.get_api_configurations().await;
    let sync = bitwarden_api_api::apis::sync_api::sync_get(&config.api, input.exclude_subdomains)
        .await
        .map_err(|e| SyncError::Core(e.into()))?;

    let org_keys: Vec<_> = require!(sync.profile.as_ref())
        .organizations
        .as_deref()
        .unwrap_or_default()
        .iter()
        .filter_map(|o| o.id.zip(o.key.as_deref().and_then(|k| k.parse().ok())))
        .collect();

    client
        .internal
        .initialize_org_crypto(org_keys)
        .map_err(bitwarden_core::Error::EncryptionSettings)?;

    let crypto = client.internal.get_crypto_service();

    SyncResponse::process_response(sync, &mut crypto.context())
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProfileResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,

    //key: String,
    //private_key: String,
    pub organizations: Vec<ProfileOrganizationResponse>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProfileOrganizationResponse {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DomainResponse {
    pub equivalent_domains: Vec<Vec<String>>,
    pub global_equivalent_domains: Vec<GlobalDomains>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SyncResponse {
    /// Data about the user, including their encryption keys and the organizations they are a part
    /// of
    pub profile: ProfileResponse,
    pub folders: Vec<Folder>,
    pub collections: Vec<Collection>,
    /// List of ciphers accessible by the user
    pub ciphers: Vec<Cipher>,
    pub domains: Option<DomainResponse>,
    //pub policies: Vec<Policy>,
    //pub sends: Vec<Send>,
}

impl SyncResponse {
    pub(crate) fn process_response(
        response: SyncResponseModel,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
    ) -> Result<SyncResponse, SyncError> {
        let profile = require!(response.profile);
        let ciphers = require!(response.ciphers);

        fn try_into_iter<In, InItem, Out, OutItem>(iter: In) -> Result<Out, InItem::Error>
        where
            In: IntoIterator<Item = InItem>,
            InItem: TryInto<OutItem>,
            Out: FromIterator<OutItem>,
        {
            iter.into_iter().map(|i| i.try_into()).collect()
        }

        Ok(SyncResponse {
            profile: ProfileResponse::process_response(*profile, ctx)?,
            folders: try_into_iter(require!(response.folders))?,
            collections: try_into_iter(require!(response.collections))?,
            ciphers: try_into_iter(ciphers)?,
            domains: response.domains.map(|d| (*d).try_into()).transpose()?,
            //policies: try_into_iter(require!(response.policies))?,
            //sends: try_into_iter(require!(response.sends))?,
        })
    }
}

impl ProfileOrganizationResponse {
    fn process_response(
        response: ProfileOrganizationResponseModel,
    ) -> Result<ProfileOrganizationResponse, Error> {
        Ok(ProfileOrganizationResponse {
            id: require!(response.id),
        })
    }
}

impl ProfileResponse {
    fn process_response(
        response: ProfileResponseModel,
        _ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
    ) -> Result<ProfileResponse, Error> {
        Ok(ProfileResponse {
            id: require!(response.id),
            name: require!(response.name),
            email: require!(response.email),
            //key: response.key,
            //private_key: response.private_key,
            organizations: response
                .organizations
                .unwrap_or_default()
                .into_iter()
                .map(ProfileOrganizationResponse::process_response)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<DomainsResponseModel> for DomainResponse {
    type Error = SyncError;

    fn try_from(value: DomainsResponseModel) -> Result<Self, Self::Error> {
        Ok(Self {
            equivalent_domains: value.equivalent_domains.unwrap_or_default(),
            global_equivalent_domains: value
                .global_equivalent_domains
                .unwrap_or_default()
                .into_iter()
                .map(|s| s.try_into())
                .collect::<Result<Vec<GlobalDomains>, _>>()?,
        })
    }
}
