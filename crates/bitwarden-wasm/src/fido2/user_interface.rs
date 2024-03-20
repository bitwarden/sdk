extern crate console_error_panic_hook;

use bitwarden_json::{
    Fido2UserInterface, NewCredentialParams, NewCredentialResult, PickCredentialParams,
    PickCredentialResult, Result, VaultItem,
};
use futures::{SinkExt, StreamExt};
use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct JsNewCredentialParams {
    credential_name: String,
    user_name: String,
}

#[wasm_bindgen]
extern "C" {
    pub type JSFido2UserInterface;

    #[wasm_bindgen(method)]
    fn confirm_new_credential(
        this: &JSFido2UserInterface,
        params: JsNewCredentialParams,
    ) -> Promise;

    #[wasm_bindgen(method)]
    fn pick_credential(this: &JSFido2UserInterface, ids: Vec<String>, rp_id: String) -> Promise;

    #[wasm_bindgen(method)]
    fn check_user_verification(this: &JSFido2UserInterface) -> Promise;

    #[wasm_bindgen(method)]
    fn check_user_presence(this: &JSFido2UserInterface) -> Promise;

    #[wasm_bindgen(method)]
    fn is_presence_enabled(this: &JSFido2UserInterface) -> bool;

    #[wasm_bindgen(method)]
    fn is_verification_enabled(this: &JSFido2UserInterface) -> Option<bool>;
}

impl JSFido2UserInterface {
    pub fn to_channel_wrapped(self) -> JSFido2UserInterfaceWrapper {
        let (tx_wrapper, mut rx_task) = futures::channel::mpsc::channel::<JsNewCredentialParams>(1);
        let (mut tx_task, rx_wrapper) = futures::channel::mpsc::channel::<NewCredentialResult>(1);

        // Spawn the local task which just waits until we receive input from the trait, note that this is not Send but we don't care
        wasm_bindgen_futures::spawn_local(async move {
            let params = rx_task.next().await.unwrap();

            let result_promise = self.confirm_new_credential(JsNewCredentialParams {
                credential_name: params.credential_name,
                user_name: params.user_name,
            });
            let result = wasm_bindgen_futures::JsFuture::from(result_promise).await;
            let result = result.unwrap().as_string().unwrap();

            tx_task
                .send(NewCredentialResult {
                    vault_item: VaultItem::new(result, "test".to_string()),
                })
                .await
                .unwrap();
        });

        JSFido2UserInterfaceWrapper {
            confirm_new_credential: (
                async_lock::Mutex::new(tx_wrapper),
                async_lock::Mutex::new(rx_wrapper),
            ),
        }
    }
}

pub struct JSFido2UserInterfaceWrapper {
    confirm_new_credential: (
        async_lock::Mutex<futures::channel::mpsc::Sender<JsNewCredentialParams>>,
        async_lock::Mutex<futures::channel::mpsc::Receiver<NewCredentialResult>>,
    ),
}

#[async_trait::async_trait]
impl Fido2UserInterface for JSFido2UserInterfaceWrapper {
    async fn confirm_new_credential(
        &self,
        params: NewCredentialParams,
    ) -> Result<NewCredentialResult> {
        log::debug!("JSFido2MakeCredentialUserInterface.pick_credential");

        self.confirm_new_credential
            .0
            .lock()
            .await
            .send(JsNewCredentialParams {
                credential_name: params.credential_name,
                user_name: params.user_name,
            })
            .await
            .unwrap();

        let result = self
            .confirm_new_credential
            .1
            .lock()
            .await
            .next()
            .await
            .unwrap();

        Ok(result)
    }

    async fn pick_credential(&self, _params: PickCredentialParams) -> Result<PickCredentialResult> {
        todo!()
    }

    async fn check_user_verification(&self) -> bool {
        todo!()
    }

    async fn check_user_presence(&self) -> bool {
        todo!()
    }

    fn is_presence_enabled(&self) -> bool {
        todo!()
    }

    fn is_verification_enabled(&self) -> Option<bool> {
        todo!()
    }
}
