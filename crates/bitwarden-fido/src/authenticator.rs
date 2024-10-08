use std::sync::Mutex;

use bitwarden_core::{Client, VaultLocked};
use bitwarden_crypto::CryptoError;
use bitwarden_vault::{CipherError, CipherView};
use itertools::Itertools;
use log::error;
use passkey::{
    authenticator::{Authenticator, DiscoverabilitySupport, StoreInfo, UIHint, UserCheck},
    types::{
        ctap2::{self, Ctap2Error, StatusCode, VendorError},
        Passkey,
    },
};
use thiserror::Error;

use super::{
    try_from_credential_new_view, types::*, CheckUserOptions, CipherViewContainer,
    Fido2CredentialStore, Fido2UserInterface, SelectedCredential, UnknownEnum, AAGUID,
};
use crate::{
    fill_with_credential, string_to_guid_bytes, try_from_credential_full, Fido2CallbackError,
    FillCredentialError, InvalidGuid,
};

#[derive(Debug, Error)]
pub enum GetSelectedCredentialError {
    #[error("No selected credential available")]
    NoSelectedCredential,
    #[error("No fido2 credentials found")]
    NoCredentialFound,

    #[error(transparent)]
    VaultLocked(#[from] VaultLocked),
    #[error(transparent)]
    CipherError(#[from] CipherError),
}

#[derive(Debug, Error)]
pub enum MakeCredentialError {
    #[error(transparent)]
    PublicKeyCredentialParametersError(#[from] PublicKeyCredentialParametersError),
    #[error(transparent)]
    UnknownEnum(#[from] UnknownEnum),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("Missing attested_credential_data")]
    MissingAttestedCredentialData,
    #[error("make_credential error: {0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum GetAssertionError {
    #[error(transparent)]
    UnknownEnum(#[from] UnknownEnum),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    GetSelectedCredentialError(#[from] GetSelectedCredentialError),
    #[error(transparent)]
    InvalidGuid(#[from] InvalidGuid),
    #[error("missing user")]
    MissingUser,
    #[error("get_assertion error: {0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum SilentlyDiscoverCredentialsError {
    #[error(transparent)]
    CipherError(#[from] CipherError),
    #[error(transparent)]
    VaultLocked(#[from] VaultLocked),
    #[error(transparent)]
    InvalidGuid(#[from] InvalidGuid),
    #[error(transparent)]
    Fido2CallbackError(#[from] Fido2CallbackError),
    #[error(transparent)]
    FromCipherViewError(#[from] Fido2CredentialAutofillViewError),
}

#[derive(Debug, Error)]
pub enum CredentialsForAutofillError {
    #[error(transparent)]
    CipherError(#[from] CipherError),
    #[error(transparent)]
    VaultLocked(#[from] VaultLocked),
    #[error(transparent)]
    InvalidGuid(#[from] InvalidGuid),
    #[error(transparent)]
    Fido2CallbackError(#[from] Fido2CallbackError),
    #[error(transparent)]
    FromCipherViewError(#[from] Fido2CredentialAutofillViewError),
}

pub struct Fido2Authenticator<'a> {
    pub client: &'a Client,
    pub user_interface: &'a dyn Fido2UserInterface,
    pub credential_store: &'a dyn Fido2CredentialStore,

    pub(crate) selected_cipher: Mutex<Option<CipherView>>,
    pub(crate) requested_uv: Mutex<Option<UV>>,
}

impl<'a> Fido2Authenticator<'a> {
    pub fn new(
        client: &'a Client,
        user_interface: &'a dyn Fido2UserInterface,
        credential_store: &'a dyn Fido2CredentialStore,
    ) -> Fido2Authenticator<'a> {
        Fido2Authenticator {
            client,
            user_interface,
            credential_store,
            selected_cipher: Mutex::new(None),
            requested_uv: Mutex::new(None),
        }
    }

    pub async fn make_credential(
        &mut self,
        request: MakeCredentialRequest,
    ) -> Result<MakeCredentialResult, MakeCredentialError> {
        // Insert the received UV to be able to return it later in check_user
        self.requested_uv
            .get_mut()
            .expect("Mutex is not poisoned")
            .replace(request.options.uv);

        let mut authenticator = self.get_authenticator(true);

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
                    rk: request.options.rk,
                    up: true,
                    uv: self.convert_requested_uv(request.options.uv).await,
                },
                pin_auth: None,
                pin_protocol: None,
            })
            .await;

        let response = match response {
            Ok(x) => x,
            Err(e) => return Err(MakeCredentialError::Other(format!("{e:?}"))),
        };

        let attestation_object = response.as_bytes().to_vec();
        let authenticator_data = response.auth_data.to_vec();
        let attested_credential_data = response
            .auth_data
            .attested_credential_data
            .ok_or(MakeCredentialError::MissingAttestedCredentialData)?;
        let credential_id = attested_credential_data.credential_id().to_vec();

        Ok(MakeCredentialResult {
            authenticator_data,
            attestation_object,
            credential_id,
        })
    }

    pub async fn get_assertion(
        &mut self,
        request: GetAssertionRequest,
    ) -> Result<GetAssertionResult, GetAssertionError> {
        // Insert the received UV to be able to return it later in check_user
        self.requested_uv
            .get_mut()
            .expect("Mutex is not poisoned")
            .replace(request.options.uv);

        let mut authenticator = self.get_authenticator(false);

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
                    uv: self.convert_requested_uv(request.options.uv).await,
                },
                pin_auth: None,
                pin_protocol: None,
            })
            .await;

        let response = match response {
            Ok(x) => x,
            Err(e) => return Err(GetAssertionError::Other(format!("{e:?}"))),
        };

        let selected_credential = self.get_selected_credential()?;
        let authenticator_data = response.auth_data.to_vec();
        let credential_id = string_to_guid_bytes(&selected_credential.credential.credential_id)?;

        Ok(GetAssertionResult {
            credential_id,
            authenticator_data,
            signature: response.signature.into(),
            user_handle: response
                .user
                .ok_or(GetAssertionError::MissingUser)?
                .id
                .into(),
            selected_credential,
        })
    }

    pub async fn silently_discover_credentials(
        &mut self,
        rp_id: String,
    ) -> Result<Vec<Fido2CredentialAutofillView>, SilentlyDiscoverCredentialsError> {
        let crypto = self.client.internal.get_crypto_service();
        let result = self.credential_store.find_credentials(None, rp_id).await?;

        result
            .into_iter()
            .map(
                |cipher| -> Result<Vec<Fido2CredentialAutofillView>, SilentlyDiscoverCredentialsError> {
                    Ok(Fido2CredentialAutofillView::from_cipher_view(&cipher, &mut crypto.context())?)
                },
            )
            .flatten_ok()
            .collect()
    }

    /// Returns all Fido2 credentials that can be used for autofill, in a view
    /// tailored for integration with OS autofill systems.
    pub async fn credentials_for_autofill(
        &mut self,
    ) -> Result<Vec<Fido2CredentialAutofillView>, CredentialsForAutofillError> {
        let crypto = self.client.internal.get_crypto_service();
        let all_credentials = self.credential_store.all_credentials().await?;

        all_credentials
            .into_iter()
            .map(
                |cipher| -> Result<Vec<Fido2CredentialAutofillView>, CredentialsForAutofillError> {
                    Ok(Fido2CredentialAutofillView::from_cipher_view(
                        &cipher,
                        &mut crypto.context(),
                    )?)
                },
            )
            .flatten_ok()
            .collect()
    }

    pub(super) fn get_authenticator(
        &self,
        create_credential: bool,
    ) -> Authenticator<CredentialStoreImpl, UserValidationMethodImpl> {
        Authenticator::new(
            AAGUID,
            CredentialStoreImpl {
                authenticator: self,
                create_credential,
            },
            UserValidationMethodImpl {
                authenticator: self,
            },
        )
    }

    async fn convert_requested_uv(&self, uv: UV) -> bool {
        let verification_enabled = self.user_interface.is_verification_enabled().await;
        match (uv, verification_enabled) {
            (UV::Preferred, true) => true,
            (UV::Preferred, false) => false,
            (UV::Required, _) => true,
            (UV::Discouraged, _) => false,
        }
    }

    pub(super) fn get_selected_credential(
        &self,
    ) -> Result<SelectedCredential, GetSelectedCredentialError> {
        let crypto = self.client.internal.get_crypto_service();

        let cipher = self
            .selected_cipher
            .lock()
            .expect("Mutex is not poisoned")
            .clone()
            .ok_or(GetSelectedCredentialError::NoSelectedCredential)?;

        let creds = cipher.decrypt_fido2_credentials(&mut crypto.context())?;

        let credential = creds
            .first()
            .ok_or(GetSelectedCredentialError::NoCredentialFound)?
            .clone();

        Ok(SelectedCredential { cipher, credential })
    }
}

pub(super) struct CredentialStoreImpl<'a> {
    authenticator: &'a Fido2Authenticator<'a>,
    create_credential: bool,
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
        #[derive(Debug, Error)]
        enum InnerError {
            #[error(transparent)]
            VaultLocked(#[from] VaultLocked),
            #[error(transparent)]
            CipherError(#[from] CipherError),
            #[error(transparent)]
            CryptoError(#[from] CryptoError),
            #[error(transparent)]
            Fido2CallbackError(#[from] Fido2CallbackError),
        }

        // This is just a wrapper around the actual implementation to allow for ? error handling
        async fn inner(
            this: &CredentialStoreImpl<'_>,
            ids: Option<&[passkey::types::webauthn::PublicKeyCredentialDescriptor]>,
            rp_id: &str,
        ) -> Result<Vec<CipherViewContainer>, InnerError> {
            let ids: Option<Vec<Vec<u8>>> =
                ids.map(|ids| ids.iter().map(|id| id.id.clone().into()).collect());

            let ciphers = this
                .authenticator
                .credential_store
                .find_credentials(ids, rp_id.to_string())
                .await?;

            let crypto = this.authenticator.client.internal.get_crypto_service();

            // Remove any that don't have Fido2 credentials
            let creds: Vec<_> = ciphers
                .into_iter()
                .filter(|c| {
                    c.login
                        .as_ref()
                        .and_then(|l| l.fido2_credentials.as_ref())
                        .is_some()
                })
                .collect();

            // When using the credential for authentication we have to ask the user to pick one.
            if this.create_credential {
                Ok(creds
                    .into_iter()
                    .map(|c| CipherViewContainer::new(c, &mut crypto.context()))
                    .collect::<Result<_, _>>()?)
            } else {
                let picked = this
                    .authenticator
                    .user_interface
                    .pick_credential_for_authentication(creds)
                    .await?;

                // Store the selected credential for later use
                this.authenticator
                    .selected_cipher
                    .lock()
                    .expect("Mutex is not poisoned")
                    .replace(picked.clone());

                Ok(vec![CipherViewContainer::new(
                    picked,
                    &mut crypto.context(),
                )?])
            }
        }

        inner(self, ids, rp_id).await.map_err(|e| {
            error!("Error finding credentials: {e:?}");
            VendorError::try_from(0xF0)
                .expect("Valid vendor error code")
                .into()
        })
    }

    async fn save_credential(
        &mut self,
        cred: Passkey,
        user: passkey::types::ctap2::make_credential::PublicKeyCredentialUserEntity,
        rp: passkey::types::ctap2::make_credential::PublicKeyCredentialRpEntity,
        options: passkey::types::ctap2::get_assertion::Options,
    ) -> Result<(), StatusCode> {
        #[derive(Debug, Error)]
        enum InnerError {
            #[error(transparent)]
            VaultLocked(#[from] VaultLocked),
            #[error(transparent)]
            FillCredentialError(#[from] FillCredentialError),
            #[error(transparent)]
            CipherError(#[from] CipherError),
            #[error(transparent)]
            CryptoError(#[from] CryptoError),
            #[error(transparent)]
            Fido2CallbackError(#[from] Fido2CallbackError),

            #[error("No selected credential available")]
            NoSelectedCredential,
        }

        // This is just a wrapper around the actual implementation to allow for ? error handling
        async fn inner(
            this: &mut CredentialStoreImpl<'_>,
            cred: Passkey,
            user: passkey::types::ctap2::make_credential::PublicKeyCredentialUserEntity,
            rp: passkey::types::ctap2::make_credential::PublicKeyCredentialRpEntity,
            options: passkey::types::ctap2::get_assertion::Options,
        ) -> Result<(), InnerError> {
            let cred = try_from_credential_full(cred, user, rp, options)?;

            // Get the previously selected cipher and add the new credential to it
            let mut selected: CipherView = this
                .authenticator
                .selected_cipher
                .lock()
                .expect("Mutex is not poisoned")
                .clone()
                .ok_or(InnerError::NoSelectedCredential)?;

            let crypto = this.authenticator.client.internal.get_crypto_service();

            selected.set_new_fido2_credentials(&mut crypto.context(), vec![cred])?;

            // Store the updated credential for later use
            this.authenticator
                .selected_cipher
                .lock()
                .expect("Mutex is not poisoned")
                .replace(selected.clone());

            // Encrypt the updated cipher before sending it to the clients to be stored
            let encrypted = crypto.encrypt(selected)?;

            this.authenticator
                .credential_store
                .save_credential(encrypted)
                .await?;

            Ok(())
        }

        inner(self, cred, user, rp, options).await.map_err(|e| {
            error!("Error saving credential: {e:?}");
            VendorError::try_from(0xF1)
                .expect("Valid vendor error code")
                .into()
        })
    }

    async fn update_credential(&mut self, cred: Passkey) -> Result<(), StatusCode> {
        #[derive(Debug, Error)]
        enum InnerError {
            #[error(transparent)]
            VaultLocked(#[from] VaultLocked),
            #[error(transparent)]
            InvalidGuid(#[from] InvalidGuid),
            #[error("Credential ID does not match selected credential")]
            CredentialIdMismatch,
            #[error(transparent)]
            FillCredentialError(#[from] FillCredentialError),
            #[error(transparent)]
            CipherError(#[from] CipherError),
            #[error(transparent)]
            CryptoError(#[from] CryptoError),
            #[error(transparent)]
            Fido2CallbackError(#[from] Fido2CallbackError),
            #[error(transparent)]
            GetSelectedCredentialError(#[from] GetSelectedCredentialError),
        }

        // This is just a wrapper around the actual implementation to allow for ? error handling
        async fn inner(
            this: &mut CredentialStoreImpl<'_>,
            cred: Passkey,
        ) -> Result<(), InnerError> {
            let crypto = this.authenticator.client.internal.get_crypto_service();

            // Get the previously selected cipher and update the credential
            let selected = this.authenticator.get_selected_credential()?;

            // Check that the provided credential ID matches the selected credential
            let new_id: &Vec<u8> = &cred.credential_id;
            let selected_id = string_to_guid_bytes(&selected.credential.credential_id)?;
            if new_id != &selected_id {
                return Err(InnerError::CredentialIdMismatch);
            }

            let cred = fill_with_credential(&selected.credential, cred)?;

            let mut selected = selected.cipher;
            selected.set_new_fido2_credentials(&mut crypto.context(), vec![cred])?;

            // Store the updated credential for later use
            this.authenticator
                .selected_cipher
                .lock()
                .expect("Mutex is not poisoned")
                .replace(selected.clone());

            // Encrypt the updated cipher before sending it to the clients to be stored
            let encrypted = crypto.encrypt(selected)?;

            this.authenticator
                .credential_store
                .save_credential(encrypted)
                .await?;

            Ok(())
        }

        inner(self, cred).await.map_err(|e| {
            error!("Error updating credential: {e:?}");
            VendorError::try_from(0xF2)
                .expect("Valid vendor error code")
                .into()
        })
    }

    async fn get_info(&self) -> StoreInfo {
        StoreInfo {
            discoverability: DiscoverabilitySupport::Full,
        }
    }
}

#[async_trait::async_trait]
impl passkey::authenticator::UserValidationMethod for UserValidationMethodImpl<'_> {
    type PasskeyItem = CipherViewContainer;

    async fn check_user<'a>(
        &self,
        hint: UIHint<'a, Self::PasskeyItem>,
        presence: bool,
        _verification: bool,
    ) -> Result<UserCheck, Ctap2Error> {
        let verification = self
            .authenticator
            .requested_uv
            .lock()
            .expect("Mutex is not poisoned")
            .ok_or(Ctap2Error::UserVerificationInvalid)?;

        let options = CheckUserOptions {
            require_presence: presence,
            require_verification: verification.into(),
        };

        let result = match hint {
            UIHint::RequestNewCredential(user, rp) => {
                let new_credential = try_from_credential_new_view(user, rp)
                    .map_err(|_| Ctap2Error::InvalidCredential)?;

                let (cipher_view, user_check) = self
                    .authenticator
                    .user_interface
                    .check_user_and_pick_credential_for_creation(options, new_credential)
                    .await
                    .map_err(|_| Ctap2Error::OperationDenied)?;

                self.authenticator
                    .selected_cipher
                    .lock()
                    .expect("Mutex is not poisoned")
                    .replace(cipher_view);

                Ok(user_check)
            }
            _ => {
                self.authenticator
                    .user_interface
                    .check_user(options, map_ui_hint(hint))
                    .await
            }
        };

        let result = result.map_err(|e| {
            error!("Error checking user: {e:?}");
            Ctap2Error::UserVerificationInvalid
        })?;

        Ok(UserCheck {
            presence: result.user_present,
            verification: result.user_verified,
        })
    }

    async fn is_presence_enabled(&self) -> bool {
        true
    }

    async fn is_verification_enabled(&self) -> Option<bool> {
        Some(
            self.authenticator
                .user_interface
                .is_verification_enabled()
                .await,
        )
    }
}

fn map_ui_hint(hint: UIHint<'_, CipherViewContainer>) -> UIHint<'_, CipherView> {
    use UIHint::*;
    match hint {
        InformExcludedCredentialFound(c) => InformExcludedCredentialFound(&c.cipher),
        InformNoCredentialsFound => InformNoCredentialsFound,
        RequestNewCredential(u, r) => RequestNewCredential(u, r),
        RequestExistingCredential(c) => RequestExistingCredential(&c.cipher),
    }
}
