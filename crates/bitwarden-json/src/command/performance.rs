use bitwarden::platform::performance_test::{DecryptPerformanceRequest, EncryptPerformanceRequest};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum PerformanceCommand {
    /// Decrypts
    Decrypt(DecryptPerformanceRequest),
    Encrypt(EncryptPerformanceRequest),
}
