use chrono::{DateTime, Utc};

use crate::{
    generate_totp, generate_totp_cipher_view, CipherListView, ClientVault, TotpError, TotpResponse,
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
    ) -> Result<TotpResponse, TotpError> {
        generate_totp(key, time)
    }

    /// Generate a TOTP code from a provided cipher list view.
    pub fn generate_totp_cipher_view(
        &'a self,
        view: CipherListView,
        time: Option<DateTime<Utc>>,
    ) -> Result<TotpResponse, TotpError> {
        let mut ctx = self.client.internal.get_crypto_service().context();

        generate_totp_cipher_view(&mut ctx, view, time)
    }
}
