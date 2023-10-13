use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct GlobalDomains {
    pub r#type: i32,
    pub domains: Vec<String>,
    pub excluded: bool,
}

impl From<bitwarden_api_api::models::GlobalDomains> for GlobalDomains {
    fn from(global_domains: bitwarden_api_api::models::GlobalDomains) -> Self {
        GlobalDomains {
            r#type: 0,
            domains: global_domains.domains.unwrap(),
            excluded: global_domains.excluded.unwrap(),
        }
    }
}
