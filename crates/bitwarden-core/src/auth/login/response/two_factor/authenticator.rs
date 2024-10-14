use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Authenticator {}

impl From<crate::auth::api::response::two_factor_provider_data::authenticator::Authenticator>
    for Authenticator
{
    fn from(
        _: crate::auth::api::response::two_factor_provider_data::authenticator::Authenticator,
    ) -> Self {
        Self {}
    }
}
