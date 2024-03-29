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
pub struct BillingResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    #[serde(rename = "paymentSource", skip_serializing_if = "Option::is_none")]
    pub payment_source: Option<Box<crate::models::BillingSource>>,
    #[serde(rename = "invoices", skip_serializing_if = "Option::is_none")]
    pub invoices: Option<Vec<crate::models::BillingInvoice>>,
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<crate::models::BillingTransaction>>,
}

impl BillingResponseModel {
    pub fn new() -> BillingResponseModel {
        BillingResponseModel {
            object: None,
            balance: None,
            payment_source: None,
            invoices: None,
            transactions: None,
        }
    }
}
