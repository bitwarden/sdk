extern crate console_error_panic_hook;

use bitwarden_json::{
    Fido2CredentialStore, FindCredentialsParams, Result, SaveCredentialParams, VaultItem,
};
use futures::{SinkExt, StreamExt};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct JsFindCredentialsParams {
    ids: Vec<JsPublicKeyCredentialDescriptor>,
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

    #[wasm_bindgen(method)]
    fn find_credentials(this: &JSFido2CredentialStore, params: JsFindCredentialsParams) -> Promise;

    #[wasm_bindgen(method)]
    fn save_credential(this: &JSFido2CredentialStore, params: JsSaveCredentialParams) -> Promise;
}

impl JSFido2CredentialStore {
    pub fn to_channel_wrapped(self) -> JSFido2CredentialStoreWrapper {
        let (tx_wrapper, mut rx_task) =
            futures::channel::mpsc::channel::<JsFindCredentialsParams>(1);
        let (mut tx_task, rx_wrapper) = futures::channel::mpsc::channel::<Vec<JsVaultItem>>(1);

        // Spawn the local task which just waits until we receive input from the trait, note that this is not Send but we don't care
        wasm_bindgen_futures::spawn_local(async move {
            let params = rx_task.next().await.unwrap();

            let result_promise = self.find_credentials(JsFindCredentialsParams {
                ids: params.ids,
                rp_id: params.rp_id,
            });
            let result = wasm_bindgen_futures::JsFuture::from(result_promise).await;
            let result = serde_wasm_bindgen::from_value(result.unwrap()).unwrap();

            tx_task.send(result).await.unwrap();
        });

        JSFido2CredentialStoreWrapper {
            find_credentials: (
                async_lock::Mutex::new(tx_wrapper),
                async_lock::Mutex::new(rx_wrapper),
            ),
        }
    }
}

pub struct JSFido2CredentialStoreWrapper {
    find_credentials: (
        async_lock::Mutex<futures::channel::mpsc::Sender<JsFindCredentialsParams>>,
        async_lock::Mutex<futures::channel::mpsc::Receiver<Vec<JsVaultItem>>>,
    ),
}

#[async_trait::async_trait]
impl Fido2CredentialStore for JSFido2CredentialStoreWrapper {
    async fn find_credentials(&self, params: FindCredentialsParams) -> Result<Vec<VaultItem>> {
        log::debug!("JSFido2CredentialStoreWrapper.find_credentials");

        // TODO: passkey-rs supports serde, so we should be able to use that instead
        self.find_credentials
            .0
            .lock()
            .await
            .send(JsFindCredentialsParams {
                ids: params
                    .ids
                    .unwrap_or_default()
                    .iter()
                    .map(|descriptor| JsPublicKeyCredentialDescriptor {
                        id: descriptor.id.clone().into(),
                    })
                    .collect(),
                rp_id: params.rp_id,
            })
            .await
            .unwrap();

        let result = self.find_credentials.1.lock().await.next().await.unwrap();

        Ok(result
            .iter()
            .map(|item| VaultItem {
                cipher_id: item.cipher_id.clone(),
                name: item.name.clone(),
            })
            .collect())
    }

    async fn save_credential(&mut self, params: SaveCredentialParams) -> Result<()> {
        todo!()
    }
}
