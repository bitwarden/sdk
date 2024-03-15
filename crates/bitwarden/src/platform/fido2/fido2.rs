use std::{borrow::Borrow, cell::Cell};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use url::Url;

use super::{
    user_interface::{self, Fido2GetAssertionUserInterface, VaultItem},
    Fido2MakeCredentialUserInterface,
};

use coset::Algorithm;

use passkey::{
    authenticator::{Authenticator, CredentialStore, UserValidationMethod},
    client::Client,
    types::{
        ctap2::{self, *},
        webauthn::*,
        Passkey,
    },
};

// impl TryInto<Passkey> for VaultItem {
//     type Error = WebauthnError;

//     fn try_into(self) -> Result<Passkey, Self::Error> {
//         todo!();
//     }
// }

impl From<VaultItem> for Passkey {
    fn from(value: VaultItem) -> Self {
        todo!()
    }
}

fn uuid_raw_to_standard_format(uuid: &Vec<u8>) -> String {
    let mut uuid_str = String::with_capacity(36);
    uuid_str.push_str(&format!(
        "{:02X}{:02X}{:02X}{:02X}-",
        uuid[0], uuid[1], uuid[2], uuid[3]
    ));
    uuid_str.push_str(&format!("{:02X}{:02X}-", uuid[4], uuid[5]));
    uuid_str.push_str(&format!("{:02X}{:02X}-", uuid[6], uuid[7]));
    uuid_str.push_str(&format!("{:02X}{:02X}-", uuid[8], uuid[9]));
    for i in 10..uuid.len() {
        uuid_str.push_str(&format!("{:02X}", uuid[i]));
    }
    uuid_str
}

// CredentialStore for get_assertion

// #[async_trait::async_trait]
// impl<'a, U> CredentialStore for Fido2CredentialStore<'a, U>
// where
//     U: Fido2GetAssertionUserInterface,
// {
//     type PasskeyItem = VaultItem;

//     async fn find_credentials(
//         &self,
//         ids: Option<&[PublicKeyCredentialDescriptor]>,
//         rp_id: &str,
//     ) -> Result<Vec<Self::PasskeyItem>, StatusCode> {
//         let id_strs = ids
//             .map(|ids| {
//                 ids.iter()
//                     .map(|id| uuid_raw_to_standard_format(&id.id))
//                     .collect::<Vec<_>>()
//             })
//             .unwrap_or_default();
//         let result = self
//             .session
//             .user_interface
//             .0
//             .pick_credential(id_strs, rp_id)
//             .await;

//         match result {
//             Ok(item) => Ok(vec![item]),
//             Err(e) => Err(StatusCode::Ctap2(Ctap2Code::Known(Ctap2Error::NotAllowed))),
//         }
//     }

//     async fn save_credential(
//         &mut self,
//         cred: Passkey,
//         user: ctap2::make_credential::PublicKeyCredentialUserEntity,
//         rp: ctap2::make_credential::PublicKeyCredentialRpEntity,
//     ) -> Result<(), StatusCode> {
//         todo!();
//     }
// }

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fido2ClientGetAssertionRequest {
    /// WebAuthn-compatible JSON string of the PublicKeyCredentialRequestOptions
    pub webauthn_json: String,
}

pub(crate) async fn client_get_assertion(
    request: Fido2ClientGetAssertionRequest,
    user_interface: impl Fido2GetAssertionUserInterface,
) -> Result<String> {
    // log::debug!("fido2.client_get_assertion");
    // // First create an Authenticator for the Client to use.
    // let my_aaguid = Aaguid::new_empty();
    // let session = Fido2Session::new(user_interface);
    // let store = Fido2CredentialStore { session: &session };
    // let user_validation = Fido2UserValidationMethod { session: &session };
    // // Create the CredentialStore for the Authenticator.
    // // Option<Passkey> is the simplest possible implementation of CredentialStore
    // let my_authenticator = Authenticator::new(my_aaguid, store, user_validation);

    // // Create the Client
    // // If you are creating credentials, you need to declare the Client as mut
    // let my_client = Client::new(my_authenticator.into());

    // let challenge = vec![0; 32];
    // let options = CredentialRequestOptions {
    //     public_key: PublicKeyCredentialRequestOptions {
    //         allow_credentials: None,
    //         attestation: AttestationConveyancePreference::None,
    //         challenge: challenge.into(),
    //         timeout: None,
    //         rp_id: Some("bitwarden.com".to_string()),
    //         user_verification: UserVerificationRequirement::Preferred,
    //         hints: None,
    //         attestation_formats: None,
    //         extensions: None,
    //     },
    // };

    // my_client
    //     .authenticate(&Url::parse("https://bitwarden.com").unwrap(), options, None)
    //     .await
    //     .unwrap();

    std::result::Result::Ok("client_get_assertion".to_string())
}
