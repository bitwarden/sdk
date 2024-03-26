extern crate console_error_panic_hook;

use std::ops::Deref;

use bitwarden_json::{
    Fido2CredentialStore, FindCredentialsParams, Result, SaveCredentialParams, VaultItem,
};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::channel_wrapper::{CallerChannel, ChannelWrapped};

#[wasm_bindgen]
struct JsFindCredentialsParams {
    ids: Vec<Vec<u8>>,
    rp_id: String,
}

#[wasm_bindgen]
struct JsSaveCredentialParams {
    cred: JsVaultItem,
    user: JsPublicKeyCredentialUserEntity,
    rp: JsPublicKeyCredentialRpEntity,
}

#[wasm_bindgen]
struct JsPublicKeyCredentialDescriptor {
    id: String,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
struct JsVaultItem {
    cipher_id: String,
    name: String,
}

#[wasm_bindgen]
struct JsPublicKeyCredentialUserEntity {
    id: String,
    name: String,
    display_name: String,
}

#[wasm_bindgen]
struct JsPublicKeyCredentialRpEntity {
    id: String,
    name: String,
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
                let promise = inner.find_credentials(params);
                let result = wasm_bindgen_futures::JsFuture::from(promise).await;
                let result: JsVaultItem = serde_wasm_bindgen::from_value(result.unwrap()).unwrap();
                result
            }),
        };

        store
    }
}

pub struct JSFido2CredentialStoreWrapper {
    // TODO: JsVaultItem -> Vec<JsVaultItem>
    find_credentials: CallerChannel<JsFindCredentialsParams, JsVaultItem>,
}

#[async_trait::async_trait]
impl Fido2CredentialStore for JSFido2CredentialStoreWrapper {
    async fn find_credentials(&self, params: FindCredentialsParams) -> Result<Vec<VaultItem>> {
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

        Ok(vec![VaultItem {
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
        todo!()
    }
}
