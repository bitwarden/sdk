use std::{collections::HashMap, num::NonZeroU32};

use bitwarden_api_identity::models::KdfType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IdentityTokenSuccessResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    token_type: String,

    #[serde(rename = "privateKey", alias = "PrivateKey")]
    pub(crate) private_key: Option<String>,
    #[serde(alias = "Key")]
    pub(crate) key: Option<String>,
    #[serde(rename = "twoFactorToken")]
    two_factor_token: Option<String>,
    #[serde(alias = "Kdf")]
    kdf: KdfType,
    #[serde(
        rename = "kdfIterations",
        alias = "KdfIterations",
        default = "crate::util::default_pbkdf2_iterations"
    )]
    kdf_iterations: NonZeroU32,

    #[serde(rename = "resetMasterPassword", alias = "ResetMasterPassword")]
    pub reset_master_password: bool,
    #[serde(rename = "forcePasswordReset", alias = "ForcePasswordReset")]
    pub force_password_reset: bool,
    #[serde(rename = "apiUseKeyConnector", alias = "ApiUseKeyConnector")]
    api_use_key_connector: Option<bool>,
    #[serde(rename = "keyConnectorUrl", alias = "KeyConnectorUrl")]
    key_connector_url: Option<String>,

    /// Stores unknown api response fields
    extra: Option<HashMap<String, Value>>,
}

#[cfg(test)]
mod test {
    use super::*;

    impl Default for IdentityTokenSuccessResponse {
        fn default() -> Self {
            Self {
                access_token: Default::default(),
                expires_in: Default::default(),
                refresh_token: Default::default(),
                token_type: Default::default(),
                private_key: Default::default(),
                key: Default::default(),
                two_factor_token: Default::default(),
                kdf: KdfType::Variant0,
                kdf_iterations: crate::util::default_pbkdf2_iterations(),
                reset_master_password: Default::default(),
                force_password_reset: Default::default(),
                api_use_key_connector: Default::default(),
                key_connector_url: Default::default(),
                extra: Default::default(),
            }
        }
    }
}
