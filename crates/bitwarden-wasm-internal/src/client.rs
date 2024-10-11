extern crate console_error_panic_hook;
use std::rc::Rc;

use bitwarden_core::{Client, ClientSettings};
use log::{set_max_level, Level};
use wasm_bindgen::prelude::*;

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

// Rc<...> is to avoid needing to take ownership of the Client during our async run_command
// function https://github.com/rustwasm/wasm-bindgen/issues/2195#issuecomment-799588401
#[wasm_bindgen]
pub struct BitwardenClient(Rc<Client>);

#[wasm_bindgen]
impl BitwardenClient {
    #[wasm_bindgen(constructor)]
    pub fn new(settings: Option<ClientSettings>, log_level: Option<LogLevel>) -> Self {
        console_error_panic_hook::set_once();
        let log_level = convert_level(log_level.unwrap_or(LogLevel::Info));
        if let Err(_e) = console_log::init_with_level(log_level) {
            set_max_level(log_level.to_level_filter())
        }

        Self(Rc::new(Client::new(settings)))
    }

    /// Test method, echoes back the input
    pub fn echo(&self, msg: String) -> String {
        msg
    }

    /// Test method, calls http endpoint
    pub async fn http_get(&self, url: String) -> Result<String, String> {
        let client = self.0.internal.get_http_client();
        let res = client.get(&url).send().await.map_err(|e| e.to_string())?;

        res.text().await.map_err(|e| e.to_string())
    }
}
