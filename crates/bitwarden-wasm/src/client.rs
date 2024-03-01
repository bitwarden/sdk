extern crate console_error_panic_hook;
use std::{fmt::Result, process::Output, rc::Rc};

use bitwarden_json::{
    client::Client as JsonClient, Fido2ClientGetAssertionRequest, Fido2GetAssertionUserInterface,
    VaultItem,
};
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
    pub type JSFido2GetAssertionUserInterface;

    #[wasm_bindgen(structural, method)]
    pub fn pick_credential(
        this: &JSFido2GetAssertionUserInterface,
        cipher_ids: Vec<String>,
        rp_id: String,
    ) -> Promise;
}

impl Fido2GetAssertionUserInterface for JSFido2GetAssertionUserInterface {
    async fn pick_credential(
        &self,
        cipher_ids: Vec<String>,
        rp_id: &str,
    ) -> bitwarden_json::Result<VaultItem> {
        log::debug!("JSFido2GetAssertionUserInterface.pick_credential");
        let picked_id_promise = self.pick_credential(cipher_ids.clone(), rp_id.to_string());

        let picked_id = wasm_bindgen_futures::JsFuture::from(picked_id_promise).await;

        Ok(VaultItem::new(
            picked_id.unwrap().as_string().unwrap(),
            "name".to_string(),
        ))
    }
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
    pub async fn client_get_assertion(
        &mut self,
        param: String,
        user_interface: JSFido2GetAssertionUserInterface,
    ) {
        log::info!("wasm_bindgen.client_get_assertion");
        log::debug!("wasm_bindgen.client_get_assertion");
        // let rc = self.0.clone();
        // future_to_promise(async move {
        //     let result = rc.run_command(&js_input).await;
        //     Ok(result.into())
        // })
        let request = Fido2ClientGetAssertionRequest {
            webauthn_json: param,
        };

        self.0
            .client_get_assertion(request, user_interface)
            .await
            .unwrap();
    }
}
