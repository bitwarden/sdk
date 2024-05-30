use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{require, Error, Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct GlobalDomains {
    pub r#type: i32,
    pub domains: Vec<String>,
    pub excluded: bool,
}

impl TryFrom<bitwarden_api_api::models::GlobalDomains> for GlobalDomains {
    type Error = Error;

    fn try_from(global_domains: bitwarden_api_api::models::GlobalDomains) -> Result<Self> {
        Ok(Self {
            r#type: require!(global_domains.r#type),
            domains: require!(global_domains.domains),
            excluded: require!(global_domains.excluded),
        })
    }
}
