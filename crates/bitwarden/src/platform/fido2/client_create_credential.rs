use std::sync::Arc;

use crate::error::Result;
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

pub(crate) async fn client_create_credential(
    request: Fido2ClientCreateCredentialRequest,
    user_interface: impl Fido2UserInterface + Send + Sync,
    credential_store: impl Fido2CredentialStore + Send,
) -> Result<VaultItem> {
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
        .unwrap();

    std::result::Result::Ok(VaultItem::new("cipher_id".to_string(), "name".to_string()))
}

// let challenge = vec![0; 32];
// let options = CredentialCreationOptions {
//     public_key: PublicKeyCredentialCreationOptions {
//         rp: PublicKeyCredentialRpEntity {
//             id: Some("bitwarden.com".to_string()),
//             name: "Bitwarden".to_string(),
//         },
//         user: PublicKeyCredentialUserEntity {
//             id: vec![].into(),
//             name: "user".to_string(),
//             display_name: "User".to_string(),
//         },
//         challenge: challenge.into(),
//         pub_key_cred_params: vec![PublicKeyCredentialParameters {
//             ty: PublicKeyCredentialType::PublicKey,
//             alg: coset::iana::Algorithm::ES256,
//         }],
//         timeout: None,
//         exclude_credentials: None,
//         authenticator_selection: None,
//         attestation: AttestationConveyancePreference::None,
//         attestation_formats: None,
//         hints: None,
//         extensions: None,
//     },
// };

fn clone_create_options(options: &CredentialCreationOptions) -> CredentialCreationOptions {
    let json: String = serde_json::to_string(options).unwrap();
    serde_json::from_str(&json).unwrap()
}
