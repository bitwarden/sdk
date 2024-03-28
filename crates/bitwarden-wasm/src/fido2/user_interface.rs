extern crate console_error_panic_hook;

use bitwarden_json::{
    Fido2UserInterface, NewCredentialParams, NewCredentialResult, PickCredentialParams,
    PickCredentialResult, Result,
};
use js_sys::Promise;
use wasm_bindgen::prelude::*;

use super::channel_wrapper::{auto_map_return, CallerChannel, ChannelWrapped};

#[wasm_bindgen]
extern "C" {
    pub type JSFido2UserInterface;

    #[wasm_bindgen(method, js_name = "confirmNewCredential")]
    fn confirm_new_credential(this: &JSFido2UserInterface, params: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = "pickCredential")]
    fn pick_credential(this: &JSFido2UserInterface, ids: Vec<String>, rp_id: String) -> Promise;

    #[wasm_bindgen(method, js_name = "checkUserVerification")]
    fn check_user_verification(this: &JSFido2UserInterface) -> Promise;

    #[wasm_bindgen(method, js_name = "checkUserPresence")]
    fn check_user_presence(this: &JSFido2UserInterface) -> Promise;

    #[wasm_bindgen(method, js_name = "isPresenceEnabled")]
    fn is_presence_enabled(this: &JSFido2UserInterface) -> bool;

    #[wasm_bindgen(method, js_name = "isVerificationEnabled")]
    fn is_verification_enabled(this: &JSFido2UserInterface) -> Option<bool>;
}

impl JSFido2UserInterface {
    pub fn to_channel_wrapped(self) -> JSFido2UserInterfaceWrapper {
        let wrapper = ChannelWrapped::new(self);

        let user_interface = JSFido2UserInterfaceWrapper {
            confirm_new_credential: wrapper.create_channel(|inner, params| {
                let js_params = serde_wasm_bindgen::to_value(&params).unwrap();
                auto_map_return(inner.confirm_new_credential(js_params))
            }),

            check_user_verification: wrapper.create_channel(|inner, _| async move {
                auto_map_return(inner.check_user_verification()).await
            }),

            check_user_presence: wrapper.create_channel(|inner, _| async move {
                auto_map_return(inner.check_user_presence()).await
            }),

            is_presence_enabled: wrapper
                .create_channel(|inner, _| async move { inner.is_presence_enabled() }),

            is_verification_enabled: wrapper
                .create_channel(|inner, _| async move { inner.is_verification_enabled() }),
        };

        user_interface
    }
}

pub struct JSFido2UserInterfaceWrapper {
    confirm_new_credential: CallerChannel<NewCredentialParams, NewCredentialResult>,
    // pick_credential // todo
    check_user_verification: CallerChannel<(), bool>,
    check_user_presence: CallerChannel<(), bool>,
    is_presence_enabled: CallerChannel<(), bool>,
    is_verification_enabled: CallerChannel<(), Option<bool>>,
}

#[async_trait::async_trait]
impl Fido2UserInterface for JSFido2UserInterfaceWrapper {
    async fn confirm_new_credential(
        &self,
        params: NewCredentialParams,
    ) -> Result<NewCredentialResult> {
        log::info!("JSFido2UserInterface.confirm_new_credential");

        self.confirm_new_credential.call(params).await
    }

    async fn pick_credential(&self, _params: PickCredentialParams) -> Result<PickCredentialResult> {
        todo!()
    }

    async fn check_user_verification(&self) -> bool {
        log::info!("JSFido2UserInterface.check_user_verification");

        self.check_user_verification.call(()).await.unwrap_or(false)
    }

    async fn check_user_presence(&self) -> bool {
        log::info!("JSFido2UserInterface.check_user_presence");

        self.check_user_presence.call(()).await.unwrap_or(false)
    }

    fn is_presence_enabled(&self) -> bool {
        log::info!("JSFido2UserInterface.is_presence_enabled");

        self.is_presence_enabled.call_blocking(()).unwrap_or(false)
    }

    fn is_verification_enabled(&self) -> Option<bool> {
        log::info!("JSFido2UserInterface.is_verification_enabled");

        self.is_verification_enabled.call_blocking(()).unwrap()
    }
}
