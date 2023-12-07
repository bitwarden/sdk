use std::collections::HashMap;

use bitwarden_crypto::EncString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    client::kdf::Kdf,
    error::{Error, Result},
    Client,
};

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct InitUserCryptoRequest {
    /// The user's KDF parameters, as received from the prelogin request
    pub kdf_params: Kdf,
    /// The user's email address
    pub email: String,
    /// The user's encrypted private key
    pub private_key: String,
    /// The initialization method to use
    pub method: InitUserCryptoMethod,
}

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum InitUserCryptoMethod {
    Password {
        /// The user's master password
        password: String,
        /// The user's encrypted symmetric crypto key
        user_key: String,
    },
    DecryptedKey {
        /// The user's decrypted encryption key, obtained using `get_user_encryption_key`
        decrypted_user_key: String,
    },
}

#[cfg(feature = "internal")]
pub async fn initialize_user_crypto(client: &mut Client, req: InitUserCryptoRequest) -> Result<()> {
    let login_method = crate::client::LoginMethod::User(crate::client::UserLoginMethod::Username {
        client_id: "".to_string(),
        email: req.email,
        kdf: req.kdf_params,
    });
    client.set_login_method(login_method);

    let private_key: EncString = req.private_key.parse()?;

    match req.method {
        InitUserCryptoMethod::Password { password, user_key } => {
            let user_key: EncString = user_key.parse()?;
            client.initialize_user_crypto(&password, user_key, private_key)?;
        }
        InitUserCryptoMethod::DecryptedKey { decrypted_user_key } => {
            client.initialize_user_crypto_decrypted_key(&decrypted_user_key, private_key)?;
        }
    }

    Ok(())
}

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct InitOrgCryptoRequest {
    /// The encryption keys for all the organizations the user is a part of
    pub organization_keys: HashMap<uuid::Uuid, EncString>,
}

#[cfg(feature = "internal")]
pub async fn initialize_org_crypto(client: &mut Client, req: InitOrgCryptoRequest) -> Result<()> {
    let organization_keys = req.organization_keys.into_iter().collect();
    client.initialize_org_crypto(organization_keys)?;
    Ok(())
}

#[cfg(feature = "internal")]
pub async fn get_user_encryption_key(client: &mut Client) -> Result<String> {
    let user_key = client
        .get_encryption_settings()?
        .get_key(&None)
        .ok_or(Error::VaultLocked)?;

    Ok(user_key.to_base64())
}
