use std::{ops::Deref, sync::Mutex};

use bitwarden_crypto::KeyEncryptable;
use log::error;
use passkey::{
    authenticator::{Authenticator, UserCheck},
    types::{
        ctap2::{self, Ctap2Error, StatusCode},
        Passkey,
    },
};

use super::{
    types::*, CheckUserOptions, CipherViewContainer, Fido2CredentialStore, Fido2UserInterface,
    SelectedCredential, Verification, AAGUID,
};
use crate::{
    error::{require, Error, Result},
    vault::{login::Fido2CredentialView, CipherView, Fido2CredentialFullView},
    Client,
};

pub struct Fido2Authenticator<'a> {
    pub(crate) client: &'a mut Client,
    pub(crate) user_interface: &'a dyn Fido2UserInterface,
    pub(crate) credential_store: &'a dyn Fido2CredentialStore,

    pub(crate) selected_credential: Mutex<Option<CipherView>>,
}

impl<'a> Fido2Authenticator<'a> {
    pub async fn make_credential(
        &mut self,
        request: MakeCredentialRequest,
    ) -> Result<MakeCredentialResult> {
        let mut authenticator = self.get_authenticator();

        let response = authenticator
            .make_credential(ctap2::make_credential::Request {
                client_data_hash: request.client_data_hash.into(),
                rp: passkey::types::ctap2::make_credential::PublicKeyCredentialRpEntity {
                    id: request.rp.id,
                    name: request.rp.name,
                },
                user: passkey::types::webauthn::PublicKeyCredentialUserEntity {
                    id: request.user.id.into(),
                    display_name: request.user.display_name,
                    name: request.user.name,
                },
                pub_key_cred_params: request
                    .pub_key_cred_params
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?,
                exclude_list: request
                    .exclude_list
                    .map(|x| x.into_iter().map(TryInto::try_into).collect())
                    .transpose()?,
                extensions: request
                    .extensions
                    .map(|e| serde_json::from_str(&e))
                    .transpose()?,
                options: passkey::types::ctap2::make_credential::Options {
                    rk: request.require_resident_key,
                    up: true,
                    uv: request.options.uv != UV::Discouraged,
                },
                pin_auth: None,
                pin_protocol: None,
            })
            .await;

        let response = match response {
            Ok(x) => x,
            Err(e) => return Err(format!("make_credential error: {e:?}").into()),
        };

        Ok(MakeCredentialResult {
            // TODO(Fido2): We can get these from response.auth_data I think?
            authenticator_data: Vec::new(),
            attested_credential_data: Vec::new(),
            credential_id: response
                .auth_data
                .attested_credential_data
                .expect("Missing attested_credential_data")
                .credential_id()
                .to_vec(),
        })
    }

    pub async fn get_assertion(
        &mut self,
        request: GetAssertionRequest,
    ) -> Result<GetAssertionResult> {
        let mut authenticator = self.get_authenticator();

        let response = authenticator
            .get_assertion(ctap2::get_assertion::Request {
                rp_id: request.rp_id,
                client_data_hash: request.client_data_hash.into(),
                allow_list: request
                    .allow_list
                    .map(|l| {
                        l.into_iter()
                            .map(TryInto::try_into)
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .transpose()?,
                extensions: request
                    .extensions
                    .map(|e| serde_json::from_str(&e))
                    .transpose()?,
                options: passkey::types::ctap2::make_credential::Options {
                    rk: request.options.rk,
                    up: true,
                    uv: request.options.uv != UV::Discouraged,
                },
                pin_auth: None,
                pin_protocol: None,
            })
            .await;

        let response = match response {
            Ok(x) => x,
            Err(e) => return Err(format!("get_assertion error: {e:?}").into()),
        };

        Ok(GetAssertionResult {
            credential_id: response
                .auth_data
                .attested_credential_data
                .ok_or("Missing attested_credential_data")?
                .credential_id()
                .to_vec(),
            // TODO(Fido2): We can get these from response.auth_data I think?
            authenticator_data: vec![],
            signature: response.signature.into(),
            user_handle: response.user.ok_or("Missing user")?.id.into(),
            selected_credential: self.get_selected_credential()?,
        })
    }

    // TODO(Fido2): We need some changes in the passkey crate to do this
    pub async fn silently_discover_credentials(
        &mut self,
        rp_id: String,
    ) -> Result<Vec<Fido2CredentialView>> {
        let result = self.credential_store.find_credentials(None, rp_id).await?;
        Ok(result
            .into_iter()
            .filter_map(|c| c.login?.fido2_credentials)
            .flatten()
            .collect())
    }

    pub(super) fn get_authenticator(
        &self,
    ) -> Authenticator<CredentialStoreImpl, UserValidationMethodImpl> {
        Authenticator::new(
            AAGUID,
            CredentialStoreImpl {
                authenticator: self,
            },
            UserValidationMethodImpl {
                authenticator: self,
            },
        )
    }

    pub(super) fn get_selected_credential(&self) -> Result<SelectedCredential> {
        let cipher = self
            .selected_credential
            .lock()
            .expect("Mutex is not poisoned")
            .take()
            .ok_or("No selected credential available")?;

        let login = require!(cipher.login.as_ref());
        let creds = require!(login.fido2_credentials.as_ref());

        let credential = creds.first().ok_or("No Fido2 credentials found")?.clone();

        Ok(SelectedCredential { cipher, credential })
    }
}

pub(super) struct CredentialStoreImpl<'a> {
    authenticator: &'a Fido2Authenticator<'a>,
}
pub(super) struct UserValidationMethodImpl<'a> {
    authenticator: &'a Fido2Authenticator<'a>,
}

#[async_trait::async_trait]
impl passkey::authenticator::CredentialStore for CredentialStoreImpl<'_> {
    type PasskeyItem = CipherViewContainer;
    async fn find_credentials(
        &self,
        ids: Option<&[passkey::types::webauthn::PublicKeyCredentialDescriptor]>,
        rp_id: &str,
    ) -> Result<Vec<Self::PasskeyItem>, StatusCode> {
        // This is just a wrapper around the actual implementation to allow for ? error handling
        // TODO(Fido2): User is unused, do we need it?
        async fn inner(
            this: &CredentialStoreImpl<'_>,
            ids: Option<&[passkey::types::webauthn::PublicKeyCredentialDescriptor]>,
            rp_id: &str,
        ) -> Result<Vec<CipherViewContainer>> {
            let ids: Option<Vec<Vec<u8>>> =
                ids.map(|ids| ids.iter().map(|id| id.id.clone().into()).collect());

            let ciphers = this
                .authenticator
                .credential_store
                .find_credentials(ids, rp_id.to_string())
                .await?;

            let enc = this.authenticator.client.get_encryption_settings()?;

            // Remove any that don't have Fido2 credentials and convert them to the container type
            let result: Vec<CipherViewContainer> = ciphers
                .into_iter()
                .filter(|c| {
                    c.login
                        .as_ref()
                        .and_then(|l| l.fido2_credentials.as_ref())
                        .is_some()
                })
                .map(|c| CipherViewContainer::new(c, enc))
                .collect::<Result<_, _>>()?;

            // Store the selected credential for later use
            *this
                .authenticator
                .selected_credential
                .lock()
                .expect("Mutex is not poisoned") = result.first().map(|c| &c.cipher).cloned();

            Ok(result)
        }

        inner(self, ids, rp_id).await.map_err(|e| {
            error!("Error finding credentials: {e:?}");
            // TODO(Fido2): What error code should we return here?
            StatusCode::from(9)
        })
    }

    async fn save_credential(
        &mut self,
        cred: Passkey,
        user: passkey::types::ctap2::make_credential::PublicKeyCredentialUserEntity,
        rp: passkey::types::ctap2::make_credential::PublicKeyCredentialRpEntity,
        // TODO(Fido2): Use this
        _options: passkey::types::ctap2::get_assertion::Options,
    ) -> Result<(), StatusCode> {
        // This is just a wrapper around the actual implementation to allow for ? error handling
        async fn inner(
            this: &mut CredentialStoreImpl<'_>,
            cred: Passkey,
            user: passkey::types::ctap2::make_credential::PublicKeyCredentialUserEntity,
            rp: passkey::types::ctap2::make_credential::PublicKeyCredentialRpEntity,
        ) -> Result<()> {
            let creds = this
                .authenticator
                .credential_store
                .find_credentials(None, rp.id.clone())
                .await?;

            let cred = Fido2CredentialFullView::try_from_credential(cred, user, rp)?;

            let mut picked = this
                .authenticator
                .user_interface
                .pick_credential_for_creation(creds, cred.clone().into())
                .await?;

            let enc = this.authenticator.client.get_encryption_settings()?;

            picked.set_new_fido2_credentials(enc, vec![cred])?;

            // Store the selected credential for later use
            *this
                .authenticator
                .selected_credential
                .lock()
                .expect("Mutex is not poisoned") = Some(picked.clone());

            let key = enc
                .get_key(&picked.organization_id)
                .ok_or(Error::VaultLocked)?;
            let encrypted = picked.encrypt_with_key(key)?;

            this.authenticator
                .credential_store
                .save_credential(encrypted)
                .await?;

            Ok(())
        }

        inner(self, cred, user, rp).await.map_err(|e| {
            error!("Error saving credential: {e:?}");
            // TODO(Fido2): What error code should we return here?
            StatusCode::from(9)
        })
    }

    async fn update_credential(&mut self, cred: Passkey) -> Result<(), StatusCode> {
        // This is just a wrapper around the actual implementation to allow for ? error handling
        async fn inner(this: &mut CredentialStoreImpl<'_>, cred: Passkey) -> Result<()> {
            let mut creds = this
                .authenticator
                .credential_store
                .find_credentials(None, cred.rp_id.clone())
                .await?;

            // Get the credential with the same id as the one we're updating
            creds.retain(|c| {
                c.login
                    .as_ref()
                    .and_then(|l| l.fido2_credentials.as_ref())
                    .and_then(|f| f.first())
                    .map(|cipher_cred| {
                        cipher_cred.credential_id.expose().as_bytes() == cred.credential_id.deref()
                    })
                    .unwrap_or(false)
            });
            let cred = match creds.len() {
                1 => creds.into_iter().next().expect("Vec has one element"),
                _ => return Err("Only one credential should match the id".into()),
            };

            let enc = this.authenticator.client.get_encryption_settings()?;

            // Store the selected credential for later use
            *this
                .authenticator
                .selected_credential
                .lock()
                .expect("Mutex is not poisoned") = Some(cred.clone());

            let key = enc
                .get_key(&cred.organization_id)
                .ok_or(Error::VaultLocked)?;
            let encrypted = cred.encrypt_with_key(key)?;

            this.authenticator
                .credential_store
                .save_credential(encrypted)
                .await?;

            Ok(())
        }

        inner(self, cred).await.map_err(|e| {
            error!("Error updating credential: {e:?}");
            // TODO(Fido2): What error code should we return here?
            StatusCode::from(9)
        })
    }
}

#[async_trait::async_trait]
impl passkey::authenticator::UserValidationMethod for UserValidationMethodImpl<'_> {
    type PasskeyItem = CipherViewContainer;

    async fn check_user(
        &self,
        credential: Option<Self::PasskeyItem>,
        presence: bool,
        verification: bool,
    ) -> Result<UserCheck, Ctap2Error> {
        let options = CheckUserOptions {
            require_presence: presence,
            require_verification: if verification {
                Verification::Required
            } else {
                Verification::Discouraged
            },
        };

        let result = match self
            .authenticator
            .user_interface
            .check_user(options, credential.map(|c| c.cipher))
            .await
        {
            Ok(r) => r,
            Err(e) => {
                error!("Error checking user: {e:?}");
                // TODO(Fido2): What error code should we return here?
                return Err(Ctap2Error::try_from(9).expect("Valid error"));
            }
        };

        Ok(UserCheck {
            presence: result.user_present,
            verification: result.user_verified,
        })
    }

    // TODO(Fido2): Do we need to return anything special here?
    fn is_presence_enabled(&self) -> bool {
        true
    }

    fn is_verification_enabled(&self) -> Option<bool> {
        Some(true)
    }
}
