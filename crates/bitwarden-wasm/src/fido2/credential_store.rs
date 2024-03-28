extern crate console_error_panic_hook;

use bitwarden_json::{
    Fido2CredentialStore, Fido2VaultItem, FindCredentialsParams, Result, SaveCredentialParams,
};
use js_sys::Promise;
use wasm_bindgen::prelude::*;

use super::channel_wrapper::{auto_map_return, CallerChannel, ChannelWrapped};

#[wasm_bindgen]
extern "C" {
    pub type JSFido2CredentialStore;

    #[wasm_bindgen(method, js_name = "findCredentials")]
    fn find_credentials(this: &JSFido2CredentialStore, params: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = "saveCredential")]
    fn save_credential(this: &JSFido2CredentialStore, params: JsValue) -> Promise;
}

impl JSFido2CredentialStore {
    pub fn to_channel_wrapped(self) -> JSFido2CredentialStoreWrapper {
        // First we create a ChannelWrapped
        let wrapper = ChannelWrapped::new(self);

        // And then we create an instance of the wrapped struct, and use ChannelWrapped to
        // create a channel for each function.
        let store = JSFido2CredentialStoreWrapper {
            find_credentials: wrapper.create_channel(|inner, params| async move {
                let js_params = serde_wasm_bindgen::to_value(&params).unwrap();
                auto_map_return(inner.find_credentials(js_params)).await
            }),

            save_credential: wrapper.create_channel(|inner, params| async move {
                let js_params = serde_wasm_bindgen::to_value(&params).unwrap();
                auto_map_return(inner.save_credential(js_params)).await
            }),
        };

        store
    }
}

pub struct JSFido2CredentialStoreWrapper {
    // TODO: JsVaultItem -> Vec<JsVaultItem>
    find_credentials: CallerChannel<FindCredentialsParams, Vec<Fido2VaultItem>>,
    save_credential: CallerChannel<SaveCredentialParams, ()>,
}

#[async_trait::async_trait]
impl Fido2CredentialStore for JSFido2CredentialStoreWrapper {
    async fn find_credentials(&self, params: FindCredentialsParams) -> Result<Vec<Fido2VaultItem>> {
        self.find_credentials.call(params).await
    }

    async fn save_credential(&mut self, params: SaveCredentialParams) -> Result<()> {
        self.save_credential.call(params).await
    }
}
