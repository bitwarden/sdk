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
pub struct SecretResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "organizationId")]
    pub organization_id: uuid::Uuid,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "creationDate")]
    pub creation_date: String,
    #[serde(rename = "revisionDate")]
    pub revision_date: String,
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<models::SecretResponseInnerProject>>,
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(rename = "write", skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
}

impl SecretResponseModel {
    pub fn new(
        id: uuid::Uuid,
        organization_id: uuid::Uuid,
        creation_date: String,
        revision_date: String,
    ) -> SecretResponseModel {
        SecretResponseModel {
            object: None,
            id,
            organization_id,
            key: None,
            value: None,
            note: None,
            creation_date,
            revision_date,
            projects: None,
            read: None,
            write: None,
        }
    }
}
