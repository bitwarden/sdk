use bitwarden_api_api::models::{
    DomainsResponseModel, ProfileOrganizationResponseModel, ProfileResponseModel, SyncResponseModel,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::domain::GlobalDomains;
use crate::{
    admin_console::Policy,
    client::{encryption_settings::EncryptionSettings, Client},
    error::{Error, Result},
    vault::{Cipher, Collection, Folder},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SyncRequest {
    /// Exclude the subdomains from the response, defaults to false
    pub exclude_subdomains: Option<bool>,
}

pub(crate) async fn sync(client: &mut Client, input: &SyncRequest) -> Result<SyncResponse> {
    let config = client.get_api_configurations().await;
    let sync =
        bitwarden_api_api::apis::sync_api::sync_get(&config.api, input.exclude_subdomains).await?;

    let org_keys: Vec<_> = sync
        .profile
        .as_ref()
        .ok_or(Error::MissingFields)?
        .organizations
        .as_deref()
        .unwrap_or_default()
        .iter()
        .filter_map(|o| o.id.zip(o.key.as_deref().and_then(|k| k.parse().ok())))
        .collect();

    let enc = client.initialize_org_crypto(org_keys)?;

    SyncResponse::process_response(sync, enc)
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
    pub policies: Vec<Policy>,
    pub sends: Vec<crate::vault::Send>,
}

impl SyncResponse {
    pub(crate) fn process_response(
        response: SyncResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<SyncResponse> {
        let profile = *response.profile.ok_or(Error::MissingFields)?;
        let ciphers = response.ciphers.ok_or(Error::MissingFields)?;

        fn try_into_iter<In, InItem, Out, OutItem>(iter: In) -> Result<Out, InItem::Error>
        where
            In: IntoIterator<Item = InItem>,
            InItem: TryInto<OutItem>,
            Out: FromIterator<OutItem>,
        {
            iter.into_iter().map(|i| i.try_into()).collect()
        }

        Ok(SyncResponse {
            profile: ProfileResponse::process_response(profile, enc)?,
            folders: try_into_iter(response.folders.ok_or(Error::MissingFields)?)?,
            collections: try_into_iter(response.collections.ok_or(Error::MissingFields)?)?,
            ciphers: try_into_iter(ciphers)?,
            domains: response.domains.map(|d| (*d).try_into()).transpose()?,
            policies: try_into_iter(response.policies.ok_or(Error::MissingFields)?)?,
            sends: try_into_iter(response.sends.ok_or(Error::MissingFields)?)?,
        })
    }
}

impl ProfileOrganizationResponse {
    fn process_response(
        response: ProfileOrganizationResponseModel,
    ) -> Result<ProfileOrganizationResponse> {
        Ok(ProfileOrganizationResponse {
            id: response.id.ok_or(Error::MissingFields)?,
        })
    }
}

impl ProfileResponse {
    fn process_response(
        response: ProfileResponseModel,
        _enc: &EncryptionSettings,
    ) -> Result<ProfileResponse> {
        Ok(ProfileResponse {
            id: response.id.ok_or(Error::MissingFields)?,
            name: response.name.ok_or(Error::MissingFields)?,
            email: response.email.ok_or(Error::MissingFields)?,
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
    type Error = Error;
    fn try_from(value: DomainsResponseModel) -> Result<Self> {
        Ok(Self {
            equivalent_domains: value.equivalent_domains.unwrap_or_default(),
            global_equivalent_domains: value
                .global_equivalent_domains
                .unwrap_or_default()
                .into_iter()
                .map(|s| s.try_into())
                .collect::<Result<Vec<GlobalDomains>>>()?,
        })
    }
}
