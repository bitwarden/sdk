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
pub struct AccessPoliciesCreateRequest {
    #[serde(
        rename = "userAccessPolicyRequests",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_access_policy_requests: Option<Vec<crate::models::AccessPolicyRequest>>,
    #[serde(
        rename = "groupAccessPolicyRequests",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_access_policy_requests: Option<Vec<crate::models::AccessPolicyRequest>>,
    #[serde(
        rename = "serviceAccountAccessPolicyRequests",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_access_policy_requests: Option<Vec<crate::models::AccessPolicyRequest>>,
}

impl AccessPoliciesCreateRequest {
    pub fn new() -> AccessPoliciesCreateRequest {
        AccessPoliciesCreateRequest {
            user_access_policy_requests: None,
            group_access_policy_requests: None,
            service_account_access_policy_requests: None,
        }
    }
}
