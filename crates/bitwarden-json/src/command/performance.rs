use bitwarden::platform::performance_test::{DecryptPerformanceRequest, EncryptPerformanceRequest, Pbkdf2PerformanceRequest};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum PerformanceCommand {
    Decrypt(DecryptPerformanceRequest),
    Encrypt(EncryptPerformanceRequest),
    Pbkdf2(Pbkdf2PerformanceRequest),
}
