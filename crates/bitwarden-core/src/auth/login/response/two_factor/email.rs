use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Email {
    /// The email to request a 2fa TOTP for
    pub email: String,
}

impl From<crate::auth::api::response::two_factor_provider_data::email::Email> for Email {
    fn from(api: crate::auth::api::response::two_factor_provider_data::email::Email) -> Self {
        Self { email: api.email }
    }
}
