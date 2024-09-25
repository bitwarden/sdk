use std::rc::Rc;
use wasm_bindgen::prelude::*;

use crate::log_level::{convert_level, LogLevel};

// Rc<...> is to avoid needing to take ownership of the Client during our async run_command
// function https://github.com/rustwasm/wasm-bindgen/issues/2195#issuecomment-799588401
#[wasm_bindgen]
pub struct RawBitwardenClient(Rc<bitwarden::Client>);

#[wasm_bindgen]
impl RawBitwardenClient {
    #[wasm_bindgen(constructor)]
    pub fn new(_settings_input: Option<String>, log_level: Option<LogLevel>) -> Self {
        // let settings = Self::parse_settings(settings_input);
        console_error_panic_hook::set_once();
        if let Err(e) =
            console_log::init_with_level(convert_level(log_level.unwrap_or(LogLevel::Info)))
        {
            panic!("failed to initialize logger: {:?}", e);
        }
        Self(Rc::new(bitwarden::Client::new(None)))
    }

    #[wasm_bindgen]
    pub fn hello(&self) -> String {
        println!("Hello from Rust!");
        "Hello from Rust!".to_string()
    }
}
