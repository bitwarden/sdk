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
pub struct BillingHistoryResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "invoices", skip_serializing_if = "Option::is_none")]
    pub invoices: Option<Vec<crate::models::BillingInvoice>>,
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<crate::models::BillingTransaction>>,
}

impl BillingHistoryResponseModel {
    pub fn new() -> BillingHistoryResponseModel {
        BillingHistoryResponseModel {
            object: None,
            invoices: None,
            transactions: None,
        }
    }
}
