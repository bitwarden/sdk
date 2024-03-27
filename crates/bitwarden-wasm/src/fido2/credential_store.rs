extern crate console_error_panic_hook;

use bitwarden_json::{
    Fido2CredentialStore, Fido2VaultItem, FindCredentialsParams, Result, SaveCredentialParams,
};
use js_sys::Promise;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

use super::channel_wrapper::{auto_map_return, CallerChannel, ChannelWrapped};

#[wasm_bindgen]
struct JsFindCredentialsParams {
    ids: Vec<Vec<u8>>,
    rp_id: String,
}

#[wasm_bindgen]
struct JsSaveCredentialParams {
    cred: Fido2VaultItem,
    user: JsPublicKeyCredentialUserEntity,
    rp: JsPublicKeyCredentialRpEntity,
}

#[wasm_bindgen]
struct JsPublicKeyCredentialDescriptor {
    id: String,
}

#[wasm_bindgen]
struct JsPublicKeyCredentialUserEntity {
    id: String,
    name: String,
    display_name: Option<String>,
}

#[wasm_bindgen]
struct JsPublicKeyCredentialRpEntity {
    id: String,
    name: Option<String>,
}

#[wasm_bindgen]
extern "C" {
    pub type JSFido2CredentialStore;

    #[wasm_bindgen(method, js_name = "findCredentials")]
    fn find_credentials(this: &JSFido2CredentialStore, params: JsFindCredentialsParams) -> Promise;

    #[wasm_bindgen(method, js_name = "saveCredential")]
    fn save_credential(this: &JSFido2CredentialStore, params: JsSaveCredentialParams) -> Promise;
}

impl JSFido2CredentialStore {
    pub fn to_channel_wrapped(self) -> JSFido2CredentialStoreWrapper {
        // First we create a ChannelWrapped
        let wrapper = ChannelWrapped::new(self);

        // And then we create an instance of the wrapped struct, and use ChannelWrapped to
        // create a channel for each function.
        let store = JSFido2CredentialStoreWrapper {
            find_credentials: wrapper.create_channel(|inner, params| async move {
                auto_map_return(inner.find_credentials(params)).await
            }),

            save_credential: wrapper.create_channel(|inner, params| async move {
                auto_map_return(inner.save_credential(params)).await
            }),
        };

        store
    }
}

pub struct JSFido2CredentialStoreWrapper {
    // TODO: JsVaultItem -> Vec<JsVaultItem>
    find_credentials: CallerChannel<JsFindCredentialsParams, Fido2VaultItem>,
    save_credential: CallerChannel<JsSaveCredentialParams, ()>,
}

#[async_trait::async_trait]
impl Fido2CredentialStore for JSFido2CredentialStoreWrapper {
    async fn find_credentials(&self, params: FindCredentialsParams) -> Result<Vec<Fido2VaultItem>> {
        log::debug!("JSFido2CredentialStoreWrapper.find_credentials");

        // TODO: passkey-rs supports serde, so we should be able to use that instead
        let result = self
            .find_credentials
            .call(JsFindCredentialsParams {
                ids: params.ids.iter().map(|id| id.deref().clone()).collect(),
                rp_id: params.rp_id,
            })
            .await
            .unwrap();

        Ok(vec![Fido2VaultItem {
            cipher_id: result.cipher_id,
            name: result.name,
        }])
        // Ok(result
        //     .iter()
        //     .map(|item| VaultItem {
        //         cipher_id: item.cipher_id.clone(),
        //         name: item.name.clone(),
        //     })
        //     .collect())
    }

    async fn save_credential(&mut self, params: SaveCredentialParams) -> Result<()> {
        let result = self
            .save_credential
            .call(JsSaveCredentialParams {
                cred: Fido2VaultItem {
                    cipher_id: params.cred.cipher_id,
                    name: params.cred.name,
                },
                user: JsPublicKeyCredentialUserEntity {
                    id: params.user.id.into(),
                    name: params.user.name.unwrap_or("".to_owned()),
                    display_name: params.user.display_name,
                },
                rp: JsPublicKeyCredentialRpEntity {
                    id: params.rp.id,
                    name: params.rp.name,
                },
            })
            .await
            .unwrap();

        Ok(result)
    }
}
