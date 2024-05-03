use bitwarden_crypto::SensitiveVec;
use serde::Serialize;

use super::{get_enum_from_string_name, SelectedCredential};

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PublicKeyCredentialRpEntity {
    pub id: String,
    pub name: Option<String>,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PublicKeyCredentialUserEntity {
    pub id: Vec<u8>,
    pub display_name: String,
    pub name: String,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
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

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
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

// TODO(Fido2): What type do we need this to be? We probably can't use Serialize over the FFI
// boundary
pub type Extensions = Option<String>;

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct MakeCredentialRequest {
    pub client_data_hash: Vec<u8>,
    pub rp: PublicKeyCredentialRpEntity,
    pub user: PublicKeyCredentialUserEntity,
    pub pub_key_cred_params: Vec<PublicKeyCredentialParameters>,
    pub exclude_list: Option<Vec<PublicKeyCredentialDescriptor>>,
    pub require_resident_key: bool,
    pub extensions: Extensions,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct MakeCredentialResult {
    pub authenticator_data: Vec<u8>,
    pub attested_credential_data: Vec<u8>,
    pub credential_id: Vec<u8>,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct GetAssertionRequest {
    pub rp_id: String,
    pub client_data_hash: Vec<u8>,
    pub allow_list: Option<Vec<PublicKeyCredentialDescriptor>>,
    pub options: Options,
    pub extensions: Extensions,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Options {
    pub rk: bool,
    pub uv: UV,
}

#[derive(Eq, PartialEq)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum UV {
    Discouraged,
    Preferred,
    Required,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct GetAssertionResult {
    pub credential_id: Vec<u8>,
    pub authenticator_data: Vec<u8>,
    pub signature: Vec<u8>,
    pub user_handle: Vec<u8>,
    /**
     * SDK IMPL NOTE: This is not part of the spec and is not returned by passkey-rs.
     * The SDK needs to add this after the response from passkey-rs is received.
     */
    pub selected_credential: SelectedCredential,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum ClientData {
    DefaultWithExtraData { android_package_name: String },
    DefaultWithCustomHash { hash: Vec<u8> },
}

// TODO(Fido2): I'm implementing this to convert into a generic passkey::client::ClientData.
// We need a custom implementation to  make sure the extra_client_data can serialize to () instead
// of None
#[derive(Clone)]
pub struct OptionalAndroidClientData {
    pub data: Option<AndroidClientData>,
}

impl Serialize for OptionalAndroidClientData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.data {
            Some(d) => d.serialize(serializer),
            None => serde::Serializer::serialize_unit(serializer),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct AndroidClientData {
    pub android_package_name: String,
}

impl passkey::client::ClientData<OptionalAndroidClientData> for ClientData {
    fn extra_client_data(&self) -> OptionalAndroidClientData {
        let data = match self {
            ClientData::DefaultWithExtraData {
                android_package_name,
            } => Some(AndroidClientData {
                android_package_name: android_package_name.clone(),
            }),
            ClientData::DefaultWithCustomHash { .. } => None,
        };

        OptionalAndroidClientData { data }
    }

    fn client_data_hash(&self) -> Option<Vec<u8>> {
        match self {
            ClientData::DefaultWithExtraData { .. } => None,
            ClientData::DefaultWithCustomHash { hash } => Some(hash.clone()),
        }
    }
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct ClientExtensionResults {
    pub cred_props: Option<CredPropsResult>,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
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

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PublicKeyCredentialAuthenticatorAttestationResponse {
    pub id: String,
    pub raw_id: Vec<u8>,
    pub ty: String,
    pub authenticator_attachment: Option<String>,
    pub client_extension_results: ClientExtensionResults,
    pub response: AuthenticatorAttestationResponse,
    pub selected_credential: SelectedCredential,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct AuthenticatorAttestationResponse {
    pub client_data_json: Vec<u8>,
    pub authenticator_data: Vec<u8>,
    pub public_key: Option<Vec<u8>>,
    pub public_key_algorithm: i64,
    pub attestation_object: Vec<u8>,
    pub transports: Option<Vec<String>>,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PublicKeyCredentialAuthenticatorAssertionResponse {
    pub id: String,
    pub raw_id: Vec<u8>,
    pub ty: String,
    pub authenticator_attachment: Option<String>,
    pub client_extension_results: ClientExtensionResults,
    pub response: AuthenticatorAssertionResponse,
    pub selected_credential: SelectedCredential,
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct AuthenticatorAssertionResponse {
    pub client_data_json: Vec<u8>,
    pub authenticator_data: Vec<u8>,
    pub signature: Vec<u8>,
    pub user_handle: Vec<u8>,
}

pub fn get_stub_selected_credential(
    key: &bitwarden_crypto::SymmetricCryptoKey,
) -> crate::error::Result<SelectedCredential> {
    use bitwarden_crypto::{KeyEncryptable, SensitiveString};

    Ok(SelectedCredential {
        cipher: crate::vault::CipherView {
            id: Some(uuid::Uuid::new_v4()),
            organization_id: None,
            folder_id: None,
            collection_ids: vec![],
            key: None,
            name: SensitiveString::new(Box::new("".to_string())),
            notes: Some(SensitiveString::new(Box::new("".to_string()))),
            r#type: crate::vault::CipherType::Login,
            login: Some(crate::vault::login::LoginView {
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
        credential: crate::vault::Fido2CredentialView {
            credential_id: SensitiveString::new(Box::new(
                "01234567-89ab-cdef-0123-456789abcdef".to_owned(),
            )),
            key_type: SensitiveString::new(Box::new("public-key".to_owned())),
            key_algorithm: SensitiveString::new(Box::new("ECDSA".to_owned())),
            key_curve: SensitiveString::new(Box::new("P-256".to_owned())),
            key_value: [].encrypt_with_key(key)?,
            rp_id: SensitiveString::new(Box::default()),
            rp_name: None,
            user_handle: Some(SensitiveVec::new(Box::default())),
            counter: SensitiveString::new(Box::new("0".to_owned())),
            user_name: None,
            user_display_name: None,
            discoverable: SensitiveString::new(Box::new("true".to_owned())),
            creation_date: chrono::offset::Utc::now(),
        },
    })
}
