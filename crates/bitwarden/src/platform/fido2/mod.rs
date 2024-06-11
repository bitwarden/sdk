use std::sync::Mutex;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use bitwarden_crypto::KeyContainer;
use bitwarden_vault::{
    CipherView, Fido2CredentialFullView, Fido2CredentialNewView, Fido2CredentialView,
};
use crypto::PrivateKeyFromSecretKeyError;
use passkey::types::{ctap2::Aaguid, Passkey};

mod authenticator;
mod client;
mod crypto;
mod traits;
mod types;

pub use authenticator::Fido2Authenticator;
pub use client::Fido2Client;
pub use passkey::authenticator::UIHint;
use thiserror::Error;
pub use traits::{
    CheckUserOptions, CheckUserResult, Fido2CallbackError, Fido2CredentialStore,
    Fido2UserInterface, Verification,
};
pub use types::{
    AuthenticatorAssertionResponse, AuthenticatorAttestationResponse, ClientData,
    GetAssertionRequest, GetAssertionResult, MakeCredentialRequest, MakeCredentialResult, Options,
    PublicKeyCredentialAuthenticatorAssertionResponse,
    PublicKeyCredentialAuthenticatorAttestationResponse, PublicKeyCredentialRpEntity,
    PublicKeyCredentialUserEntity,
};

use self::crypto::{cose_key_to_pkcs8, pkcs8_to_cose_key};
use crate::{
    error::{Error, Result},
    Client,
};

// This is the AAGUID for the Bitwarden Passkey provider (d548826e-79b4-db40-a3d8-11116f7e8349)
// It is used for the Relaying Parties to identify the authenticator during registration
const AAGUID: Aaguid = Aaguid([
    0xd5, 0x48, 0x82, 0x6e, 0x79, 0xb4, 0xdb, 0x40, 0xa3, 0xd8, 0x11, 0x11, 0x6f, 0x7e, 0x83, 0x49,
]);

pub struct ClientFido2<'a> {
    #[allow(dead_code)]
    pub(crate) client: &'a mut Client,
}

impl<'a> ClientFido2<'a> {
    pub fn create_authenticator(
        &'a mut self,

        user_interface: &'a dyn Fido2UserInterface,
        credential_store: &'a dyn Fido2CredentialStore,
    ) -> Result<Fido2Authenticator<'a>> {
        Ok(Fido2Authenticator {
            client: self.client,
            user_interface,
            credential_store,
            selected_cipher: Mutex::new(None),
            requested_uv: Mutex::new(None),
        })
    }

    pub fn create_client(
        &'a mut self,

        user_interface: &'a dyn Fido2UserInterface,
        credential_store: &'a dyn Fido2CredentialStore,
    ) -> Result<Fido2Client<'a>> {
        Ok(Fido2Client {
            authenticator: self.create_authenticator(user_interface, credential_store)?,
        })
    }
}

#[allow(dead_code)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct SelectedCredential {
    cipher: CipherView,
    credential: Fido2CredentialView,
}

// This container is needed so we can properly implement the TryFrom trait for Passkey
// Otherwise we need to decrypt the Fido2 credentials every time we create a CipherView
#[derive(Clone)]
pub(crate) struct CipherViewContainer {
    cipher: CipherView,
    fido2_credentials: Vec<Fido2CredentialFullView>,
}

impl CipherViewContainer {
    fn new(cipher: CipherView, enc: &dyn KeyContainer) -> Result<Self> {
        let fido2_credentials = cipher.get_fido2_credentials(enc)?;
        Ok(Self {
            cipher,
            fido2_credentials,
        })
    }
}

#[derive(Debug, Error)]
pub enum Fido2Error {
    #[error(transparent)]
    UnknownEnum(#[from] UnknownEnum),

    #[error(transparent)]
    InvalidGuid(#[from] InvalidGuid),

    #[error(transparent)]
    PrivateKeyFromSecretKeyError(#[from] PrivateKeyFromSecretKeyError),

    #[error("No Fido2 credentials found")]
    NoFido2CredentialsFound,
}

impl TryFrom<CipherViewContainer> for Passkey {
    type Error = Fido2Error;

    fn try_from(value: CipherViewContainer) -> Result<Self, Self::Error> {
        let cred = value
            .fido2_credentials
            .first()
            .ok_or(Fido2Error::NoFido2CredentialsFound)?;

        try_from_credential_full_view(cred.clone())
    }
}

fn try_from_credential_full_view(value: Fido2CredentialFullView) -> Result<Passkey, Fido2Error> {
    let counter: u32 = value.counter.parse().expect("Invalid counter");
    let counter = (counter != 0).then_some(counter);

    let key = pkcs8_to_cose_key(&value.key_value)?;

    Ok(Passkey {
        key,
        credential_id: string_to_guid_bytes(&value.credential_id)?.into(),
        rp_id: value.rp_id.clone(),
        user_handle: value.user_handle.map(|u| u.into()),
        counter,
    })
}

pub fn fill_with_credential(
    view: &Fido2CredentialView,
    value: Passkey,
) -> Result<Fido2CredentialFullView> {
    let cred_id: Vec<u8> = value.credential_id.into();

    Ok(Fido2CredentialFullView {
        credential_id: guid_bytes_to_string(&cred_id)?,
        key_type: "public-key".to_owned(),
        key_algorithm: "ECDSA".to_owned(),
        key_curve: "P-256".to_owned(),
        key_value: cose_key_to_pkcs8(&value.key).map_err(|e| e.to_string())?,
        rp_id: value.rp_id,
        rp_name: view.rp_name.clone(),
        user_handle: Some(cred_id),

        counter: value.counter.unwrap_or(0).to_string(),
        user_name: view.user_name.clone(),
        user_display_name: view.user_display_name.clone(),
        discoverable: "true".to_owned(),
        creation_date: chrono::offset::Utc::now(),
    })
}

pub(crate) fn try_from_credential_new_view(
    user: &passkey::types::ctap2::make_credential::PublicKeyCredentialUserEntity,
    rp: &passkey::types::ctap2::make_credential::PublicKeyCredentialRpEntity,
) -> Result<Fido2CredentialNewView> {
    let cred_id: Vec<u8> = vec![0; 16];

    Ok(Fido2CredentialNewView {
        credential_id: guid_bytes_to_string(&cred_id)?,
        key_type: "public-key".to_owned(),
        key_algorithm: "ECDSA".to_owned(),
        key_curve: "P-256".to_owned(),
        rp_id: rp.id.clone(),
        rp_name: rp.name.clone(),
        user_handle: Some(cred_id),

        counter: 0.to_string(),
        user_name: user.name.clone(),
        user_display_name: user.display_name.clone(),
        discoverable: "true".to_owned(),
        creation_date: chrono::offset::Utc::now(),
    })
}

pub(crate) fn try_from_credential_full(
    value: Passkey,
    user: passkey::types::ctap2::make_credential::PublicKeyCredentialUserEntity,
    rp: passkey::types::ctap2::make_credential::PublicKeyCredentialRpEntity,
) -> Result<Fido2CredentialFullView> {
    let cred_id: Vec<u8> = value.credential_id.into();

    Ok(Fido2CredentialFullView {
        credential_id: guid_bytes_to_string(&cred_id)?,
        key_type: "public-key".to_owned(),
        key_algorithm: "ECDSA".to_owned(),
        key_curve: "P-256".to_owned(),
        key_value: cose_key_to_pkcs8(&value.key).map_err(|e| e.to_string())?,
        rp_id: value.rp_id,
        rp_name: rp.name,
        user_handle: Some(cred_id),

        counter: value.counter.unwrap_or(0).to_string(),
        user_name: user.name,
        user_display_name: user.display_name,
        discoverable: "true".to_owned(),
        creation_date: chrono::offset::Utc::now(),
    })
}

pub fn guid_bytes_to_string(source: &[u8]) -> Result<String> {
    if source.len() != 16 {
        return Err(Error::Internal("Input should be a 16 byte array".into()));
    }
    Ok(uuid::Uuid::from_bytes(source.try_into().expect("Invalid length")).to_string())
}

#[derive(Debug, Error)]
#[error("Invalid GUID")]
pub struct InvalidGuid;

pub fn string_to_guid_bytes(source: &str) -> Result<Vec<u8>, InvalidGuid> {
    if source.starts_with("b64.") {
        let bytes = URL_SAFE_NO_PAD
            .decode(source.trim_start_matches("b64."))
            .map_err(|_| InvalidGuid)?;
        Ok(bytes)
    } else {
        let Ok(uuid) = uuid::Uuid::try_parse(source) else {
            return Err(InvalidGuid);
        };
        Ok(uuid.as_bytes().to_vec())
    }
}

#[derive(Debug, Error)]
#[error("Unknown enum value")]
pub struct UnknownEnum;

// Some utilities to convert back and forth between enums and strings
fn get_enum_from_string_name<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, UnknownEnum> {
    let serialized = format!(r#""{}""#, s);
    let deserialized: T = serde_json::from_str(&serialized).map_err(|_| UnknownEnum)?;
    Ok(deserialized)
}

fn get_string_name_from_enum(s: impl serde::Serialize) -> Result<String, serde_json::Error> {
    let serialized = serde_json::to_string(&s)?;
    let deserialized: String = serde_json::from_str(&serialized)?;
    Ok(deserialized)
}

#[cfg(test)]
mod tests {
    use passkey::types::webauthn::AuthenticatorAttachment;

    use super::{get_enum_from_string_name, get_string_name_from_enum};

    #[test]
    fn test_enum_string_conversion_works_as_expected() {
        assert_eq!(
            get_string_name_from_enum(AuthenticatorAttachment::CrossPlatform).unwrap(),
            "cross-platform"
        );

        assert_eq!(
            get_enum_from_string_name::<AuthenticatorAttachment>("cross-platform").unwrap(),
            AuthenticatorAttachment::CrossPlatform
        );
    }

    #[test]
    fn string_to_guid_with_uuid_works() {
        let uuid = "d548826e-79b4-db40-a3d8-11116f7e8349";
        let bytes = super::string_to_guid_bytes(uuid).unwrap();
        assert_eq!(
            bytes,
            vec![213, 72, 130, 110, 121, 180, 219, 64, 163, 216, 17, 17, 111, 126, 131, 73]
        );
    }

    #[test]
    fn string_to_guid_with_b64_works() {
        let b64 = "b64.1UiCbnm020Cj2BERb36DSQ";
        let bytes = super::string_to_guid_bytes(b64).unwrap();
        assert_eq!(
            bytes,
            vec![213, 72, 130, 110, 121, 180, 219, 64, 163, 216, 17, 17, 111, 126, 131, 73]
        );
    }
}
