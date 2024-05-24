/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSponsorshipRedeemRequestModel {
    #[serde(rename = "planSponsorshipType")]
    pub plan_sponsorship_type: models::PlanSponsorshipType,
    #[serde(rename = "sponsoredOrganizationId")]
    pub sponsored_organization_id: uuid::Uuid,
}

impl OrganizationSponsorshipRedeemRequestModel {
    pub fn new(
        plan_sponsorship_type: models::PlanSponsorshipType,
        sponsored_organization_id: uuid::Uuid,
    ) -> OrganizationSponsorshipRedeemRequestModel {
        OrganizationSponsorshipRedeemRequestModel {
            plan_sponsorship_type,
            sponsored_organization_id,
        }
    }
}
