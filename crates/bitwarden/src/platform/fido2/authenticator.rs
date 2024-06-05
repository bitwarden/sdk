#![allow(dead_code, unused_mut, unused_imports, unused_variables)]

use bitwarden_crypto::{EncString, KeyEncryptable};
use chrono::DateTime;
use passkey::{
    authenticator::{Authenticator, UserCheck},
    types::{
        ctap2::{make_credential::Request, Aaguid, Ctap2Error, StatusCode},
        Passkey,
    },
};
use uuid::Uuid;

use super::{CredentialStore, SelectedCredential, UserInterface};
use crate::{
    error::{Error, Result},
    vault::{
        login::{Fido2CredentialView, LoginView},
        CipherView, Fido2Credential,
    },
    Client,
};

pub struct Fido2Authenticator<'a> {
    pub(crate) client: &'a mut Client,
    pub(crate) user_interface: &'a dyn UserInterface,
    pub(crate) credential_store: &'a dyn CredentialStore,
}

impl<'a> Fido2Authenticator<'a> {
    pub async fn make_credential(
        &mut self,
        request: MakeCredentialRequest,
    ) -> Result<MakeCredentialResult> {
        // TODO: Placeholder value
        let my_aaguid = Aaguid::new_empty();

        let mut authenticator = Authenticator::new(
            my_aaguid,
            self.to_credential_store(),
            self.to_user_interface(),
        );

        /*let response = authenticator
            .make_credential(Request {
                client_data_hash: request.client_data_hash.into(),
                rp: passkey::types::ctap2::make_credential::PublicKeyCredentialRpEntity {
                    id: request.rp.id,
                    name: request.rp.name,
                },
                user: passkey::types::webauthn::PublicKeyCredentialUserEntity {
                    id: request.user.id.into(),
                    display_name: request.user.display_name,
                    name: request.user.name,
                },
                pub_key_cred_params: request
                    .pub_key_cred_params
                    .into_iter()
                    .map(
                        |x| passkey::types::webauthn::PublicKeyCredentialParameters {
                            ty: todo!(),
                            alg: todo!(),
                        },
                    )
                    .collect(),
                exclude_list: request
                    .exclude_list
                    .map(|x: Vec<PublicKeyCredentialDescriptor>| {
                        x.into_iter()
                            .map(
                                |x| passkey::types::webauthn::PublicKeyCredentialDescriptor {
                                    ty: todo!(),
                                    id: todo!(),
                                    transports: None,
                                },
                            )
                            .collect()
                    }),
                extensions: None, // TODO: request.extensions,
                options: passkey::types::ctap2::make_credential::Options {
                    rk: true,
                    up: true,
                    uv: true,
                },
                pin_auth: None,
                pin_protocol: None,
            })
            .await;

        let response = match response {
            Ok(x) => x,
            Err(e) => return Err(format!("make_credential error: {e:?}").into()),
        };

        Ok(MakeCredentialResult {
            credential_id: response
                .auth_data
                .attested_credential_data
                .expect("Missing attested_credential_data")
                .credential_id()
                .to_vec(),
        })*/

        Ok(MakeCredentialResult {
            credential_id: vec![],
        })
    }

    pub async fn get_assertion(
        &mut self,
        request: GetAssertionRequest,
    ) -> Result<GetAssertionResult> {
        let enc = self.client.get_encryption_settings()?;
        let key = enc.get_key(&None).ok_or(Error::VaultLocked)?;

        Ok(GetAssertionResult {
            credential_id: vec![],
            authenticator_data: vec![],
            signature: vec![],
            user_handle: vec![],
            selected_credential: SelectedCredential {
                cipher: CipherView {
                    id: Some(Uuid::new_v4()),
                    organization_id: None,
                    folder_id: None,
                    collection_ids: vec![],
                    key: None,
                    name: "".to_string(),
                    notes: Some("".to_string()),
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

    // TODO: Fido2CredentialView contains all the fields, maybe we need a Fido2CredentialListView?
    pub async fn silently_discover_credentials(
        &mut self,
        rp_id: String,
    ) -> Result<Vec<Fido2CredentialView>> {
        Ok(vec![])
    }

    pub(crate) fn to_user_interface(&'a self) -> UserInterfaceImpl<'_> {
        UserInterfaceImpl {
            authenticator: self,
        }
    }
    pub(crate) fn to_credential_store(&'a self) -> CredentialStoreImpl<'_> {
        CredentialStoreImpl {
            authenticator: self,
        }
    }
}

pub(crate) struct CredentialStoreImpl<'a> {
    authenticator: &'a Fido2Authenticator<'a>,
}
pub(crate) struct UserInterfaceImpl<'a> {
    authenticator: &'a Fido2Authenticator<'a>,
}

#[async_trait::async_trait]
impl passkey::authenticator::CredentialStore for CredentialStoreImpl<'_> {
    type PasskeyItem = CipherView;

    async fn find_credentials(
        &self,
        ids: Option<&[passkey::types::webauthn::PublicKeyCredentialDescriptor]>,
        rp_id: &str,
    ) -> Result<Vec<Self::PasskeyItem>, StatusCode> {
        Ok(vec![])
    }

    async fn save_credential(
        &mut self,
        cred: Passkey,
        user: passkey::types::ctap2::make_credential::PublicKeyCredentialUserEntity,
        rp: passkey::types::ctap2::make_credential::PublicKeyCredentialRpEntity,
    ) -> Result<(), StatusCode> {
        Ok(())
    }

    async fn update_credential(&mut self, cred: Passkey) -> Result<(), StatusCode> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl passkey::authenticator::UserValidationMethod for UserInterfaceImpl<'_> {
    type PasskeyItem = CipherView;

    async fn check_user(
        &self,
        credential: Option<Self::PasskeyItem>,
        presence: bool,
        verification: bool,
    ) -> Result<UserCheck, Ctap2Error> {
        Ok(UserCheck {
            presence,
            verification,
        })
    }

    fn is_presence_enabled(&self) -> bool {
        true
    }

    fn is_verification_enabled(&self) -> Option<bool> {
        Some(true)
    }
}

// What type do we need this to be? We probably can't use Serialize over the FFI boundary
pub type Extensions = Option<std::collections::HashMap<String, String>>;

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PublicKeyCredentialRpEntity {
    pub id: String,
    pub name: Option<String>,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PublicKeyCredentialUserEntity {
    pub id: Vec<u8>,
    pub display_name: String,
    pub name: String,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PublicKeyCredentialParameters {
    pub ty: String,
    pub alg: i64,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PublicKeyCredentialDescriptor {
    pub ty: i64,
    pub id: Vec<u8>,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct MakeCredentialRequest {
    client_data_hash: Vec<u8>,
    rp: PublicKeyCredentialRpEntity,
    user: PublicKeyCredentialUserEntity,
    pub_key_cred_params: Vec<PublicKeyCredentialParameters>,
    exclude_list: Option<Vec<PublicKeyCredentialDescriptor>>,
    require_resident_key: bool,
    extensions: Extensions,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct MakeCredentialResult {
    // TODO
    // authenticator_data: Vec<u8>,
    // attested_credential_data: Vec<u8>,
    credential_id: Vec<u8>,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct GetAssertionRequest {
    rp_id: String,
    client_data_hash: Vec<u8>,
    allow_list: Option<Vec<PublicKeyCredentialDescriptor>>,
    options: Options,
    extensions: Extensions,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Options {
    rk: bool,
    uv: UV,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum UV {
    Discouraged,
    Preferred,
    Required,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct GetAssertionResult {
    credential_id: Vec<u8>,
    authenticator_data: Vec<u8>,
    signature: Vec<u8>,
    user_handle: Vec<u8>,
    /**
     * SDK IMPL NOTE: This is not part of the spec and is not returned by passkey-rs.
     * The SDK needs to add this after the response from passkey-rs is received.
     */
    selected_credential: SelectedCredential,
}
