use std::sync::Mutex;

use bitwarden_crypto::{KeyContainer, SensitiveString, SensitiveVec};
use passkey::types::{ctap2::Aaguid, Passkey};

mod authenticator;
mod client;
mod crypto;
mod traits;
mod types;

pub use authenticator::Fido2Authenticator;
pub use client::Fido2Client;
pub use traits::{
    CheckUserOptions, CheckUserResult, Fido2CredentialStore, Fido2UserInterface, Verification,
};
pub use types::{
    AuthenticatorAssertionResponse, AuthenticatorAttestationResponse, ClientData,
    GetAssertionRequest, GetAssertionResult, MakeCredentialRequest, MakeCredentialResult,
    PublicKeyCredentialAuthenticatorAssertionResponse,
    PublicKeyCredentialAuthenticatorAttestationResponse,
};

use self::crypto::{cose_key_to_pkcs8, pkcs8_to_cose_key};
use crate::{
    error::{Error, Result},
    vault::{CipherView, Fido2CredentialFullView, Fido2CredentialView},
    Client,
};

// AAGUID: d548826e-79b4-db40-a3d8-11116f7e8349
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
            selected_credential: Mutex::new(None),
            selected_uv: Mutex::new(None),
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
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
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

impl TryFrom<CipherViewContainer> for Passkey {
    type Error = crate::error::Error;

    fn try_from(value: CipherViewContainer) -> Result<Self, Self::Error> {
        let cred = value
            .fido2_credentials
            .first()
            .ok_or(Error::Internal("No Fido2 credentials found".into()))?;

        cred.clone().try_into()
    }
}

impl TryFrom<Fido2CredentialFullView> for Passkey {
    type Error = crate::error::Error;

    fn try_from(value: Fido2CredentialFullView) -> Result<Self, Self::Error> {
        let counter: u32 = value.counter.expose().parse().expect("Invalid counter");
        let counter = (counter != 0).then_some(counter);

        let key = pkcs8_to_cose_key(value.key_value.expose().as_ref())?;

        Ok(Self {
            key,
            credential_id: string_to_guid_bytes(value.credential_id.expose())?.into(),
            rp_id: value.rp_id.expose().clone(),
            user_handle: value.user_handle.map(|u| u.expose().to_vec().into()),
            counter,
        })
    }
}

impl Fido2CredentialFullView {
    pub(crate) fn try_from_credential(
        value: Passkey,
        user: passkey::types::ctap2::make_credential::PublicKeyCredentialUserEntity,
        rp: passkey::types::ctap2::make_credential::PublicKeyCredentialRpEntity,
    ) -> Result<Self> {
        let cred_id: Vec<u8> = value.credential_id.into();

        Ok(Fido2CredentialFullView {
            credential_id: SensitiveString::new(Box::new(guid_bytes_to_string(&cred_id)?)),
            key_type: SensitiveString::new(Box::new("public-key".to_owned())),
            key_algorithm: SensitiveString::new(Box::new("ECDSA".to_owned())),
            key_curve: SensitiveString::new(Box::new("P-256".to_owned())),
            key_value: SensitiveVec::new(Box::new(cose_key_to_pkcs8(&value.key)?)),
            rp_id: SensitiveString::new(Box::new(value.rp_id)),
            rp_name: rp.name.map(|n| SensitiveString::new(Box::new(n))),
            user_handle: Some(SensitiveVec::new(Box::new(cred_id))),

            // TODO(Fido2): Storing the counter as a string seems like a bad idea, but we don't have
            // support for EncString -> number decryption
            counter: SensitiveString::new(Box::new(value.counter.unwrap_or(0).to_string())),
            user_name: user.name.map(|n| SensitiveString::new(Box::new(n))),
            user_display_name: user.display_name.map(|n| SensitiveString::new(Box::new(n))),
            // TODO(Fido2): Same as the counter, but with booleans this time
            discoverable: SensitiveString::new(Box::new("true".to_owned())),
            creation_date: chrono::offset::Utc::now(),
        })
    }
}

pub fn guid_bytes_to_string(source: &[u8]) -> Result<String> {
    if source.len() != 16 {
        return Err(Error::Internal("Input should be a 16 byte array".into()));
    }
    Ok(uuid::Uuid::from_bytes(source.try_into().expect("Invalid length")).to_string())
}

pub fn string_to_guid_bytes(source: &str) -> Result<Vec<u8>> {
    let Ok(uuid) = uuid::Uuid::parse_str(source) else {
        return Err(Error::Internal("Input should be a valid GUID".into()));
    };
    Ok(uuid.as_bytes().to_vec())
}

// Some utilities to convert back and forth between enums and strings
fn get_enum_from_string_name<T: serde::de::DeserializeOwned>(s: &str) -> Result<T> {
    let serialized = format!(r#""{}""#, s);
    let deserialized: T = serde_json::from_str(&serialized)?;
    Ok(deserialized)
}

fn get_string_name_from_enum(s: impl serde::Serialize) -> Result<String> {
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
}
