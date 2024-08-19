use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherDetailsResponseModel {
    pub object: Option<String>,
    pub id: Option<uuid::Uuid>,
    pub organization_id: Option<uuid::Uuid>,
    pub r#type: Option<CipherType>,
    pub name: Option<String>,
    pub notes: Option<String>,
    pub login: Option<CipherLoginModel>,
    pub card: Option<Box<CipherCardModel>>,
    pub identity: Option<Box<CipherIdentityModel>>,
    pub secure_note: Option<Box<CipherSecureNoteModel>>,
    pub fields: Option<Vec<CipherFieldModel>>,
    pub password_history: Option<Vec<CipherPasswordHistoryModel>>,
    pub attachments: Option<Vec<AttachmentResponseModel>>,
    pub organization_use_totp: Option<bool>,
    pub revision_date: Option<String>,
    pub creation_date: Option<String>,
    pub deleted_date: Option<String>,
    pub reprompt: Option<CipherRepromptType>,
    pub key: Option<String>,
    pub folder_id: Option<uuid::Uuid>,
    pub favorite: Option<bool>,
    pub edit: Option<bool>,
    pub view_password: Option<bool>,
    pub collection_ids: Option<Vec<uuid::Uuid>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum CipherType {
    #[default]
    Login = 1,
    SecureNote = 2,
    Card = 3,
    Identity = 4,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherLoginModel {
    pub uri: Option<String>,
    pub uris: Option<Vec<CipherLoginUriModel>>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub password_revision_date: Option<String>,
    pub totp: Option<String>,
    pub autofill_on_page_load: Option<bool>,
    pub fido2_credentials: Option<Vec<CipherFido2CredentialModel>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherLoginUriModel {
    pub uri: Option<String>,
    pub uri_checksum: Option<String>,
    pub r#match: Option<UriMatchType>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum UriMatchType {
    #[default]
    Domain = 0,
    Host = 1,
    StartsWith = 2,
    Exact = 3,
    RegularExpression = 4,
    Never = 5,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherFido2CredentialModel {
    pub credential_id: Option<String>,
    pub key_type: Option<String>,
    pub key_algorithm: Option<String>,
    pub key_curve: Option<String>,
    pub key_value: Option<String>,
    pub rp_id: Option<String>,
    pub rp_name: Option<String>,
    pub user_handle: Option<String>,
    pub user_name: Option<String>,
    pub user_display_name: Option<String>,
    pub counter: Option<String>,
    pub discoverable: Option<String>,
    pub creation_date: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherCardModel {
    pub cardholder_name: Option<String>,
    pub brand: Option<String>,
    pub number: Option<String>,
    pub exp_month: Option<String>,
    pub exp_year: Option<String>,
    pub code: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherIdentityModel {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "address1", skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    #[serde(rename = "address2", skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    #[serde(rename = "address3", skip_serializing_if = "Option::is_none")]
    pub address3: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "ssn", skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "passportNumber", skip_serializing_if = "Option::is_none")]
    pub passport_number: Option<String>,
    #[serde(rename = "licenseNumber", skip_serializing_if = "Option::is_none")]
    pub license_number: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherSecureNoteModel {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SecureNoteType>,
}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum SecureNoteType {
    #[default]
    Generic = 0,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherFieldModel {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FieldType>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "linkedId", skip_serializing_if = "Option::is_none")]
    pub linked_id: Option<i32>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    #[default]
    Text = 0,
    Hidden = 1,
    Boolean = 2,
    Linked = 3,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherPasswordHistoryModel {
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "lastUsedDate")]
    pub last_used_date: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "sizeName", skip_serializing_if = "Option::is_none")]
    pub size_name: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum CipherRepromptType {
    #[default]
    None = 0,
    Password = 1,
}
