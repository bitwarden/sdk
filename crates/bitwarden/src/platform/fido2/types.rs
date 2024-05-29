use serde::Serialize;

use super::{get_enum_from_string_name, SelectedCredential, Verification};

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

impl TryFrom<PublicKeyCredentialParameters>
    for passkey::types::webauthn::PublicKeyCredentialParameters
{
    type Error = crate::error::Error;

    fn try_from(value: PublicKeyCredentialParameters) -> Result<Self, Self::Error> {
        use coset::iana::EnumI64;
        Ok(Self {
            ty: get_enum_from_string_name(&value.ty)?,
            alg: coset::iana::Algorithm::from_i64(value.alg).ok_or("Invalid algorithm")?,
        })
    }
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PublicKeyCredentialDescriptor {
    pub ty: String,
    pub id: Vec<u8>,
}

impl TryFrom<PublicKeyCredentialDescriptor>
    for passkey::types::webauthn::PublicKeyCredentialDescriptor
{
    type Error = crate::error::Error;

    fn try_from(value: PublicKeyCredentialDescriptor) -> Result<Self, Self::Error> {
        Ok(Self {
            ty: get_enum_from_string_name(&value.ty)?,
            id: value.id.into(),
            // TODO(Fido2): Do we need to expose this?
            transports: None,
        })
    }
}

pub type Extensions = Option<String>;

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct MakeCredentialRequest {
    pub client_data_hash: Vec<u8>,
    pub rp: PublicKeyCredentialRpEntity,
    pub user: PublicKeyCredentialUserEntity,
    pub pub_key_cred_params: Vec<PublicKeyCredentialParameters>,
    pub exclude_list: Option<Vec<PublicKeyCredentialDescriptor>>,
    pub options: Options,
    pub extensions: Extensions,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct MakeCredentialResult {
    pub authenticator_data: Vec<u8>,
    pub attested_credential_data: Vec<u8>,
    pub credential_id: Vec<u8>,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct GetAssertionRequest {
    pub rp_id: String,
    pub client_data_hash: Vec<u8>,
    pub allow_list: Option<Vec<PublicKeyCredentialDescriptor>>,
    pub options: Options,
    pub extensions: Extensions,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Options {
    pub rk: bool,
    pub uv: UV,
}

impl From<super::CheckUserOptions> for Options {
    fn from(value: super::CheckUserOptions) -> Self {
        Self {
            rk: value.require_presence,
            uv: value.require_verification.into(),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum UV {
    Discouraged,
    Preferred,
    Required,
}

impl From<UV> for Verification {
    fn from(value: UV) -> Self {
        match value {
            UV::Discouraged => Verification::Discouraged,
            UV::Preferred => Verification::Preferred,
            UV::Required => Verification::Required,
        }
    }
}

impl From<Verification> for UV {
    fn from(value: Verification) -> Self {
        match value {
            Verification::Discouraged => UV::Discouraged,
            Verification::Preferred => UV::Preferred,
            Verification::Required => UV::Required,
        }
    }
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct GetAssertionResult {
    pub credential_id: Vec<u8>,
    pub authenticator_data: Vec<u8>,
    pub signature: Vec<u8>,
    pub user_handle: Vec<u8>,

    pub selected_credential: SelectedCredential,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum ClientData {
    DefaultWithExtraData { android_package_name: String },
    DefaultWithCustomHash { hash: Vec<u8> },
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct AndroidClientData {
    android_package_name: String,
}

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

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ClientExtensionResults {
    pub cred_props: Option<CredPropsResult>,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CredPropsResult {
    pub rk: Option<bool>,
    pub authenticator_display_name: Option<String>,
}

impl From<passkey::types::webauthn::CredentialPropertiesOutput> for CredPropsResult {
    fn from(value: passkey::types::webauthn::CredentialPropertiesOutput) -> Self {
        Self {
            rk: value.discoverable,
            authenticator_display_name: value.authenticator_display_name,
        }
    }
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PublicKeyCredentialAuthenticatorAttestationResponse {
    pub id: String,
    pub raw_id: Vec<u8>,
    pub ty: String,
    pub authenticator_attachment: Option<String>,
    pub client_extension_results: ClientExtensionResults,
    pub response: AuthenticatorAttestationResponse,
    pub selected_credential: SelectedCredential,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct AuthenticatorAttestationResponse {
    pub client_data_json: Vec<u8>,
    pub authenticator_data: Vec<u8>,
    pub public_key: Option<Vec<u8>>,
    pub public_key_algorithm: i64,
    pub attestation_object: Vec<u8>,
    pub transports: Option<Vec<String>>,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PublicKeyCredentialAuthenticatorAssertionResponse {
    pub id: String,
    pub raw_id: Vec<u8>,
    pub ty: String,
    pub authenticator_attachment: Option<String>,
    pub client_extension_results: ClientExtensionResults,
    pub response: AuthenticatorAssertionResponse,
    pub selected_credential: SelectedCredential,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct AuthenticatorAssertionResponse {
    pub client_data_json: Vec<u8>,
    pub authenticator_data: Vec<u8>,
    pub signature: Vec<u8>,
    pub user_handle: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::platform::fido2::types::AndroidClientData;

    // This is a stripped down of the passkey-rs implementation, to test the
    // serialization of the `ClientData` enum, and to make sure that () and None
    // are serialized the same way when going through #[serde(flatten)].
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CollectedClientData<E = ()>
    where
        E: Serialize,
    {
        pub origin: String,

        #[serde(flatten)]
        pub extra_data: E,
    }

    #[test]
    fn test_serialize_unit_data() {
        let data = CollectedClientData {
            origin: "https://example.com".to_owned(),
            extra_data: (),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#"{"origin":"https://example.com"}"#);
    }

    #[test]
    fn test_serialize_none_data() {
        let data = CollectedClientData {
            origin: "https://example.com".to_owned(),
            extra_data: Option::<AndroidClientData>::None,
        };

        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#"{"origin":"https://example.com"}"#);
    }

    #[test]
    fn test_serialize_android_data() {
        let data = CollectedClientData {
            origin: "https://example.com".to_owned(),
            extra_data: Some(AndroidClientData {
                android_package_name: "com.example.app".to_owned(),
            }),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(
            serialized,
            r#"{"origin":"https://example.com","androidPackageName":"com.example.app"}"#
        );
    }
}
