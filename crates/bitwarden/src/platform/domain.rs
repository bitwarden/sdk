use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

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
            r#type: global_domains.r#type.ok_or(Error::MissingFields)?,
            domains: global_domains.domains.ok_or(Error::MissingFields)?,
            excluded: global_domains.excluded.ok_or(Error::MissingFields)?,
        })
    }
}
