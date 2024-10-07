use std::borrow::Cow;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use bitwarden_core::key_management::{AsymmetricKeyRef, SymmetricKeyRef};
use bitwarden_crypto::service::CryptoServiceContext;
use bitwarden_vault::{CipherError, CipherView};
use passkey::types::webauthn::UserVerificationRequirement;
use reqwest::Url;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    get_enum_from_string_name, string_to_guid_bytes, InvalidGuid, SelectedCredential, UnknownEnum,
    Verification,
};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Fido2CredentialAutofillView {
    pub credential_id: Vec<u8>,
    pub cipher_id: uuid::Uuid,
    pub rp_id: String,
    pub user_name_for_ui: Option<String>,
    pub user_handle: Vec<u8>,
}

trait NoneWhitespace {
    /// Convert only whitespace to None
    fn none_whitespace(&self) -> Option<String>;
}

impl NoneWhitespace for String {
    fn none_whitespace(&self) -> Option<String> {
        match self.trim() {
            "" => None,
            s => Some(s.to_owned()),
        }
    }
}

impl NoneWhitespace for Option<String> {
    fn none_whitespace(&self) -> Option<String> {
        self.as_ref().and_then(|s| s.none_whitespace())
    }
}

#[derive(Debug, Error)]
pub enum Fido2CredentialAutofillViewError {
    #[error(
        "Autofill credentials can only be created from existing ciphers that have a cipher id"
    )]
    MissingCipherId,

    #[error(transparent)]
    InvalidGuid(#[from] InvalidGuid),

    #[error(transparent)]
    CipherError(#[from] CipherError),

    #[error(transparent)]
    Base64DecodeError(#[from] base64::DecodeError),
}

impl Fido2CredentialAutofillView {
    pub fn from_cipher_view(
        cipher: &CipherView,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
    ) -> Result<Vec<Fido2CredentialAutofillView>, Fido2CredentialAutofillViewError> {
        let credentials = cipher.decrypt_fido2_credentials(ctx)?;

        credentials
            .into_iter()
            .filter_map(|c| -> Option<Result<_, Fido2CredentialAutofillViewError>> {
                c.user_handle
                    .map(|u| URL_SAFE_NO_PAD.decode(u))
                    .map(|user_handle| {
                        Ok(Fido2CredentialAutofillView {
                            credential_id: string_to_guid_bytes(&c.credential_id)?,
                            cipher_id: cipher
                                .id
                                .ok_or(Fido2CredentialAutofillViewError::MissingCipherId)?,
                            rp_id: c.rp_id.clone(),
                            user_handle: user_handle?,
                            user_name_for_ui: c
                                .user_name
                                .none_whitespace()
                                .or(c.user_display_name.none_whitespace())
                                .or(cipher
                                    .login
                                    .as_ref()
                                    .and_then(|l| l.username.none_whitespace()))
                                .or(cipher.name.none_whitespace()),
                        })
                    })
            })
            .collect()
    }
}

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

#[derive(Debug, Error)]
pub enum PublicKeyCredentialParametersError {
    #[error("Invalid algorithm")]
    InvalidAlgorithm,

    #[error("Unknown type")]
    UnknownEnum(#[from] UnknownEnum),
}

impl TryFrom<PublicKeyCredentialParameters>
    for passkey::types::webauthn::PublicKeyCredentialParameters
{
    type Error = PublicKeyCredentialParametersError;

    fn try_from(value: PublicKeyCredentialParameters) -> Result<Self, Self::Error> {
        use coset::iana::EnumI64;
        Ok(Self {
            ty: get_enum_from_string_name(&value.ty)?,
            alg: coset::iana::Algorithm::from_i64(value.alg)
                .ok_or(PublicKeyCredentialParametersError::InvalidAlgorithm)?,
        })
    }
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PublicKeyCredentialDescriptor {
    pub ty: String,
    pub id: Vec<u8>,
    pub transports: Option<Vec<String>>,
}

impl TryFrom<PublicKeyCredentialDescriptor>
    for passkey::types::webauthn::PublicKeyCredentialDescriptor
{
    type Error = UnknownEnum;

    fn try_from(value: PublicKeyCredentialDescriptor) -> Result<Self, Self::Error> {
        Ok(Self {
            ty: get_enum_from_string_name(&value.ty)?,
            id: value.id.into(),
            transports: value
                .transports
                .map(|tt| {
                    tt.into_iter()
                        .map(|t| get_enum_from_string_name(&t))
                        .collect::<Result<Vec<_>, Self::Error>>()
                })
                .transpose()?,
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
    pub attestation_object: Vec<u8>,
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

impl From<Options> for super::CheckUserOptions {
    fn from(value: Options) -> Self {
        Self {
            require_presence: value.rk,
            require_verification: value.uv.into(),
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

impl From<UserVerificationRequirement> for UV {
    fn from(value: UserVerificationRequirement) -> Self {
        match value {
            UserVerificationRequirement::Discouraged => UV::Discouraged,
            UserVerificationRequirement::Preferred => UV::Preferred,
            UserVerificationRequirement::Required => UV::Required,
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

#[derive(Debug, Error)]
#[error("Invalid origin: {0}")]
pub struct InvalidOriginError(String);

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
/// An Unverified asset link.
pub struct UnverifiedAssetLink {
    /// Application package name.
    package_name: String,
    /// Fingerprint to compare.
    sha256_cert_fingerprint: String,
    /// Host to lookup the well known asset link.
    host: String,
    /// When sourced from the application statement list or parsed from host for passkeys.
    /// Will be generated from `host` if not provided.
    asset_link_url: Option<String>,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
/// The origin of a WebAuthn request.
pub enum Origin {
    /// A Url, meant for a request in the web browser.
    Web(String),
    /// An android digital asset fingerprint.
    /// Meant for a request coming from an android application.
    Android(UnverifiedAssetLink),
}

impl<'a> TryFrom<Origin> for passkey::client::Origin<'a> {
    type Error = InvalidOriginError;

    fn try_from(value: Origin) -> Result<Self, Self::Error> {
        Ok(match value {
            Origin::Web(url) => {
                let url = Url::parse(&url).map_err(|e| InvalidOriginError(format!("{}", e)))?;
                passkey::client::Origin::Web(Cow::Owned(url))
            }
            Origin::Android(link) => passkey::client::Origin::Android(link.try_into()?),
        })
    }
}

impl<'a> TryFrom<UnverifiedAssetLink> for passkey::client::UnverifiedAssetLink<'a> {
    type Error = InvalidOriginError;

    fn try_from(value: UnverifiedAssetLink) -> Result<Self, Self::Error> {
        let asset_link_url = match value.asset_link_url {
            Some(url) => Some(Url::parse(&url).map_err(|e| InvalidOriginError(format!("{}", e)))?),
            None => None,
        };

        passkey::client::UnverifiedAssetLink::new(
            Cow::from(value.package_name),
            value.sha256_cert_fingerprint.as_str(),
            Cow::from(value.host),
            asset_link_url,
        )
        .map_err(|e| InvalidOriginError(format!("{:?}", e)))
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::AndroidClientData;

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
