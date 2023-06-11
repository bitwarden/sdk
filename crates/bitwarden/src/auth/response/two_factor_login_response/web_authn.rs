use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WebAuthn {}

impl From<crate::auth::api::response::two_factor_provider_data::web_authn::WebAuthn> for WebAuthn {
    fn from(_: crate::auth::api::response::two_factor_provider_data::web_authn::WebAuthn) -> Self {
        Self {}
    }
}
