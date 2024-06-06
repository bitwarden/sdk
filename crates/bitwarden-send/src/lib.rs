#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

mod client_sends;
pub use client_sends::{ClientSends, ClientSendsExt};
mod send;
pub use send::{Send, SendListView, SendView};

#[cfg(feature = "uniffi")]
mod uniffi_support;
