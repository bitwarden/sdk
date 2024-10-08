use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use bitwarden_core::key_management::{AsymmetricKeyRef, SymmetricKeyRef};
use bitwarden_crypto::service::CryptoServiceContext;
use bitwarden_vault::{
    CipherError, CipherView, Fido2CredentialFullView, Fido2CredentialNewView, Fido2CredentialView,
};
use crypto::{CoseKeyToPkcs8Error, PrivateKeyFromSecretKeyError};
use passkey::types::{ctap2::Aaguid, Passkey};

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
#[cfg(feature = "uniffi")]
mod uniffi_support;

mod authenticator;
mod client;
mod client_fido;
mod crypto;
mod traits;
mod types;
pub use authenticator::{
    CredentialsForAutofillError, Fido2Authenticator, GetAssertionError, MakeCredentialError,
    SilentlyDiscoverCredentialsError,
};
pub use client::{Fido2Client, Fido2ClientError};
pub use client_fido::{ClientFido2, ClientFido2Ext, DecryptFido2AutofillCredentialsError};
pub use passkey::authenticator::UIHint;
use thiserror::Error;
pub use traits::{
    CheckUserOptions, CheckUserResult, Fido2CallbackError, Fido2CredentialStore,
    Fido2UserInterface, Verification,
};
pub use types::{
    AuthenticatorAssertionResponse, AuthenticatorAttestationResponse, ClientData,
    Fido2CredentialAutofillView, Fido2CredentialAutofillViewError, GetAssertionRequest,
    GetAssertionResult, MakeCredentialRequest, MakeCredentialResult, Options, Origin,
    PublicKeyCredentialAuthenticatorAssertionResponse,
    PublicKeyCredentialAuthenticatorAttestationResponse, PublicKeyCredentialRpEntity,
    PublicKeyCredentialUserEntity, UnverifiedAssetLink,
};

use self::crypto::{cose_key_to_pkcs8, pkcs8_to_cose_key};

// This is the AAGUID for the Bitwarden Passkey provider (d548826e-79b4-db40-a3d8-11116f7e8349)
// It is used for the Relaying Parties to identify the authenticator during registration
const AAGUID: Aaguid = Aaguid([
    0xd5, 0x48, 0x82, 0x6e, 0x79, 0xb4, 0xdb, 0x40, 0xa3, 0xd8, 0x11, 0x11, 0x6f, 0x7e, 0x83, 0x49,
]);

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
    fn new(
        cipher: CipherView,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
    ) -> Result<Self, CipherError> {
        let fido2_credentials = cipher.get_fido2_credentials(ctx)?;
        Ok(Self {
            cipher,
            fido2_credentials,
        })
    }
}

#[derive(Debug, Error)]
pub enum Fido2Error {
    #[error(transparent)]
    DecodeError(#[from] base64::DecodeError),

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
    let key_value = URL_SAFE_NO_PAD.decode(value.key_value)?;
    let user_handle = value
        .user_handle
        .map(|u| URL_SAFE_NO_PAD.decode(u))
        .transpose()?;

    let key = pkcs8_to_cose_key(&key_value)?;

    Ok(Passkey {
        key,
        credential_id: string_to_guid_bytes(&value.credential_id)?.into(),
        rp_id: value.rp_id.clone(),
        user_handle: user_handle.map(|u| u.into()),
        counter,
    })
}

#[derive(Debug, Error)]
pub enum FillCredentialError {
    #[error(transparent)]
    InvalidInputLength(#[from] InvalidInputLength),
    #[error(transparent)]
    CoseKeyToPkcs8Error(#[from] CoseKeyToPkcs8Error),
}

pub fn fill_with_credential(
    view: &Fido2CredentialView,
    value: Passkey,
) -> Result<Fido2CredentialFullView, FillCredentialError> {
    let cred_id: Vec<u8> = value.credential_id.into();
    let user_handle = value
        .user_handle
        .map(|u| URL_SAFE_NO_PAD.encode(u.to_vec()));
    let key_value = URL_SAFE_NO_PAD.encode(cose_key_to_pkcs8(&value.key)?);

    Ok(Fido2CredentialFullView {
        credential_id: guid_bytes_to_string(&cred_id)?,
        key_type: "public-key".to_owned(),
        key_algorithm: "ECDSA".to_owned(),
        key_curve: "P-256".to_owned(),
        key_value,
        rp_id: value.rp_id,
        rp_name: view.rp_name.clone(),
        user_handle,

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
) -> Result<Fido2CredentialNewView, InvalidInputLength> {
    let cred_id: Vec<u8> = vec![0; 16];
    let user_handle = URL_SAFE_NO_PAD.encode(user.id.to_vec());

    Ok(Fido2CredentialNewView {
        // TODO: Why do we have a credential id here?
        credential_id: guid_bytes_to_string(&cred_id)?,
        key_type: "public-key".to_owned(),
        key_algorithm: "ECDSA".to_owned(),
        key_curve: "P-256".to_owned(),
        rp_id: rp.id.clone(),
        rp_name: rp.name.clone(),
        user_handle: Some(user_handle),

        counter: 0.to_string(),
        user_name: user.name.clone(),
        user_display_name: user.display_name.clone(),
        creation_date: chrono::offset::Utc::now(),
    })
}

pub(crate) fn try_from_credential_full(
    value: Passkey,
    user: passkey::types::ctap2::make_credential::PublicKeyCredentialUserEntity,
    rp: passkey::types::ctap2::make_credential::PublicKeyCredentialRpEntity,
    options: passkey::types::ctap2::get_assertion::Options,
) -> Result<Fido2CredentialFullView, FillCredentialError> {
    let cred_id: Vec<u8> = value.credential_id.into();
    let key_value = URL_SAFE_NO_PAD.encode(cose_key_to_pkcs8(&value.key)?);
    let user_handle = URL_SAFE_NO_PAD.encode(user.id.to_vec());

    Ok(Fido2CredentialFullView {
        credential_id: guid_bytes_to_string(&cred_id)?,
        key_type: "public-key".to_owned(),
        key_algorithm: "ECDSA".to_owned(),
        key_curve: "P-256".to_owned(),
        key_value,
        rp_id: value.rp_id,
        rp_name: rp.name,
        user_handle: Some(user_handle),

        counter: value.counter.unwrap_or(0).to_string(),
        user_name: user.name,
        user_display_name: user.display_name,
        discoverable: options.rk.to_string(),
        creation_date: chrono::offset::Utc::now(),
    })
}

#[derive(Debug, Error)]
#[error("Input should be a 16 byte array")]
pub struct InvalidInputLength;

pub fn guid_bytes_to_string(source: &[u8]) -> Result<String, InvalidInputLength> {
    if source.len() != 16 {
        return Err(InvalidInputLength);
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
