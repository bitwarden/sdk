use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::Result, state::state_service::FOLDERS_SERVICE, Client};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderDeleteRequest {
    /// ID of the folder to delete
    pub id: Uuid,
}

pub(crate) async fn delete_folder(client: &mut Client, input: FolderDeleteRequest) -> Result<()> {
    let config: &crate::client::ApiConfigurations = client.get_api_configurations().await;
    bitwarden_api_api::apis::folders_api::folders_id_delete(&config.api, &input.id.to_string())
        .await?;

    client
        .get_state_service(FOLDERS_SERVICE)
        .modify(move |folders| {
            folders.remove(&input.id);
            Ok(())
        })
        .await?;
    Ok(())
}
