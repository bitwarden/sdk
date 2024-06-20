use std::sync::Arc;

use bitwarden::{
    error::Error,
    platform::fido2::{
        CheckUserOptions, ClientData, Fido2CallbackError as BitFido2CallbackError,
        Fido2CredentialAutofillView, GetAssertionRequest, GetAssertionResult,
        MakeCredentialRequest, MakeCredentialResult,
        PublicKeyCredentialAuthenticatorAssertionResponse,
        PublicKeyCredentialAuthenticatorAttestationResponse, PublicKeyCredentialRpEntity,
        PublicKeyCredentialUserEntity,
    },
    vault::{Cipher, CipherView, Fido2CredentialNewView},
};

use crate::{error::Result, Client};

#[derive(uniffi::Object)]
pub struct ClientFido2(pub(crate) Arc<Client>);

#[uniffi::export]
impl ClientFido2 {
    pub fn authenticator(
        self: Arc<Self>,
        user_interface: Arc<dyn Fido2UserInterface>,
        credential_store: Arc<dyn Fido2CredentialStore>,
    ) -> Arc<ClientFido2Authenticator> {
        Arc::new(ClientFido2Authenticator(
            self.0.clone(),
            user_interface,
            credential_store,
        ))
    }

    pub fn client(
        self: Arc<Self>,
        user_interface: Arc<dyn Fido2UserInterface>,
        credential_store: Arc<dyn Fido2CredentialStore>,
    ) -> Arc<ClientFido2Client> {
        Arc::new(ClientFido2Client(ClientFido2Authenticator(
            self.0.clone(),
            user_interface,
            credential_store,
        )))
    }

    pub fn decrypt_fido2_autofill_credentials(
        self: Arc<Self>,
        cipher_view: CipherView,
    ) -> Result<Vec<Fido2CredentialAutofillView>> {
        let result = self
            .0
             .0
            .platform()
            .fido2()
            .decrypt_fido2_autofill_credentials(cipher_view)
            .map_err(Error::DecryptFido2AutofillCredentialsError)?;

        Ok(result)
    }
}

#[derive(uniffi::Object)]
pub struct ClientFido2Authenticator(
    pub(crate) Arc<Client>,
    pub(crate) Arc<dyn Fido2UserInterface>,
    pub(crate) Arc<dyn Fido2CredentialStore>,
);

#[uniffi::export]
impl ClientFido2Authenticator {
    pub async fn make_credential(
        &self,
        request: MakeCredentialRequest,
    ) -> Result<MakeCredentialResult> {
        let platform = self.0 .0.platform();
        let fido2 = platform.fido2();
        let ui = UniffiTraitBridge(self.1.as_ref());
        let cs = UniffiTraitBridge(self.2.as_ref());
        let mut auth = fido2.create_authenticator(&ui, &cs);

        let result = auth
            .make_credential(request)
            .await
            .map_err(Error::MakeCredential)?;
        Ok(result)
    }

    pub async fn get_assertion(&self, request: GetAssertionRequest) -> Result<GetAssertionResult> {
        let platform = self.0 .0.platform();
        let fido2 = platform.fido2();
        let ui = UniffiTraitBridge(self.1.as_ref());
        let cs = UniffiTraitBridge(self.2.as_ref());
        let mut auth = fido2.create_authenticator(&ui, &cs);

        let result = auth
            .get_assertion(request)
            .await
            .map_err(Error::GetAssertion)?;
        Ok(result)
    }

    pub async fn silently_discover_credentials(
        &self,
        rp_id: String,
    ) -> Result<Vec<Fido2CredentialAutofillView>> {
        let platform = self.0 .0.platform();
        let fido2 = platform.fido2();
        let ui = UniffiTraitBridge(self.1.as_ref());
        let cs = UniffiTraitBridge(self.2.as_ref());
        let mut auth = fido2.create_authenticator(&ui, &cs);

        let result = auth
            .silently_discover_credentials(rp_id)
            .await
            .map_err(Error::SilentlyDiscoverCredentials)?;
        Ok(result)
    }

    pub async fn credentials_for_autofill(&self) -> Result<Vec<Fido2CredentialAutofillView>> {
        let platform = self.0 .0.platform();
        let fido2 = platform.fido2();
        let ui = UniffiTraitBridge(self.1.as_ref());
        let cs = UniffiTraitBridge(self.2.as_ref());
        let mut auth = fido2.create_authenticator(&ui, &cs);

        let result = auth
            .credentials_for_autofill()
            .await
            .map_err(Error::CredentialsForAutofillError)?;
        Ok(result)
    }
}

#[derive(uniffi::Object)]
pub struct ClientFido2Client(pub(crate) ClientFido2Authenticator);

#[uniffi::export]
impl ClientFido2Client {
    pub async fn register(
        &self,
        origin: String,
        request: String,
        client_data: ClientData,
    ) -> Result<PublicKeyCredentialAuthenticatorAttestationResponse> {
        let platform = self.0 .0 .0.platform();
        let fido2 = platform.fido2();
        let ui = UniffiTraitBridge(self.0 .1.as_ref());
        let cs = UniffiTraitBridge(self.0 .2.as_ref());
        let mut client = fido2.create_client(&ui, &cs);

        let result = client
            .register(origin, request, client_data)
            .await
            .map_err(Error::Fido2Client)?;
        Ok(result)
    }

    pub async fn authenticate(
        &self,
        origin: String,
        request: String,
        client_data: ClientData,
    ) -> Result<PublicKeyCredentialAuthenticatorAssertionResponse> {
        let platform = self.0 .0 .0.platform();
        let fido2 = platform.fido2();
        let ui = UniffiTraitBridge(self.0 .1.as_ref());
        let cs = UniffiTraitBridge(self.0 .2.as_ref());
        let mut client = fido2.create_client(&ui, &cs);

        let result = client
            .authenticate(origin, request, client_data)
            .await
            .map_err(Error::Fido2Client)?;
        Ok(result)
    }
}

// Note that uniffi doesn't support external traits for now it seems, so we have to duplicate them
// here.

#[allow(dead_code)]
#[derive(uniffi::Record)]
pub struct CheckUserResult {
    user_present: bool,
    user_verified: bool,
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum Fido2CallbackError {
    #[error("The operation requires user interaction")]
    UserInterfaceRequired,

    #[error("The operation was cancelled by the user")]
    OperationCancelled,

    #[error("Unknown error: {reason}")]
    Unknown { reason: String },
}

// Need to implement this From<> impl in order to handle unexpected callback errors.  See the
// following page in the Uniffi user guide:
// <https://mozilla.github.io/uniffi-rs/foreign_traits.html#error-handling>
impl From<uniffi::UnexpectedUniFFICallbackError> for Fido2CallbackError {
    fn from(e: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::Unknown { reason: e.reason }
    }
}

impl From<Fido2CallbackError> for BitFido2CallbackError {
    fn from(val: Fido2CallbackError) -> Self {
        match val {
            Fido2CallbackError::UserInterfaceRequired => Self::UserInterfaceRequired,
            Fido2CallbackError::OperationCancelled => Self::OperationCancelled,
            Fido2CallbackError::Unknown { reason } => Self::Unknown(reason),
        }
    }
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait Fido2UserInterface: Send + Sync {
    async fn check_user(
        &self,
        options: CheckUserOptions,
        hint: UIHint,
    ) -> Result<CheckUserResult, Fido2CallbackError>;
    async fn pick_credential_for_authentication(
        &self,
        available_credentials: Vec<CipherView>,
    ) -> Result<CipherViewWrapper, Fido2CallbackError>;
    async fn check_user_and_pick_credential_for_creation(
        &self,
        options: CheckUserOptions,
        new_credential: Fido2CredentialNewView,
    ) -> Result<CipherViewWrapper, Fido2CallbackError>;
    async fn is_verification_enabled(&self) -> bool;
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait Fido2CredentialStore: Send + Sync {
    async fn find_credentials(
        &self,
        ids: Option<Vec<Vec<u8>>>,
        rip_id: String,
    ) -> Result<Vec<CipherView>, Fido2CallbackError>;

    async fn all_credentials(&self) -> Result<Vec<CipherView>, Fido2CallbackError>;

    async fn save_credential(&self, cred: Cipher) -> Result<(), Fido2CallbackError>;
}

// Because uniffi doesn't support external traits, we have to make a copy of the trait here.
// Ideally we'd want to implement the original trait for every item that implements our local copy,
// but the orphan rules don't allow us to blanket implement an external trait. So we have to wrap
// the trait in a newtype and implement the trait for the newtype.
struct UniffiTraitBridge<T>(T);

#[async_trait::async_trait]
impl bitwarden::platform::fido2::Fido2CredentialStore
    for UniffiTraitBridge<&dyn Fido2CredentialStore>
{
    async fn find_credentials(
        &self,
        ids: Option<Vec<Vec<u8>>>,
        rip_id: String,
    ) -> Result<Vec<CipherView>, BitFido2CallbackError> {
        self.0
            .find_credentials(ids, rip_id)
            .await
            .map_err(Into::into)
    }

    async fn all_credentials(&self) -> Result<Vec<CipherView>, BitFido2CallbackError> {
        self.0.all_credentials().await.map_err(Into::into)
    }

    async fn save_credential(&self, cred: Cipher) -> Result<(), BitFido2CallbackError> {
        self.0.save_credential(cred).await.map_err(Into::into)
    }
}

// Uniffi seems to have trouble generating code for Android when a local trait returns a type from
// an external crate. If the type is small we can just copy it over and convert back and forth, but
// Cipher is too big for that to be practical. So we wrap it in a newtype, which is local to the
// trait and so we can sidestep the Uniffi issue
#[derive(uniffi::Record)]
pub struct CipherViewWrapper {
    cipher: CipherView,
}

#[derive(uniffi::Enum)]
pub enum UIHint {
    InformExcludedCredentialFound(CipherView),
    InformNoCredentialsFound,
    RequestNewCredential(PublicKeyCredentialUserEntity, PublicKeyCredentialRpEntity),
    RequestExistingCredential(CipherView),
}

impl From<bitwarden::platform::fido2::UIHint<'_, CipherView>> for UIHint {
    fn from(hint: bitwarden::platform::fido2::UIHint<'_, CipherView>) -> Self {
        use bitwarden::platform::fido2::UIHint as BWUIHint;
        match hint {
            BWUIHint::InformExcludedCredentialFound(cipher) => {
                UIHint::InformExcludedCredentialFound(cipher.clone())
            }
            BWUIHint::InformNoCredentialsFound => UIHint::InformNoCredentialsFound,
            BWUIHint::RequestNewCredential(user, rp) => UIHint::RequestNewCredential(
                PublicKeyCredentialUserEntity {
                    id: user.id.clone().into(),
                    name: user.name.clone().unwrap_or_default(),
                    display_name: user.display_name.clone().unwrap_or_default(),
                },
                PublicKeyCredentialRpEntity {
                    id: rp.id.clone(),
                    name: rp.name.clone(),
                },
            ),
            BWUIHint::RequestExistingCredential(cipher) => {
                UIHint::RequestExistingCredential(cipher.clone())
            }
        }
    }
}

#[async_trait::async_trait]
impl bitwarden::platform::fido2::Fido2UserInterface for UniffiTraitBridge<&dyn Fido2UserInterface> {
    async fn check_user<'a>(
        &self,
        options: CheckUserOptions,
        hint: bitwarden::platform::fido2::UIHint<'a, CipherView>,
    ) -> Result<bitwarden::platform::fido2::CheckUserResult, BitFido2CallbackError> {
        self.0
            .check_user(options.clone(), hint.into())
            .await
            .map(|r| bitwarden::platform::fido2::CheckUserResult {
                user_present: r.user_present,
                user_verified: r.user_verified,
            })
            .map_err(Into::into)
    }
    async fn pick_credential_for_authentication(
        &self,
        available_credentials: Vec<CipherView>,
    ) -> Result<CipherView, BitFido2CallbackError> {
        self.0
            .pick_credential_for_authentication(available_credentials)
            .await
            .map(|v| v.cipher)
            .map_err(Into::into)
    }
    async fn check_user_and_pick_credential_for_creation(
        &self,
        options: CheckUserOptions,
        new_credential: Fido2CredentialNewView,
    ) -> Result<CipherView, BitFido2CallbackError> {
        self.0
            .check_user_and_pick_credential_for_creation(options, new_credential)
            .await
            .map(|v| v.cipher)
            .map_err(Into::into)
    }
    async fn is_verification_enabled(&self) -> bool {
        self.0.is_verification_enabled().await
    }
}
