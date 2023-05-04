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
pub struct CipherMiniResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "organizationId", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<uuid::Uuid>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::CipherType>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<Box<crate::models::CipherLoginModel>>,
    #[serde(rename = "card", skip_serializing_if = "Option::is_none")]
    pub card: Option<Box<crate::models::CipherCardModel>>,
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<Box<crate::models::CipherIdentityModel>>,
    #[serde(rename = "secureNote", skip_serializing_if = "Option::is_none")]
    pub secure_note: Option<Box<crate::models::CipherSecureNoteModel>>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::CipherFieldModel>>,
    #[serde(rename = "passwordHistory", skip_serializing_if = "Option::is_none")]
    pub password_history: Option<Vec<crate::models::CipherPasswordHistoryModel>>,
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<crate::models::AttachmentResponseModel>>,
    #[serde(
        rename = "organizationUseTotp",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_use_totp: Option<bool>,
    #[serde(rename = "revisionDate", skip_serializing_if = "Option::is_none")]
    pub revision_date: Option<String>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "deletedDate", skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<String>,
    #[serde(rename = "reprompt", skip_serializing_if = "Option::is_none")]
    pub reprompt: Option<crate::models::CipherRepromptType>,
}

impl CipherMiniResponseModel {
    pub fn new() -> CipherMiniResponseModel {
        CipherMiniResponseModel {
            object: None,
            id: None,
            organization_id: None,
            r#type: None,
            data: None,
            name: None,
            notes: None,
            login: None,
            card: None,
            identity: None,
            secure_note: None,
            fields: None,
            password_history: None,
            attachments: None,
            organization_use_totp: None,
            revision_date: None,
            creation_date: None,
            deleted_date: None,
            reprompt: None,
        }
    }
}
