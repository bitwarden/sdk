pub struct CipherDataV1 {
    pub notes: Option<String>,
    pub login: Option<CipherLoginDataV1>,
    pub card: Option<CipherCardDataV1>,
    pub identity: Option<CipherIdentityDataV1>,
    pub secure_note: Option<CipherSecureNodeDataV1>,
    pub fields: Option<CipherFieldDataV1>,
    pub password_history: Option<Vec<CipherPasswordHistoryDataV1>>,
    pub attachments: Option<Vec<CipherAttachmentDataV1>>,
    // pub organization_use_totp: Option<bool>,
    pub revision_date: Option<String>,
    pub creation_date: Option<String>,
    pub deleted_date: Option<String>,
    pub reprompt: Option<CipherRepromptTypeDataV1>,
    pub key: Option<String>,
    // pub folder_id: Option<uuid::Uuid>,
    pub favorite: Option<bool>,
    pub edit: Option<bool>,
    // pub view_password: Option<bool>,
}

pub struct CipherLoginDataV1 {
    pub uri: Option<String>,
    pub uris: Option<Vec<CipherLoginUriDataV1>>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub password_revision_date: Option<String>,
    pub totp: Option<String>,
    pub autofill_on_page_load: Option<bool>,
    pub fido2_credentials: Option<Vec<CipherFido2CredentialDataV1>>,
}

pub struct CipherLoginUriDataV1 {
    pub uri: Option<String>,
    pub uri_checksum: Option<String>,
    pub r#match: Option<UriMatchTypeDataV1>,
}

pub enum UriMatchTypeDataV1 {
    Domain = 0,
    Host = 1,
    StartsWith = 2,
    Exact = 3,
    RegularExpression = 4,
    Never = 5,
}

pub struct CipherFido2CredentialDataV1 {
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

pub struct CipherCardDataV1 {
    pub cardholder_name: Option<String>,
    pub brand: Option<String>,
    pub number: Option<String>,
    pub exp_month: Option<String>,
    pub exp_year: Option<String>,
    pub code: Option<String>,
}

pub struct CipherIdentityDataV1 {
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ssn: Option<String>,
    pub username: Option<String>,
    pub passport_number: Option<String>,
    pub license_number: Option<String>,
}

pub struct CipherSecureNodeDataV1 {
    pub r#type: Option<SecureNoteTypeDataV1>,
}

pub enum SecureNoteTypeDataV1 {
    Generic = 0,
}

pub struct CipherFieldDataV1 {
    pub r#type: Option<FieldTypeDataV1>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub linked_id: Option<i32>,
}

pub enum FieldTypeDataV1 {
    Text = 0,
    Hidden = 1,
    Boolean = 2,
    Linked = 3,
}

pub struct CipherPasswordHistoryDataV1 {
    pub password: String,
    pub last_used_date: String,
}

pub struct CipherAttachmentDataV1 {
    pub object: Option<String>,
    pub id: Option<String>,
    pub url: Option<String>,
    pub file_name: Option<String>,
    pub key: Option<String>,
    pub size: Option<String>,
    pub size_name: Option<String>,
}

pub enum CipherRepromptTypeDataV1 {
    None = 0,
    Password = 1,
}
