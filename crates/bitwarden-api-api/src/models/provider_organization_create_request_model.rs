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
pub struct ProviderOrganizationCreateRequestModel {
    #[serde(rename = "clientOwnerEmail")]
    pub client_owner_email: String,
    #[serde(rename = "organizationCreateRequest")]
    pub organization_create_request: Box<crate::models::OrganizationCreateRequestModel>,
}

impl ProviderOrganizationCreateRequestModel {
    pub fn new(
        client_owner_email: String,
        organization_create_request: crate::models::OrganizationCreateRequestModel,
    ) -> ProviderOrganizationCreateRequestModel {
        ProviderOrganizationCreateRequestModel {
            client_owner_email,
            organization_create_request: Box::new(organization_create_request),
        }
    }
}
