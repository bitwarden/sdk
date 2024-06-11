use chrono::{DateTime, Utc};

use crate::{generate_totp, ClientVault, TotpError, TotpResponse};

impl<'a> ClientVault<'a> {
    /// Generate a TOTP code from a provided key.
    ///
    /// Key can be either:
    /// - A base32 encoded string
    /// - OTP Auth URI
    /// - Steam URI
    pub fn generate_totp(
        &'a self,
        key: String,
        time: Option<DateTime<Utc>>,
    ) -> Result<TotpResponse, TotpError> {
        generate_totp(key, time)
    }
}
