use std::cell::Cell;

use crate::error::Result;
use url::Url;

use super::{
    user_interface::{self, VaultItem},
    Fido2MakeCredentialUserInterface,
};

use passkey::{
    authenticator::{Authenticator, CredentialStore, UserValidationMethod},
    client::Client,
    types::{
        ctap2::{self, *},
        webauthn::*,
        Passkey,
    },
};

// TODO: Use Mutex instead Wrap
#[derive(Default)]
struct Wrap<T>(T);
unsafe impl<T> Sync for Wrap<T> {}

// Split session into attest and assert
#[derive(Default)]
struct Fido2Session<U> {
    user_interface: Wrap<U>,
    user_presence: Wrap<Cell<bool>>,
}

impl<U> Fido2Session<U>
// where
//     U: Fido2MakeCredentialUserInterface,
{
    fn new(user_interface: U) -> Self {
        Self {
            user_interface: Wrap(user_interface),
            user_presence: Wrap(Cell::new(false)),
        }
    }
}

struct Fido2CredentialStore<'a, U>
// where
//     U: Fido2MakeCredentialUserInterface,
{
    session: &'a Fido2Session<U>,
}

// CredentialStore for make_credential

#[async_trait::async_trait]
impl<'a, U> CredentialStore for Fido2CredentialStore<'a, U>
where
    U: Fido2MakeCredentialUserInterface,
{
    type PasskeyItem = VaultItem;

    async fn find_credentials(
        &self,
        ids: Option<&[PublicKeyCredentialDescriptor]>,
        rp_id: &str,
    ) -> std::result::Result<Vec<Self::PasskeyItem>, StatusCode> {
        let result: Vec<Self::PasskeyItem> = vec![];
        std::result::Result::Ok(result)
    }

    async fn save_credential(
        &mut self,
        cred: Passkey,
        user: ctap2::make_credential::PublicKeyCredentialUserEntity,
        rp: ctap2::make_credential::PublicKeyCredentialRpEntity,
    ) -> Result<(), StatusCode> {
        let result = self
            .session
            .user_interface
            .0
            .confirm_new_credential(user_interface::NewCredentialParams {
                credential_name: rp.name.unwrap_or(rp.id),
                user_name: user
                    .name
                    .or(user.display_name)
                    .unwrap_or("Unknown user".to_owned()),
                user_verification: true,
            })
            .await
            .map_err(|_| StatusCode::Ctap2(Ctap2Code::Known(Ctap2Error::NotAllowed)))
            .unwrap();

        self.session.user_presence.0.set(result.user_verified);

        std::result::Result::Ok(())
    }
}

struct Fido2UserValidationMethod<'a, U> {
    session: &'a Fido2Session<U>,
}

#[async_trait::async_trait]
impl<'a, U> UserValidationMethod for Fido2UserValidationMethod<'a, U>
where
    U: Fido2MakeCredentialUserInterface,
{
    async fn check_user_verification(&self) -> bool {
        true
    }

    async fn check_user_presence(&self) -> bool {
        self.session.user_presence.0.get()
    }

    fn is_presence_enabled(&self) -> bool {
        true
    }

    fn is_verification_enabled(&self) -> Option<bool> {
        Some(true)
    }
}

pub struct Fido2ClientCreateCredentialRequest {
    /// WebAuthn-compatible JSON string of the PublicKeyCredentialRequestOptions
    pub webauthn_json: String,
}

pub(crate) async fn client_create_credential(
    request: Fido2ClientCreateCredentialRequest,
    user_interface: impl Fido2MakeCredentialUserInterface,
) -> Result<VaultItem> {
    log::debug!("fido2.client_create_credential");
    // First create an Authenticator for the Client to use.
    let my_aaguid = Aaguid::new_empty();
    let session = Fido2Session::new(user_interface);
    let store = Fido2CredentialStore { session: &session };
    let user_validation = Fido2UserValidationMethod { session: &session };
    // Create the CredentialStore for the Authenticator.
    // Option<Passkey> is the simplest possible implementation of CredentialStore
    let my_authenticator = Authenticator::new(my_aaguid, store, user_validation);

    // Create the Client
    // If you are creating credentials, you need to declare the Client as mut
    let mut my_client = Client::new(my_authenticator.into());

    let challenge = vec![0; 32];
    let options = CredentialCreationOptions {
        public_key: PublicKeyCredentialCreationOptions {
            rp: PublicKeyCredentialRpEntity {
                id: Some("bitwarden.com".to_string()),
                name: "Bitwarden".to_string(),
            },
            user: PublicKeyCredentialUserEntity {
                id: vec![].into(),
                name: "user".to_string(),
                display_name: "User".to_string(),
            },
            challenge: challenge.into(),
            pub_key_cred_params: vec![PublicKeyCredentialParameters {
                ty: PublicKeyCredentialType::PublicKey,
                alg: coset::iana::Algorithm::ES256,
            }],
            timeout: None,
            exclude_credentials: None,
            authenticator_selection: None,
            attestation: AttestationConveyancePreference::None,
            attestation_formats: None,
            hints: None,
            extensions: None,
        },
    };

    my_client
        .register(&Url::parse("https://bitwarden.com").unwrap(), options, None)
        .await
        .unwrap();

    std::result::Result::Ok(VaultItem::new("cipher_id".to_string(), "name".to_string()))
}
