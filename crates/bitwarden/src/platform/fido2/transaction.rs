use std::{
    borrow::{Borrow, BorrowMut},
    cell::Cell,
    sync::Arc,
};

use passkey::{
    authenticator::{CredentialStore, UserValidationMethod},
    types::{
        ctap2::{self, Ctap2Code, Ctap2Error, StatusCode},
        webauthn::{self, PublicKeyCredentialDescriptor},
        Passkey,
    },
};

use super::{
    credential_store::Fido2CredentialStore, user_interface::Fido2UserInterface, Fido2VaultItem,
    NewCredentialParams,
};

pub enum Fido2Options {
    CreateCredential(webauthn::CredentialCreationOptions),
    GetCredential(webauthn::CredentialRequestOptions),
}

pub struct Fido2Transaction<U, S>
where
    U: Fido2UserInterface + Send + Sync,
    S: Fido2CredentialStore + Send,
{
    options: Fido2Options,
    user_interface: Arc<U>,
    credential_store: async_lock::Mutex<S>,
    user_presence: async_lock::Mutex<bool>,
}

impl<U, S> Fido2Transaction<U, S>
where
    U: Fido2UserInterface + Send + Sync,
    S: Fido2CredentialStore + Send,
{
    pub fn new(request: Fido2Options, user_interface: U, credential_store: S) -> Self {
        Self {
            options: request,
            user_interface: Arc::new(user_interface),
            credential_store: async_lock::Mutex::new(credential_store),
            user_presence: async_lock::Mutex::new(false),
        }
    }

    pub fn into_user_validation_method(self: &Arc<Self>) -> PasskeyRsUserValidationMethod<U, S> {
        PasskeyRsUserValidationMethod {
            context: Arc::clone(self),
        }
    }

    pub fn into_credential_store(self: &Arc<Self>) -> PasskeyRsCredentialStore<U, S> {
        PasskeyRsCredentialStore {
            context: Arc::clone(self),
        }
    }
}

pub struct PasskeyRsCredentialStore<U, S>
where
    U: Fido2UserInterface + Send + Sync,
    S: Fido2CredentialStore + Send,
{
    context: Arc<Fido2Transaction<U, S>>,
}

#[async_trait::async_trait]
impl<U, S> CredentialStore for PasskeyRsCredentialStore<U, S>
where
    U: Fido2UserInterface + Send + Sync,
    S: Fido2CredentialStore + Send,
{
    type PasskeyItem = Fido2VaultItem;

    async fn find_credentials(
        &self,
        ids: Option<&[PublicKeyCredentialDescriptor]>,
        rp_id: &str,
    ) -> std::result::Result<Vec<Self::PasskeyItem>, StatusCode> {
        log::info!("PasskeyRsCredentialStore.find_credentials");
        let result = self
            .context
            .credential_store
            .lock()
            .await
            .find_credentials(super::FindCredentialsParams {
                ids: ids
                    .unwrap_or_default()
                    .iter()
                    .map(|descriptor| descriptor.id.clone())
                    .collect(),
                rp_id: rp_id.to_owned(),
            })
            .await
            // TODO: Look into error code
            .map_err(|_error| StatusCode::Ctap2(Ctap2Code::Known(Ctap2Error::NotAllowed)))?;

        std::result::Result::Ok(result)
    }

    async fn save_credential(
        &mut self,
        cred: Passkey,
        user: ctap2::make_credential::PublicKeyCredentialUserEntity,
        rp: ctap2::make_credential::PublicKeyCredentialRpEntity,
    ) -> Result<(), StatusCode> {
        match &self.context.options {
            Fido2Options::CreateCredential(request) => {
                let target = self
                    .context
                    .user_interface
                    .confirm_new_credential(NewCredentialParams {
                        credential_name: cred.rp_id, /* TODO: This should take other fields into
                                                      * account */
                        user_name: user.name.clone().unwrap_or("Unknown name".to_owned()),
                    })
                    .await
                    // TODO: We should convert the error more intelligently
                    .map_err(|error| StatusCode::Ctap2(Ctap2Code::Known(Ctap2Error::NotAllowed)))?;

                *self.context.user_presence.lock().await = true; // true because the user actively interacted with the UI

                // target.vault_item.fido2_credential = new Fido2Credential(cred, user, rp);

                // TODO: Fix trying to use non-mutable reference
                self.context
                    .credential_store
                    .lock()
                    .await
                    .save_credential(super::SaveCredentialParams {
                        cred: target.vault_item,
                        user,
                        rp,
                    })
                    .await // TODO: Don't unwrap this but return an actual Err
                    .unwrap();

                std::result::Result::Ok(())
            }
            Fido2Options::GetCredential(_) => {
                todo!("save_credential for GetCredential not implemented");
                // self.get_credential(cred, user, rp).await?;
            }
        }
    }
}

pub struct PasskeyRsUserValidationMethod<U, S>
where
    U: Fido2UserInterface + Send + Sync,
    S: Fido2CredentialStore + Send,
{
    context: Arc<Fido2Transaction<U, S>>,
}

#[async_trait::async_trait]
impl<U, S> UserValidationMethod for PasskeyRsUserValidationMethod<U, S>
where
    U: Fido2UserInterface + Send + Sync,
    S: Fido2CredentialStore + Send,
{
    async fn check_user_verification(&self) -> bool {
        self.context.user_interface.check_user_verification().await
    }

    async fn check_user_presence(&self) -> bool {
        if *self.context.user_presence.lock().await {
            return true;
        }

        self.context.user_interface.check_user_presence().await
    }

    fn is_presence_enabled(&self) -> bool {
        true
    }

    fn is_verification_enabled(&self) -> Option<bool> {
        Some(true)
    }
}
