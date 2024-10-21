use bitwarden_error::FlatError;
use wasm_bindgen::prelude::*;

// Importing an error class defined in JavaScript instead of defining it in Rust
// allows us to extend the `Error` class. It also provides much better console output.
#[wasm_bindgen(module = "/src/error.js")]
extern "C" {
    type WasmError;

    #[wasm_bindgen(constructor)]
    fn new(message: String, name: Option<String>) -> WasmError;
}

pub type Result<T, E = GenericError> = std::result::Result<T, E>;

pub struct GenericError(pub String);

pub struct FlattenedError {
    pub variant: String,
    pub message: String,
}

impl<T: ToString> From<T> for GenericError {
    fn from(error: T) -> Self {
        GenericError(error.to_string())
    }
}

impl<T: FlatError> From<T> for FlattenedError {
    fn from(error: T) -> Self {
        FlattenedError {
            variant: error.get_variant().to_owned(),
            message: error.get_message().to_owned(),
        }
    }
}

impl From<GenericError> for JsValue {
    fn from(error: GenericError) -> Self {
        WasmError::new(error.0, None).into()
    }
}

impl From<FlattenedError> for JsValue {
    fn from(error: FlattenedError) -> Self {
        WasmError::new(error.message, Some(error.variant)).into()
    }
}
