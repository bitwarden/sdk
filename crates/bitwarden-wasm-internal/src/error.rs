use wasm_bindgen::prelude::*;

// Importing an error class defined in JavaScript instead of defining it in Rust
// allows us to extend the `Error` class. It also provides much better console output.
#[wasm_bindgen(module = "/src/error.js")]
extern "C" {
    type WasmError;

    #[wasm_bindgen(constructor)]
    fn new(message: String) -> WasmError;
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Error(bitwarden::error::Error);

impl From<bitwarden::error::Error> for Error {
    fn from(error: bitwarden::error::Error) -> Self {
        Self(error)
    }
}

impl From<bitwarden::Error> for Error {
    fn from(error: bitwarden::Error) -> Self {
        Self(error.into())
    }
}

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        WasmError::new(error.0.to_string()).into()
    }
}
