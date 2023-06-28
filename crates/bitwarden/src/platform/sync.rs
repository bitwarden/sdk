use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    error::{Error, Result},
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

    client.state.set_account_sync_data(account_id, sync).await?;
    client.initialize_org_crypto().await?;

    Ok(())
}
