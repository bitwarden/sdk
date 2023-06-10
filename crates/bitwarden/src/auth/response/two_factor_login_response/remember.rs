use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Remember {}

impl From<crate::api::response::two_factor_provider_data::remember::Remember> for Remember {
    fn from(_: crate::api::response::two_factor_provider_data::remember::Remember) -> Self {
        Self {}
    }
}
