use crate::{
    client::Client,
    error::{Error, Result},
    sdk::request::sync_request::SyncRequest,
};

#[allow(dead_code)]
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
