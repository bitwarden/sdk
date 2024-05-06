#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

use bitwarden_crypto::{KeyEncryptable, SensitiveString};
use passkey::{authenticator::Authenticator, types::ctap2::Aaguid};
use reqwest::Url;
use serde::Serialize;
use uuid::Uuid;

use super::{Fido2Authenticator, SelectedCredential};
use crate::{
    error::{Error, Result},
    vault::{login::LoginView, CipherView, Fido2Credential},
};

pub struct Fido2Client<'a> {
    pub(crate) authenticator: Fido2Authenticator<'a>,
}

impl<'a> Fido2Client<'a> {
    pub async fn register(
        &mut self,
        origin: String,
        request: String,
        client_data: ClientData,
    ) -> Result<PublicKeyCredentialAuthenticatorAttestationResponse> {
        // TODO: Placeholder value
        let my_aaguid = Aaguid::new_empty();

        let authenticator = Authenticator::new(
            my_aaguid,
            self.authenticator.to_credential_store(),
            self.authenticator.to_user_interface(),
        );
        let mut client = passkey::client::Client::new(authenticator);

        let origin = Url::parse(&origin).expect("Invalid URL");

        let result = client
            .register(&origin, serde_json::from_str(&request)?, client_data)
            .await?;

        /*Ok(PublicKeyCredentialAuthenticatorAttestationResponse {
            id: result.id,
            raw_id: result.raw_id.into(),
            ty: "public-key".to_string(),
            authenticator_attachment: todo!(),
            client_extension_results: todo!(),
            response: AuthenticatorAttestationResponse {
                client_data_json: result.response.client_data_json.into(),
                authenticator_data: result.response.authenticator_data.into(),
                public_key: result.response.public_key.map(|x| x.into()),
                public_key_algorithm: result.response.public_key_algorithm,
                attestation_object: result.response.attestation_object.into(),
                transports: todo!(),
            },
            selected_credential: SelectedCredential {
                cipher: todo!(),
                credential: todo!(),
            },
        })*/
        let enc = self.authenticator.client.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(Error::VaultLocked)?;

        Ok(PublicKeyCredentialAuthenticatorAttestationResponse {
            id: String::new(),
            raw_id: vec![],
            ty: "public-key".to_string(),
            authenticator_attachment: String::new(),
            client_extension_results: HashMap::new(),
            response: AuthenticatorAttestationResponse {
                client_data_json: vec![],
                authenticator_data: vec![],
                public_key: None,
                public_key_algorithm: 0,
                attestation_object: vec![],
                transports: None,
            },
            selected_credential: SelectedCredential {
                cipher: CipherView {
                    id: Some(Uuid::new_v4()),
                    organization_id: None,
                    folder_id: None,
                    collection_ids: vec![],
                    key: None,
                    name: SensitiveString::new("".to_string()),
                    notes: Some(SensitiveString::new("".to_string())),
                    r#type: crate::vault::CipherType::Login,
                    login: Some(LoginView {
                        username: None,
                        password: None,
                        password_revision_date: None,
                        uris: None,
                        totp: None,
                        autofill_on_page_load: None,
                        fido2_credentials: Some(vec![]),
                    }),
                    identity: None,
                    card: None,
                    secure_note: None,
                    favorite: false,
                    reprompt: crate::vault::CipherRepromptType::None,
                    organization_use_totp: true,
                    edit: true,
                    view_password: true,
                    local_data: None,
                    attachments: Some(vec![]),
                    fields: Some(vec![]),
                    password_history: Some(vec![]),
                    creation_date: chrono::offset::Utc::now(),
                    deleted_date: None,
                    revision_date: chrono::offset::Utc::now(),
                },
                credential: Fido2Credential {
                    credential_id: "".to_owned().encrypt_with_key(key)?,
                    key_type: "".to_owned().encrypt_with_key(key)?,
                    key_algorithm: "".to_owned().encrypt_with_key(key)?,
                    key_curve: "".to_owned().encrypt_with_key(key)?,
                    key_value: "".to_owned().encrypt_with_key(key)?,
                    rp_id: "".to_owned().encrypt_with_key(key)?,
                    user_handle: Some("".to_owned().encrypt_with_key(key)?),
                    user_name: Some("".to_owned().encrypt_with_key(key)?),
                    counter: "".to_owned().encrypt_with_key(key)?,
                    rp_name: Some("".to_owned().encrypt_with_key(key)?),
                    user_display_name: Some("".to_owned().encrypt_with_key(key)?),
                    discoverable: "".to_owned().encrypt_with_key(key)?,
                    creation_date: chrono::offset::Utc::now(),
                },
            },
        })
    }
    pub async fn authenticate(
        &mut self,
        origin: String,
        request: String,
        client_data: ClientData,
    ) -> Result<PublicKeyCredentialAuthenticatorAssertionResponse> {
        // TODO: Placeholder value
        let my_aaguid = Aaguid::new_empty();

        let authenticator = Authenticator::new(
            my_aaguid,
            self.authenticator.to_credential_store(),
            self.authenticator.to_user_interface(),
        );
        let mut client = passkey::client::Client::new(authenticator);

        let origin = Url::parse(&origin).expect("Invalid URL");

        let result = client
            .authenticate(&origin, serde_json::from_str(&request)?, client_data)
            .await?;

        /*Ok(PublicKeyCredentialAuthenticatorAssertionResponse {
            id: result.id,
            raw_id: result.raw_id.into(),
            ty: "public-key".to_string(),
            authenticator_attachment: todo!(),
            client_extension_results: todo!(),
            response: AuthenticatorAssertionResponse {
                client_data_json: result.response.client_data_json.into(),
                authenticator_data: result.response.authenticator_data.into(),
                signature: result.response.signature.into(),
                user_handle: result.response.user_handle.unwrap_or_default().into(),
            },
            selected_credential: SelectedCredential {
                cipher: todo!(),
                credential: todo!(),
            },
        })*/

        let enc = self.authenticator.client.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(Error::VaultLocked)?;

        Ok(PublicKeyCredentialAuthenticatorAssertionResponse {
            id: String::new(),
            raw_id: vec![],
            ty: "public-key".to_string(),
            authenticator_attachment: String::new(),
            client_extension_results: HashMap::new(),
            response: AuthenticatorAssertionResponse {
                client_data_json: vec![],
                authenticator_data: vec![],
                signature: vec![],
                user_handle: vec![],
            },
            selected_credential: SelectedCredential {
                cipher: CipherView {
                    id: Some(Uuid::new_v4()),
                    organization_id: None,
                    folder_id: None,
                    collection_ids: vec![],
                    key: None,
                    name: SensitiveString::new("".to_string()),
                    notes: Some(SensitiveString::new("".to_string())),
                    r#type: crate::vault::CipherType::Login,
                    login: Some(LoginView {
                        username: None,
                        password: None,
                        password_revision_date: None,
                        uris: None,
                        totp: None,
                        autofill_on_page_load: None,
                        fido2_credentials: Some(vec![]),
                    }),
                    identity: None,
                    card: None,
                    secure_note: None,
                    favorite: false,
                    reprompt: crate::vault::CipherRepromptType::None,
                    organization_use_totp: true,
                    edit: true,
                    view_password: true,
                    local_data: None,
                    attachments: Some(vec![]),
                    fields: Some(vec![]),
                    password_history: Some(vec![]),
                    creation_date: chrono::offset::Utc::now(),
                    deleted_date: None,
                    revision_date: chrono::offset::Utc::now(),
                },
                credential: Fido2Credential {
                    credential_id: "".to_owned().encrypt_with_key(key)?,
                    key_type: "".to_owned().encrypt_with_key(key)?,
                    key_algorithm: "".to_owned().encrypt_with_key(key)?,
                    key_curve: "".to_owned().encrypt_with_key(key)?,
                    key_value: "".to_owned().encrypt_with_key(key)?,
                    rp_id: "".to_owned().encrypt_with_key(key)?,
                    user_handle: Some("".to_owned().encrypt_with_key(key)?),
                    user_name: Some("".to_owned().encrypt_with_key(key)?),
                    counter: "".to_owned().encrypt_with_key(key)?,
                    rp_name: Some("".to_owned().encrypt_with_key(key)?),
                    user_display_name: Some("".to_owned().encrypt_with_key(key)?),
                    discoverable: "".to_owned().encrypt_with_key(key)?,
                    creation_date: chrono::offset::Utc::now(),
                },
            },
        })
    }
}

#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum ClientData {
    DefaultWithExtraData { android_package_name: String },
    DefaultWithCustomHash { hash: Vec<u8> },
}

#[derive(Serialize, Clone)]
struct AndroidClientData {
    android_package_name: String,
}

// TODO: I'm implementing this to convert from a basic enum into the generic
// passkey::client::ClientData Not fully sure that it's correct to return None for extra_client_data
// instead of ()
impl passkey::client::ClientData<Option<AndroidClientData>> for ClientData {
    fn extra_client_data(&self) -> Option<AndroidClientData> {
        match self {
            ClientData::DefaultWithExtraData {
                android_package_name,
            } => Some(AndroidClientData {
                android_package_name: android_package_name.clone(),
            }),
            ClientData::DefaultWithCustomHash { .. } => None,
        }
    }

    fn client_data_hash(&self) -> Option<Vec<u8>> {
        match self {
            ClientData::DefaultWithExtraData { .. } => None,
            ClientData::DefaultWithCustomHash { hash } => Some(hash.clone()),
        }
    }
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PublicKeyCredentialAuthenticatorAttestationResponse {
    id: String,
    raw_id: Vec<u8>,
    ty: String,
    authenticator_attachment: String,
    client_extension_results: HashMap<String, bool>,
    response: AuthenticatorAttestationResponse,
    selected_credential: SelectedCredential,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct AuthenticatorAttestationResponse {
    client_data_json: Vec<u8>,
    authenticator_data: Vec<u8>,
    public_key: Option<Vec<u8>>,
    public_key_algorithm: i64,
    attestation_object: Vec<u8>,
    transports: Option<Vec<String>>,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PublicKeyCredentialAuthenticatorAssertionResponse {
    id: String,
    raw_id: Vec<u8>,
    ty: String,
    authenticator_attachment: String,
    client_extension_results: HashMap<String, bool>,
    response: AuthenticatorAssertionResponse,
    selected_credential: SelectedCredential,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct AuthenticatorAssertionResponse {
    client_data_json: Vec<u8>,
    authenticator_data: Vec<u8>,
    signature: Vec<u8>,
    user_handle: Vec<u8>,
}
