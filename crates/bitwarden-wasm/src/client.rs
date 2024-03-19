extern crate console_error_panic_hook;
use std::{cell::Cell, fmt::Result, process::Output, rc::Rc, sync::RwLock};

use bitwarden_json::{
    client::Client as JsonClient, Fido2ClientCreateCredentialRequest, Fido2CredentialStore,
    Fido2UserInterface, NewCredentialParams, NewCredentialResult, VaultItem,
};
use futures::{SinkExt, StreamExt};
use js_sys::{Object, Promise};
use log::Level;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

fn convert_level(level: LogLevel) -> Level {
    match level {
        LogLevel::Trace => Level::Trace,
        LogLevel::Debug => Level::Debug,
        LogLevel::Info => Level::Info,
        LogLevel::Warn => Level::Warn,
        LogLevel::Error => Level::Error,
    }
}

#[wasm_bindgen]
extern "C" {
    pub type JSFido2MakeCredentialUserInterface;

    #[wasm_bindgen(structural, method)]
    pub fn confirm_new_credential(
        this: &JSFido2MakeCredentialUserInterface,
        credential_name: String,
        user_name: String,
        user_verification: bool,
    ) -> Promise;
}

// Rc<...> is to avoid needing to take ownership of the Client during our async run_command
// function https://github.com/rustwasm/wasm-bindgen/issues/2195#issuecomment-799588401
#[wasm_bindgen]
pub struct BitwardenClient(Rc<JsonClient>);

#[wasm_bindgen]
impl BitwardenClient {
    #[wasm_bindgen(constructor)]
    pub fn new(settings_input: Option<String>, log_level: Option<LogLevel>) -> Self {
        console_error_panic_hook::set_once();
        if let Err(e) =
            console_log::init_with_level(convert_level(log_level.unwrap_or(LogLevel::Info)))
        {
            panic!("failed to initialize logger: {:?}", e);
        }

        Self(Rc::new(bitwarden_json::client::Client::new(settings_input)))
    }

    #[wasm_bindgen]
    pub fn run_command(&self, js_input: String) -> Promise {
        let rc = self.0.clone();
        future_to_promise(async move {
            let result = rc.run_command(&js_input).await;
            Ok(result.into())
        })
    }

    #[wasm_bindgen]
    pub async fn client_create_credential(
        &mut self,
        param: String,
        user_interface: JSFido2MakeCredentialUserInterface,
    ) {
        log::info!("wasm_bindgen.client_create_credential");
        log::debug!("wasm_bindgen.client_create_credential");
        // let request = Fido2ClientCreateCredentialRequest {
        //     webauthn_json: param,
        // };
        // let wrapped_user_interface = user_interface.to_channel_wrapped();

        // self.0
        //     .client_create_credential(request, wrapped_user_interface)
        //     .await
        //     .unwrap();
    }
}
