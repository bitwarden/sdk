extern crate console_error_panic_hook;

use bitwarden_json::{
    Fido2UserInterface, NewCredentialParams, NewCredentialResult, PickCredentialParams,
    PickCredentialResult, Result,
};
use js_sys::Promise;
use wasm_bindgen::prelude::*;

use super::channel_wrapper::{CallerChannel, ChannelWrapped};

#[wasm_bindgen]
struct JsNewCredentialParams {
    credential_name: String,
    user_name: String,
}

#[wasm_bindgen]
extern "C" {
    pub type JSFido2UserInterface;

    #[wasm_bindgen(method, js_name = "confirmNewCredential")]
    fn confirm_new_credential(
        this: &JSFido2UserInterface,
        params: JsNewCredentialParams,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = "pickCredential")]
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
        let wrapper = ChannelWrapped::new(self);

        let user_interface = JSFido2UserInterfaceWrapper {
            confirm_new_credential: wrapper.create_channel(|inner, params| async move {
                let promise = inner.confirm_new_credential(params);
                let result = wasm_bindgen_futures::JsFuture::from(promise).await;
                let result: NewCredentialResult =
                    serde_wasm_bindgen::from_value(result.unwrap()).unwrap();
                result
            }),
        };

        user_interface
    }
}

pub struct JSFido2UserInterfaceWrapper {
    confirm_new_credential: CallerChannel<JsNewCredentialParams, NewCredentialResult>,
}

#[async_trait::async_trait]
impl Fido2UserInterface for JSFido2UserInterfaceWrapper {
    async fn confirm_new_credential(
        &self,
        params: NewCredentialParams,
    ) -> Result<NewCredentialResult> {
        log::info!("JSFido2UserInterface.confirm_new_credential");

        let result = self
            .confirm_new_credential
            .call(JsNewCredentialParams {
                credential_name: params.credential_name,
                user_name: params.user_name,
            })
            .await
            .unwrap()
            .unwrap(); // TODO: Map to thread crashed result

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
