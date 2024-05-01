use std::sync::Arc;

use bitwarden::{
    error::Result as BitResult,
    platform::fido2::{
        CheckUserOptions, CheckUserResult, ClientData, GetAssertionRequest, GetAssertionResult,
        MakeCredentialRequest, MakeCredentialResult,
        PublicKeyCredentialAuthenticatorAssertionResponse,
        PublicKeyCredentialAuthenticatorAttestationResponse,
    },
    vault::{Cipher, CipherView, Fido2Credential, Fido2CredentialView},
};

use crate::{error::Result, Client};

#[derive(uniffi::Object)]
pub struct ClientFido2(pub(crate) Arc<Client>);

#[uniffi::export]
impl ClientFido2 {
    pub fn authenticator(
        self: Arc<Self>,
        user_interface: Arc<dyn UserInterface>,
        credential_store: Arc<dyn CredentialStore>,
    ) -> Arc<ClientFido2Authenticator> {
        Arc::new(ClientFido2Authenticator(
            self.0.clone(),
            user_interface,
            credential_store,
        ))
    }

    pub fn client(
        self: Arc<Self>,
        user_interface: Arc<dyn UserInterface>,
        credential_store: Arc<dyn CredentialStore>,
    ) -> Arc<ClientFido2Client> {
        Arc::new(ClientFido2Client(ClientFido2Authenticator(
            self.0.clone(),
            user_interface,
            credential_store,
        )))
    }
}

#[derive(uniffi::Object)]
pub struct ClientFido2Authenticator(
    pub(crate) Arc<Client>,
    pub(crate) Arc<dyn UserInterface>,
    pub(crate) Arc<dyn CredentialStore>,
);

#[uniffi::export]
impl ClientFido2Authenticator {
    pub async fn make_credential(
        &self,
        request: MakeCredentialRequest,
    ) -> Result<MakeCredentialResult> {
        let mut client = self.0 .0.write().await;

        let mut platform = client.platform();
        let mut fido2 = platform.fido2();
        let mut auth = fido2.create_authenticator(
            UniffiTraitBridge(self.1.as_ref()),
            UniffiTraitBridge(self.2.as_ref()),
        )?;

        let result = auth.make_credential(request).await?;
        Ok(result)
    }

    pub async fn get_assertion(&self, request: GetAssertionRequest) -> Result<GetAssertionResult> {
        let mut client = self.0 .0.write().await;

        let mut platform = client.platform();
        let mut fido2 = platform.fido2();
        let mut auth = fido2.create_authenticator(
            UniffiTraitBridge(self.1.as_ref()),
            UniffiTraitBridge(self.2.as_ref()),
        )?;

        let result = auth.get_assertion(request).await?;
        Ok(result)
    }

    pub async fn silently_discover_credentials(
        &self,
        rp_id: String,
    ) -> Result<Vec<Fido2CredentialView>> {
        let mut client = self.0 .0.write().await;

        let mut platform = client.platform();
        let mut fido2 = platform.fido2();
        let mut auth = fido2.create_authenticator(
            UniffiTraitBridge(self.1.as_ref()),
            UniffiTraitBridge(self.2.as_ref()),
        )?;

        let result = auth.silently_discover_credentials(rp_id).await?;
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
        let mut client = self.0 .0 .0.write().await;

        let mut platform = client.platform();
        let mut fido2 = platform.fido2();
        let mut client = fido2.create_client(
            UniffiTraitBridge(self.0 .1.as_ref()),
            UniffiTraitBridge(self.0 .2.as_ref()),
        )?;

        let result = client.register(origin, request, client_data).await?;
        Ok(result)
    }

    pub async fn authenticate(
        &self,
        origin: String,
        request: String,
        client_data: ClientData,
    ) -> Result<PublicKeyCredentialAuthenticatorAssertionResponse> {
        let mut client = self.0 .0 .0.write().await;

        let mut platform = client.platform();
        let mut fido2 = platform.fido2();
        let mut client = fido2.create_client(
            UniffiTraitBridge(self.0 .1.as_ref()),
            UniffiTraitBridge(self.0 .2.as_ref()),
        )?;

        let result = client.authenticate(origin, request, client_data).await?;
        Ok(result)
    }
}

// Note that uniffi doesn't support external traits for now it seems, so we have to duplicate them
// here.

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait UserInterface: Send + Sync {
    async fn check_user(
        &self,
        options: CheckUserOptions,
        credential: Option<CipherView>,
    ) -> Result<CheckUserResult>;
    async fn pick_credential_for_authentication(
        &self,
        available_credentials: Vec<Cipher>,
    ) -> Result<CipherView>;
    async fn pick_credential_for_creation(
        &self,
        available_credentials: Vec<Cipher>,
        new_credential: Fido2Credential,
    ) -> Result<CipherView>;
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait CredentialStore: Send + Sync {
    async fn find_credentials(
        &self,
        ids: Option<Vec<Vec<u8>>>,
        rip_id: String,
    ) -> Result<Vec<Cipher>>;

    async fn save_credential(&self, cred: Cipher) -> Result<()>;
}

struct UniffiTraitBridge<T>(T);

#[async_trait::async_trait]
impl bitwarden::platform::fido2::CredentialStore for UniffiTraitBridge<&dyn CredentialStore> {
    async fn find_credentials(
        &self,
        ids: Option<Vec<Vec<u8>>>,
        rip_id: String,
    ) -> BitResult<Vec<Cipher>> {
        self.0
            .find_credentials(ids, rip_id)
            .await
            .map_err(Into::into)
    }

    async fn save_credential(&self, cred: Cipher) -> BitResult<()> {
        self.0.save_credential(cred).await.map_err(Into::into)
    }
}

#[async_trait::async_trait]
impl bitwarden::platform::fido2::UserInterface for UniffiTraitBridge<&dyn UserInterface> {
    async fn check_user(
        &self,
        options: CheckUserOptions,
        credential: Option<CipherView>,
    ) -> BitResult<CheckUserResult> {
        self.0
            .check_user(options, credential)
            .await
            .map_err(Into::into)
    }
    async fn pick_credential_for_authentication(
        &self,
        available_credentials: Vec<Cipher>,
    ) -> BitResult<CipherView> {
        self.0
            .pick_credential_for_authentication(available_credentials)
            .await
            .map_err(Into::into)
    }
    async fn pick_credential_for_creation(
        &self,
        available_credentials: Vec<Cipher>,
        new_credential: Fido2Credential,
    ) -> BitResult<CipherView> {
        self.0
            .pick_credential_for_creation(available_credentials, new_credential)
            .await
            .map_err(Into::into)
    }
}
