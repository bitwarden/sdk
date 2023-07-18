use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use bitwarden_api_api::models::ProfileResponseModel;

use crate::{
    error::{Error, Result},
    state::{state::State, state_service::ServiceDefinition},
    Client,
};

const PROFILE_SERVICE: ServiceDefinition<Option<Profile>> = ServiceDefinition::new("profile");

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Profile {
    pub(crate) user_id: Uuid,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) last_sync: DateTime<Utc>,
}

impl TryFrom<&ProfileResponseModel> for Profile {
    type Error = Error;

    fn try_from(value: &ProfileResponseModel) -> Result<Self> {
        Ok(Profile {
            user_id: value.id.ok_or(Error::MissingFields)?,
            name: value.name.clone().ok_or(Error::MissingFields)?,
            email: value.email.clone().ok_or(Error::MissingFields)?,
            last_sync: Utc::now(),
        })
    }
}

impl Profile {
    pub(crate) async fn get(client: &Client) -> Option<Profile> {
        client.get_state_service(PROFILE_SERVICE).get().await
    }
}

pub(crate) async fn store_profile_from_sync(
    profile: &ProfileResponseModel,
    state: &State,
) -> Result<()> {
    state
        .get_state_service(PROFILE_SERVICE)
        .modify(|k| {
            *k = Some(profile.try_into()?);
            Ok(())
        })
        .await
}
