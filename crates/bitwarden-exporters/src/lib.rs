use schemars::JsonSchema;

mod csv;
use crate::csv::export_csv;
mod json;
use json::export_json;
mod client_exporter;
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
