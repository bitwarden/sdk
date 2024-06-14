#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

mod error;
pub use error::SendParseError;
mod client_sends;
pub use client_sends::{ClientSends, ClientSendsExt};
mod send;
pub use send::{Send, SendListView, SendView};

#[cfg(feature = "uniffi")]
mod uniffi_support;
