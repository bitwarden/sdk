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
pub struct SecretsManagerSubscribeRequestModel {
    #[serde(rename = "additionalSmSeats")]
    pub additional_sm_seats: i32,
    #[serde(rename = "additionalServiceAccounts")]
    pub additional_service_accounts: i32,
}

impl SecretsManagerSubscribeRequestModel {
    pub fn new(
        additional_sm_seats: i32,
        additional_service_accounts: i32,
    ) -> SecretsManagerSubscribeRequestModel {
        SecretsManagerSubscribeRequestModel {
            additional_sm_seats,
            additional_service_accounts,
        }
    }
}
