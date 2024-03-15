extern crate console_error_panic_hook;
use std::{cell::Cell, fmt::Result, process::Output, rc::Rc, sync::RwLock};

use bitwarden_json::{
    client::Client as JsonClient, Fido2ClientCreateCredentialRequest,
    Fido2ClientGetAssertionRequest, Fido2MakeCredentialUserInterface, NewCredentialParams,
    NewCredentialResult, VaultItem,
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

// We create a new function that takes the wasm object, spawns a local task and creates the communication channels
// This function will wrap the channels in a struct to allow us to implement the trait there
impl JSFido2MakeCredentialUserInterface {
    fn to_channel_wrapped(self) -> JSFido2MakeCredentialUserInterfaceWrapper {
        let (tx_wrapper, mut rx_task) = futures::channel::mpsc::channel::<NewCredentialParams>(1);
        let (mut tx_task, rx_wrapper) = futures::channel::mpsc::channel::<NewCredentialResult>(1);

        // Spawn the local task which just waits until we receive input from the trait, note that this is not Send but we don't care
        wasm_bindgen_futures::spawn_local(async move {
            let params = rx_task.next().await.unwrap();

            let picked_id_promise = self.confirm_new_credential(
                params.credential_name,
                params.user_name,
                params.user_verification,
            );
            let picked_id = wasm_bindgen_futures::JsFuture::from(picked_id_promise).await;
            let picked_id = picked_id.unwrap().as_string().unwrap();

            tx_task
                .send(NewCredentialResult {
                    cipher_id: picked_id,
                    user_verified: false,
                })
                .await
                .unwrap();
        });

        JSFido2MakeCredentialUserInterfaceWrapper {
            tx: async_lock::Mutex::new(tx_wrapper),
            rx: async_lock::Mutex::new(rx_wrapper),
        }
    }
}

struct JSFido2MakeCredentialUserInterfaceWrapper {
    tx: async_lock::Mutex<futures::channel::mpsc::Sender<NewCredentialParams>>,
    rx: async_lock::Mutex<futures::channel::mpsc::Receiver<NewCredentialResult>>,
}

// This is implemented over the wrapper now, which only contains the channels, and should be Send
#[async_trait::async_trait]
impl Fido2MakeCredentialUserInterface for JSFido2MakeCredentialUserInterfaceWrapper {
    async fn confirm_new_credential(
        &self,
        params: NewCredentialParams,
    ) -> bitwarden_json::Result<NewCredentialResult> {
        log::debug!("JSFido2MakeCredentialUserInterface.pick_credential");

        self.tx.lock().await.send(params).await.unwrap();

        let result = self.rx.lock().await.next().await.unwrap();

        Ok(result)
    }
}

// impl Fido2GetAssertionUserInterface for JSFido2GetAssertionUserInterface {
//     async fn pick_credential(
//         &self,
//         cipher_ids: Vec<String>,
//         rp_id: &str,
//     ) -> bitwarden_json::Result<VaultItem> {
//         log::debug!("JSFido2GetAssertionUserInterface.pick_credential");
//         let picked_id_promise = self.pick_credential(cipher_ids.clone(), rp_id.to_string());

//         let picked_id = wasm_bindgen_futures::JsFuture::from(picked_id_promise).await;

//         Ok(VaultItem::new(
//             picked_id.unwrap().as_string().unwrap(),
//             "name".to_string(),
//         ))
//     }
// }

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
        // let rc = self.0.clone();
        // future_to_promise(async move {
        //     let result = rc.run_command(&js_input).await;
        //     Ok(result.into())
        // })
        let request = Fido2ClientCreateCredentialRequest {
            webauthn_json: param,
        };
        let wrapped_user_interface = user_interface.to_channel_wrapped();

        self.0
            .client_create_credential(request, wrapped_user_interface)
            .await
            .unwrap();
    }
}
