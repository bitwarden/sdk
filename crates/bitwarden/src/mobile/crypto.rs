use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    client::auth_settings::{AuthSettings, Kdf},
    crypto::CipherString,
    error::Result,
    Client,
};

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct InitCryptoRequest {
    /// The user's KDF parameters, as received from the prelogin request
    pub kdf_params: Kdf,
    /// The user's email address
    pub email: String,
    /// The user's master password
    pub password: String,
    /// The user's encrypted symmetric crypto key
    pub user_key: String,
    /// The user's encryptred private key
    pub private_key: String,
    /// The encryption keys for all the organizations the user is a part of
    pub organization_keys: HashMap<uuid::Uuid, String>,
}

#[cfg(feature = "internal")]
pub async fn initialize_crypto(client: &mut Client, req: InitCryptoRequest) -> Result<()> {
    let auth_settings = AuthSettings {
        email: req.email,
        kdf: req.kdf_params,
    };
    client.set_auth_settings(auth_settings);

    let user_key = req.user_key.parse::<CipherString>()?;
    let private_key = req.private_key.parse::<CipherString>()?;

    client.initialize_user_crypto(&req.password, user_key, private_key)?;

    let organization_keys = req
        .organization_keys
        .into_iter()
        .map(|(k, v)| Ok((k, v.parse::<CipherString>()?)))
        .collect::<Result<Vec<_>>>()?;

    client.initialize_org_crypto(organization_keys)?;

    Ok(())
}
