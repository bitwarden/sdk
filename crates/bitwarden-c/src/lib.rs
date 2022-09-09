#[cfg(not(target_arch = "wasm32"))]
pub use c::*;

#[cfg(not(target_arch = "wasm32"))]
mod c;
mod macros;
