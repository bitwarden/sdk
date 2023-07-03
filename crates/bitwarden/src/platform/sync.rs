use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    client::{keys::Keys, Client},
    error::{Error, Result},
    state::state,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SyncRequest {
    /// Exclude the subdomains from the response, defaults to false
    pub exclude_subdomains: Option<bool>,
}

pub(crate) async fn sync(client: &mut Client, input: &SyncRequest) -> Result<()> {
    let config = client.get_api_configurations().await;
    let sync =
        bitwarden_api_api::apis::sync_api::sync_get(&config.api, input.exclude_subdomains).await?;

    let profile = sync.profile.as_ref().ok_or(Error::MissingFields)?;
    let account_id = profile.id.ok_or(Error::MissingFields)?;

    state::set_account_sync_data(client, account_id, sync).await?;

    let keys = Keys::get(client).await.ok_or(Error::VaultLocked)?;
    client
        .initialize_org_crypto(&keys.organization_keys)
        .await?;

    Ok(())
}
