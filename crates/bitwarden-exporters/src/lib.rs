use schemars::JsonSchema;

mod client_exporter;
mod csv;
mod json;
pub use client_exporter::{ClientExporters, ClientExportersExt};
mod encrypted_json;
mod error;
pub(crate) mod export;
pub use error::ExportError;
mod models;

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

#[derive(JsonSchema)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum ExportFormat {
    Csv,
    Json,
    EncryptedJson { password: String },
}
