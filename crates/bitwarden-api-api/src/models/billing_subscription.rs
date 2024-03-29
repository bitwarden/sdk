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
pub struct BillingSubscription {
    #[serde(rename = "trialStartDate", skip_serializing_if = "Option::is_none")]
    pub trial_start_date: Option<String>,
    #[serde(rename = "trialEndDate", skip_serializing_if = "Option::is_none")]
    pub trial_end_date: Option<String>,
    #[serde(rename = "periodStartDate", skip_serializing_if = "Option::is_none")]
    pub period_start_date: Option<String>,
    #[serde(rename = "periodEndDate", skip_serializing_if = "Option::is_none")]
    pub period_end_date: Option<String>,
    #[serde(rename = "cancelledDate", skip_serializing_if = "Option::is_none")]
    pub cancelled_date: Option<String>,
    #[serde(rename = "cancelAtEndDate", skip_serializing_if = "Option::is_none")]
    pub cancel_at_end_date: Option<bool>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "cancelled", skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<bool>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::BillingSubscriptionItem>>,
}

impl BillingSubscription {
    pub fn new() -> BillingSubscription {
        BillingSubscription {
            trial_start_date: None,
            trial_end_date: None,
            period_start_date: None,
            period_end_date: None,
            cancelled_date: None,
            cancel_at_end_date: None,
            status: None,
            cancelled: None,
            items: None,
        }
    }
}
