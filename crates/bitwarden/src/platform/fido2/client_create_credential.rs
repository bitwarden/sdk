use std::sync::Arc;

use crate::error::{Error, Result};
use url::Url;

use super::{
    credential_store::Fido2CredentialStore,
    transaction::{Fido2Options, Fido2Transaction},
    user_interface::{Fido2UserInterface, VaultItem},
};

use passkey::{
    authenticator::Authenticator,
    client::Client,
    types::{ctap2::*, webauthn::*},
};

// AAGUID: d548826e-79b4-db40-a3d8-11116f7e8349
const AAGUID: Aaguid = Aaguid([
    0xd5, 0x48, 0x82, 0x6e, 0x79, 0xb4, 0xdb, 0x40, 0xa3, 0xd8, 0x11, 0x11, 0x6f, 0x7e, 0x83, 0x49,
]);

#[derive(Debug)]
pub struct Fido2ClientCreateCredentialRequest {
    pub options: CredentialCreationOptions,
    pub origin: String,
}

pub type Fido2CreatedPublicKeyCredential = CreatedPublicKeyCredential;

pub(crate) async fn client_create_credential(
    request: Fido2ClientCreateCredentialRequest,
    user_interface: impl Fido2UserInterface + Send + Sync,
    credential_store: impl Fido2CredentialStore + Send,
) -> Result<CreatedPublicKeyCredential> {
    log::debug!("fido2.client_create_credential, request: {:?}", request);
    let context = Arc::new(Fido2Transaction::new(
        Fido2Options::CreateCredential(clone_create_options(&request.options)),
        user_interface,
        credential_store,
    ));
    let authenticator = Authenticator::new(
        AAGUID,
        context.into_credential_store(),
        context.into_user_validation_method(),
    );
    let mut client = Client::new(authenticator.into());

    client
        .register(&Url::parse(&request.origin).unwrap(), request.options, None)
        .await
        .map_err(|error| Error::Internal("Unable to create credential".into()))
}

fn clone_create_options(options: &CredentialCreationOptions) -> CredentialCreationOptions {
    let json: String = serde_json::to_string(options).unwrap();
    serde_json::from_str(&json).unwrap()
}
