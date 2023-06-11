use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct YubiKey {
    /// Whether the stored yubikey supports near field communication
    pub nfc: bool,
}

impl From<crate::auth::api::response::two_factor_provider_data::yubi_key::YubiKey> for YubiKey {
    fn from(api: crate::auth::api::response::two_factor_provider_data::yubi_key::YubiKey) -> Self {
        Self { nfc: api.nfc }
    }
}
