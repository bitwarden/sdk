use bitwarden_core::require;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::VaultParseError;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct GlobalDomains {
    pub r#type: i32,
    pub domains: Vec<String>,
    pub excluded: bool,
}

impl TryFrom<bitwarden_api_api::models::GlobalDomains> for GlobalDomains {
    type Error = VaultParseError;

    fn try_from(
        global_domains: bitwarden_api_api::models::GlobalDomains,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            r#type: require!(global_domains.r#type),
            domains: require!(global_domains.domains),
            excluded: require!(global_domains.excluded),
        })
    }
}
