use chrono::{DateTime, Utc};

use crate::{
    error::Result,
    vault::{generate_totp, ClientVault, TotpResponse},
};

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
    ) -> Result<TotpResponse> {
        generate_totp(key, time)
    }
}
