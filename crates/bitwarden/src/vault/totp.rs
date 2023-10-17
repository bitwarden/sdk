use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct TotpResponse {
    pub code: String,
    pub interval: u32,
}

pub async fn generate_totp(_key: String) -> TotpResponse {
    TotpResponse {
        code: "000 000".to_string(),
        interval: 30,
    }
}
