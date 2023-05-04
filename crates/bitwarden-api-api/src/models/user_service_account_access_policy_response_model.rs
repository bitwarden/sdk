/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserServiceAccountAccessPolicyResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(rename = "write", skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "revisionDate", skip_serializing_if = "Option::is_none")]
    pub revision_date: Option<String>,
    #[serde(rename = "organizationUserId", skip_serializing_if = "Option::is_none")]
    pub organization_user_id: Option<uuid::Uuid>,
    #[serde(
        rename = "organizationUserName",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_user_name: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[serde(
        rename = "grantedServiceAccountId",
        skip_serializing_if = "Option::is_none"
    )]
    pub granted_service_account_id: Option<uuid::Uuid>,
}

impl UserServiceAccountAccessPolicyResponseModel {
    pub fn new() -> UserServiceAccountAccessPolicyResponseModel {
        UserServiceAccountAccessPolicyResponseModel {
            object: None,
            id: None,
            read: None,
            write: None,
            creation_date: None,
            revision_date: None,
            organization_user_id: None,
            organization_user_name: None,
            user_id: None,
            granted_service_account_id: None,
        }
    }
}
