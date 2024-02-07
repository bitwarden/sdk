/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalDomains {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    #[serde(rename = "excluded", skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,
}

impl GlobalDomains {
    pub fn new() -> GlobalDomains {
        GlobalDomains {
            r#type: None,
            domains: None,
            excluded: None,
        }
    }
}
