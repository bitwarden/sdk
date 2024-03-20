use std::{
    borrow::{Borrow, BorrowMut},
    cell::Cell,
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
    credential_store::Fido2CredentialStore, user_interface::Fido2UserInterface,
    NewCredentialParams, VaultItem,
};

// TODO: Use Mutex instead Wrap
#[derive(Default)]
struct Wrap<T>(T);
unsafe impl<T> Sync for Wrap<T> {}

pub enum Fido2Options {
    CreateCredential(webauthn::CredentialCreationOptions),
    GetCredential(webauthn::CredentialRequestOptions),
}

pub struct Fido2Transaction<U, S>
where
    U: Fido2UserInterface,
    S: Fido2CredentialStore,
{
    options: Wrap<Fido2Options>,
    user_interface: Wrap<U>,
    credential_store: Wrap<S>,
    user_presence: Wrap<Cell<bool>>,
}

impl<U, S> Fido2Transaction<U, S>
where
    U: Fido2UserInterface,
    S: Fido2CredentialStore,
{
    pub fn new(request: Fido2Options, user_interface: U, credential_store: S) -> Self {
        Self {
            options: Wrap(request),
            user_interface: Wrap(user_interface),
            credential_store: Wrap(credential_store),
            user_presence: Wrap(Cell::new(false)),
        }
    }

    pub fn into_user_validation_method(&self) -> PasskeyRsUserValidationMethod<U, S> {
        PasskeyRsUserValidationMethod { context: &self }
    }

    pub fn into_credential_store(&self) -> PasskeyRsCredentialStore<U, S> {
        PasskeyRsCredentialStore { context: &self }
    }
}

pub struct PasskeyRsCredentialStore<'a, U, S>
where
    U: Fido2UserInterface,
    S: Fido2CredentialStore,
{
    context: &'a Fido2Transaction<U, S>,
}

#[async_trait::async_trait]
impl<'a, U, S> CredentialStore for PasskeyRsCredentialStore<'a, U, S>
where
    U: Fido2UserInterface,
    S: Fido2CredentialStore,
{
    type PasskeyItem = VaultItem;

    async fn find_credentials(
        &self,
        ids: Option<&[PublicKeyCredentialDescriptor]>,
        rp_id: &str,
    ) -> std::result::Result<Vec<Self::PasskeyItem>, StatusCode> {
        let result = self
            .context
            .credential_store
            .0
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
        match self.context.options.0.borrow() {
            Fido2Options::CreateCredential(request) => {
                let target = self
                    .context
                    .user_interface
                    .0
                    .confirm_new_credential(NewCredentialParams {
                        credential_name: cred.rp_id, // TODO: This should take other fields into account
                        user_name: user.name.unwrap_or("Unknown name".to_owned()),
                    })
                    .await
                    // TODO: We should convert the error more intelligently
                    .map_err(|error| StatusCode::Ctap2(Ctap2Code::Known(Ctap2Error::NotAllowed)))?;

                self.context.user_presence.0.set(true); // true because the user actively interacted with the UI

                // target.vault_item.fido2_credential = new Fido2Credential(cred, user, rp);

                // TODO: Fix trying to use non-mutable reference
                // self.context
                //     .credential_store
                //     .0
                //     .get_mut()
                //     .save_credential(target.vault_item, user, rp)
                //     .await?;

                std::result::Result::Ok(())
            }
            Fido2Options::GetCredential(_) => {
                todo!("save_credential for GetCredential not implemented");
                // self.get_credential(cred, user, rp).await?;
            }
        }
    }
}

pub struct PasskeyRsUserValidationMethod<'a, U, S>
where
    U: Fido2UserInterface,
    S: Fido2CredentialStore,
{
    context: &'a Fido2Transaction<U, S>,
}

#[async_trait::async_trait]
impl<'a, U, S> UserValidationMethod for PasskeyRsUserValidationMethod<'a, U, S>
where
    U: Fido2UserInterface,
    S: Fido2CredentialStore,
{
    async fn check_user_verification(&self) -> bool {
        self.context
            .user_interface
            .0
            .check_user_verification()
            .await
    }

    async fn check_user_presence(&self) -> bool {
        if self.context.user_presence.0.get() {
            return true;
        }

        self.context.user_interface.0.check_user_presence().await
    }

    fn is_presence_enabled(&self) -> bool {
        true
    }

    fn is_verification_enabled(&self) -> Option<bool> {
        Some(true)
    }
}
