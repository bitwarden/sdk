// These are the C bindings, we're going to have to use unsafe raw pointers
#![allow(clippy::not_unsafe_ptr_arg_deref)]

#[cfg(not(target_arch = "wasm32"))]
pub use c::*;

#[cfg(not(target_arch = "wasm32"))]
mod c;
mod macros;
