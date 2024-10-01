use bitwarden_api_api::models::cipher_details_response_model::CipherDetailsMetaDataResponseModel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherDetailsResponseModel {
    pub object: Option<String>, // Not sure what this is?

    // MetaData structure is owned by the Server
    // This doesn't necessarily need to be in it's own struct
    pub meta_data: CipherDetailsMetaDataResponseModel,

    // Data structure is owned by the Client
    pub data: CipherDetailsData,
}

// #[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
// pub struct CipherDetailsMetaDataResponseModel {
//     pub id: Option<uuid::Uuid>,
//     pub organization_id: Option<uuid::Uuid>,
//     pub collection_ids: Option<Vec<uuid::Uuid>>,
//     pub folder_id: Option<uuid::Uuid>,
//     pub view_password: Option<bool>,
//     pub organization_use_totp: Option<bool>,
//     pub revision_date: Option<String>,
//     pub creation_date: Option<String>,
//     pub deleted_date: Option<String>,
// }

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherDetailsData {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CipherType>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<CipherLoginModel>,
    #[serde(rename = "card", skip_serializing_if = "Option::is_none")]
    pub card: Option<CipherCardModel>,
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<CipherIdentityModel>,
    #[serde(rename = "secureNote", skip_serializing_if = "Option::is_none")]
    pub secure_note: Option<CipherSecureNoteModel>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<CipherFieldModel>>,
    #[serde(rename = "passwordHistory", skip_serializing_if = "Option::is_none")]
    pub password_history: Option<Vec<CipherPasswordHistoryModel>>,
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentResponseModel>>,
    #[serde(rename = "reprompt", skip_serializing_if = "Option::is_none")]
    pub reprompt: Option<CipherRepromptType>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "favorite", skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
    #[serde(rename = "edit", skip_serializing_if = "Option::is_none")]
    pub edit: Option<bool>,
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
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "uris", skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<CipherLoginUriModel>>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(
        rename = "passwordRevisionDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub password_revision_date: Option<String>,
    #[serde(rename = "totp", skip_serializing_if = "Option::is_none")]
    pub totp: Option<String>,
    #[serde(rename = "autofillOnPageLoad", skip_serializing_if = "Option::is_none")]
    pub autofill_on_page_load: Option<bool>,
    #[serde(rename = "fido2Credentials", skip_serializing_if = "Option::is_none")]
    pub fido2_credentials: Option<Vec<CipherFido2CredentialModel>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherLoginUriModel {
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "uriChecksum", skip_serializing_if = "Option::is_none")]
    pub uri_checksum: Option<String>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "credentialId", skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
    #[serde(rename = "keyType", skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(rename = "keyAlgorithm", skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    #[serde(rename = "keyCurve", skip_serializing_if = "Option::is_none")]
    pub key_curve: Option<String>,
    #[serde(rename = "keyValue", skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
    #[serde(rename = "rpId", skip_serializing_if = "Option::is_none")]
    pub rp_id: Option<String>,
    #[serde(rename = "rpName", skip_serializing_if = "Option::is_none")]
    pub rp_name: Option<String>,
    #[serde(rename = "userHandle", skip_serializing_if = "Option::is_none")]
    pub user_handle: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "userDisplayName", skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
    #[serde(rename = "counter", skip_serializing_if = "Option::is_none")]
    pub counter: Option<String>,
    #[serde(rename = "discoverable", skip_serializing_if = "Option::is_none")]
    pub discoverable: Option<String>,
    #[serde(rename = "creationDate")]
    pub creation_date: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CipherCardModel {
    #[serde(rename = "cardholderName", skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    #[serde(rename = "brand", skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "expMonth", skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<String>,
    #[serde(rename = "expYear", skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
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
