pub struct CipherDataV2 {
    pub notes: Option<String>,
    pub login: Option<CipherLoginDataV2>,
    pub card: Option<CipherCardDataV2>,
    pub identity: Option<CipherIdentityDataV2>,
    pub secure_note: Option<CipherSecureNodeDataV2>,
    pub fields: Option<CipherFieldDataV2>,
    pub password_history: Option<Vec<CipherPasswordHistoryDataV2>>,
    pub attachments: Option<Vec<CipherAttachmentDataV2>>,
    // pub organization_use_totp: Option<bool>,
    pub revision_date: Option<String>,
    pub creation_date: Option<String>,
    pub deleted_date: Option<String>,
    pub reprompt: Option<CipherRepromptTypeDataV2>,
    pub key: Option<String>,
    // pub folder_id: Option<uuid::Uuid>,
    pub favorite: Option<bool>,
    pub edit: Option<bool>,
    // pub view_password: Option<bool>,
}

pub struct CipherLoginDataV2 {
    pub uri: Option<String>,
    pub uris: Option<Vec<CipherLoginUriDataV2>>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub password_revision_date: Option<String>,
    pub totp: Option<String>,
    pub autofill_on_page_load: Option<bool>,
    pub fido2_credentials: Option<Vec<CipherFido2CredentialDataV2>>,
}

pub struct CipherLoginUriDataV2 {
    pub uri: Option<String>,
    pub uri_checksum: Option<String>,
    pub r#match: Option<UriMatchTypeDataV2>,
}

pub enum UriMatchTypeDataV2 {
    Domain = 0,
    Host = 1,
    StartsWith = 2,
    Exact = 3,
    RegularExpression = 4,
    Never = 5,
}

pub struct CipherFido2CredentialDataV2 {
    pub credential_id_type: Option<CredentialIdTypeDataV2>,
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

pub enum CredentialIdTypeDataV2 {
    Uuid = 0,
    Base64 = 1,
}

pub struct CipherCardDataV2 {
    pub cardholder_name: Option<String>,
    pub brand: Option<String>,
    pub number: Option<String>,
    pub exp_month: Option<String>,
    pub exp_year: Option<String>,
    pub code: Option<String>,
}

pub struct CipherIdentityDataV2 {
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

pub struct CipherSecureNodeDataV2 {
    pub r#type: Option<SecureNoteTypeDataV2>,
}

pub enum SecureNoteTypeDataV2 {
    Generic = 0,
}

pub struct CipherFieldDataV2 {
    pub r#type: Option<FieldTypeDataV2>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub linked_id: Option<i32>,
}

pub enum FieldTypeDataV2 {
    Text = 0,
    Hidden = 1,
    Boolean = 2,
    Linked = 3,
}

pub struct CipherPasswordHistoryDataV2 {
    pub password: String,
    pub last_used_date: String,
}

pub struct CipherAttachmentDataV2 {
    pub object: Option<String>,
    pub id: Option<String>,
    pub url: Option<String>,
    pub file_name: Option<String>,
    pub key: Option<String>,
    pub size: Option<String>,
    pub size_name: Option<String>,
}

pub enum CipherRepromptTypeDataV2 {
    None = 0,
    Password = 1,
}
