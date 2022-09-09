use std::str::FromStr;

use crate::{
    client::Client,
    crypto::CipherString,
    error::{Error, Result},
    sdk::{request::sync_request::SyncRequest, response::sync_response::SyncResponse},
};

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
        .filter_map(|o| o.id.as_deref().zip(o.key.as_deref()))
        .map(|(id, key)| CipherString::from_str(key).map(|k| (id.to_owned(), k)))
        .collect::<Result<_, _>>()?;

    let enc = client.initialize_org_crypto(org_keys)?;

    SyncResponse::process_response(sync, enc)
}
