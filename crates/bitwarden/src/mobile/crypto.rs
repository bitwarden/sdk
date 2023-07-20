use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{crypto::CipherString, error::Result, Client};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InitUserCryptoRequest {
    /// The user's master password
    pub password: String,
    /// The user's encrypted symmetric crypto key
    pub user_key: String,
    /// The user's encryptred private key
    pub private_key: String,
}

pub async fn initialize_user_crypto(client: &mut Client, req: InitUserCryptoRequest) -> Result<()> {
    let user_key = req.user_key.parse::<CipherString>()?;
    let private_key = req.private_key.parse::<CipherString>()?;

    client.initialize_user_crypto(&req.password, user_key, private_key)?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InitOrgCryptoRequest {
    /// The encryption keys for all the organizations the user is a part of
    pub organization_keys: HashMap<uuid::Uuid, String>,
}

pub async fn initialize_org_crypto(client: &mut Client, req: InitOrgCryptoRequest) -> Result<()> {
    let organization_keys = req
        .organization_keys
        .into_iter()
        .map(|(k, v)| Ok((k, v.parse::<CipherString>()?)))
        .collect::<Result<Vec<_>>>()?;

    client.initialize_org_crypto(organization_keys)?;

    Ok(())
}
