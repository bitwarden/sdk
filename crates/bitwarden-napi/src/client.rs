extern crate log;

use bitwarden_json::client::Client as JsonClient;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

fn convert_level(level: LogLevel) -> log::LevelFilter {
    match level {
        LogLevel::Trace => log::LevelFilter::Trace,
        LogLevel::Debug => log::LevelFilter::Debug,
        LogLevel::Info => log::LevelFilter::Info,
        LogLevel::Warn => log::LevelFilter::Warn,
        LogLevel::Error => log::LevelFilter::Error,
    }
}

#[napi]
pub struct BitwardenClient(JsonClient);

#[napi]
impl BitwardenClient {
    #[napi(constructor)]
    pub fn new(settings_input: Option<String>, log_level: Option<LogLevel>) -> Self {
        env_logger::Builder::from_default_env()
            .filter_level(convert_level(log_level.unwrap_or(LogLevel::Info)))
            .init();
        Self(bitwarden_json::client::Client::new(settings_input))
    }

    #[napi]
    pub async unsafe fn run_command(&mut self, command_input: String) -> String {
        self.0.run_command(&command_input).await.into()
    }
}
