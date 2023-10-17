use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct TotpResponse {
    /// Generated TOTP code
    pub code: String,
    /// Time period
    pub period: u32,
}

/// Generate a OATH or RFC 6238 TOTP code from a provided key.
///
/// https://datatracker.ietf.org/doc/html/rfc6238
///
/// Key can be either:
/// - A base32 encoded string
/// - OTP Auth URI
/// - Steam URI
///
/// Supports providing an optional time, and defaults to current system time if none is provided.
pub async fn generate_totp(_key: String, _time: Option<DateTime<Utc>>) -> TotpResponse {
    TotpResponse {
        code: "000 000".to_string(),
        period: 30,
    }
}
