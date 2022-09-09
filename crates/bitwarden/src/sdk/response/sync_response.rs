use bitwarden_api_api::models::{
    CipherDetailsResponseModel, ProfileOrganizationResponseModel, ProfileResponseModel,
    SyncResponseModel,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    client::encryption_settings::EncryptionSettings,
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProfileResponse {
    pub id: String,
    pub name: String,
    pub email: String,

    //key: String,
    //private_key: String,
    pub organizations: Vec<ProfileOrganizationResponse>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProfileOrganizationResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherDetailsResponse {}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SyncResponse {
    /// Data about the user, including their encryption keys and the organizations they are a part of
    pub profile: ProfileResponse,
    /// List of ciphers accesible by the user
    pub ciphers: Vec<CipherDetailsResponse>,
}

impl SyncResponse {
    pub fn process_response(
        response: SyncResponseModel,
        enc: &EncryptionSettings,
    ) -> Result<SyncResponse> {
        let profile = *response.profile.ok_or(Error::MissingFields)?;
        let ciphers = response.ciphers.ok_or(Error::MissingFields)?;

        Ok(SyncResponse {
            profile: ProfileResponse::process_response(profile, enc)?,
            ciphers: ciphers
                .into_iter()
                .map(CipherDetailsResponse::process_response)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl CipherDetailsResponse {
    fn process_response(_response: CipherDetailsResponseModel) -> Result<CipherDetailsResponse> {
        Ok(CipherDetailsResponse {})
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
