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
pub struct PasswordManagerPlanFeaturesResponseModel {
    #[serde(rename = "stripePlanId", skip_serializing_if = "Option::is_none")]
    pub stripe_plan_id: Option<String>,
    #[serde(rename = "stripeSeatPlanId", skip_serializing_if = "Option::is_none")]
    pub stripe_seat_plan_id: Option<String>,
    #[serde(
        rename = "stripeProviderPortalSeatPlanId",
        skip_serializing_if = "Option::is_none"
    )]
    pub stripe_provider_portal_seat_plan_id: Option<String>,
    #[serde(rename = "basePrice", skip_serializing_if = "Option::is_none")]
    pub base_price: Option<f64>,
    #[serde(rename = "seatPrice", skip_serializing_if = "Option::is_none")]
    pub seat_price: Option<f64>,
    #[serde(
        rename = "providerPortalSeatPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_portal_seat_price: Option<f64>,
    #[serde(rename = "allowSeatAutoscale", skip_serializing_if = "Option::is_none")]
    pub allow_seat_autoscale: Option<bool>,
    #[serde(
        rename = "hasAdditionalSeatsOption",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_additional_seats_option: Option<bool>,
    #[serde(rename = "maxAdditionalSeats", skip_serializing_if = "Option::is_none")]
    pub max_additional_seats: Option<i32>,
    #[serde(rename = "baseSeats", skip_serializing_if = "Option::is_none")]
    pub base_seats: Option<i32>,
    #[serde(
        rename = "hasPremiumAccessOption",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_premium_access_option: Option<bool>,
    #[serde(
        rename = "stripePremiumAccessPlanId",
        skip_serializing_if = "Option::is_none"
    )]
    pub stripe_premium_access_plan_id: Option<String>,
    #[serde(
        rename = "premiumAccessOptionPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub premium_access_option_price: Option<f64>,
    #[serde(rename = "maxSeats", skip_serializing_if = "Option::is_none")]
    pub max_seats: Option<i32>,
    #[serde(rename = "baseStorageGb", skip_serializing_if = "Option::is_none")]
    pub base_storage_gb: Option<i32>,
    #[serde(
        rename = "hasAdditionalStorageOption",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_additional_storage_option: Option<bool>,
    #[serde(
        rename = "additionalStoragePricePerGb",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_storage_price_per_gb: Option<f64>,
    #[serde(
        rename = "stripeStoragePlanId",
        skip_serializing_if = "Option::is_none"
    )]
    pub stripe_storage_plan_id: Option<String>,
    #[serde(
        rename = "maxAdditionalStorage",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_additional_storage: Option<i32>,
    #[serde(rename = "maxCollections", skip_serializing_if = "Option::is_none")]
    pub max_collections: Option<i32>,
}

impl PasswordManagerPlanFeaturesResponseModel {
    pub fn new() -> PasswordManagerPlanFeaturesResponseModel {
        PasswordManagerPlanFeaturesResponseModel {
            stripe_plan_id: None,
            stripe_seat_plan_id: None,
            stripe_provider_portal_seat_plan_id: None,
            base_price: None,
            seat_price: None,
            provider_portal_seat_price: None,
            allow_seat_autoscale: None,
            has_additional_seats_option: None,
            max_additional_seats: None,
            base_seats: None,
            has_premium_access_option: None,
            stripe_premium_access_plan_id: None,
            premium_access_option_price: None,
            max_seats: None,
            base_storage_gb: None,
            has_additional_storage_option: None,
            additional_storage_price_per_gb: None,
            stripe_storage_plan_id: None,
            max_additional_storage: None,
            max_collections: None,
        }
    }
}
