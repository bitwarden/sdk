use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{client::kdf::Kdf, crypto::EncString, error::Result, Client};

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
    let login_method = crate::client::LoginMethod::User(crate::client::UserLoginMethod::Username {
        client_id: "".to_string(),
        email: req.email,
        kdf: req.kdf_params,
    });
    client.set_login_method(login_method);

    let user_key = req.user_key.parse::<EncString>()?;
    let private_key = req.private_key.parse::<EncString>()?;

    client.initialize_user_crypto(&req.password, user_key, private_key)?;

    let organization_keys = req
        .organization_keys
        .into_iter()
        .map(|(k, v)| Ok((k, v.parse::<EncString>()?)))
        .collect::<Result<Vec<_>>>()?;

    client.initialize_org_crypto(organization_keys)?;

    Ok(())
}
