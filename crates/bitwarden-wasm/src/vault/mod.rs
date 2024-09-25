use serde_wasm_bindgen::to_value;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

use bitwarden::{
    error::Error,
    vault::{ClientVaultExt, TotpError, TotpResponse},
};
use chrono::prelude::*;

use crate::error::Result;

#[wasm_bindgen(typescript_custom_section)]
const TOTP_RESPONSE: &'static str = r#"
export interface TotpResponse {
  code: string;
  period_foo: number;
}

export interface ClientVault {
  generate_totp(key: string, time?: string): Promise<TotpResponse>;
}
"#;

#[wasm_bindgen]
pub struct JsTotpResult {
    code: String,
    period: u64,
}

// #[derive(uniffi::Object)]
#[wasm_bindgen]
pub struct ClientVault(pub(crate) Arc<bitwarden::Client>);

// #[uniffi::export]
#[wasm_bindgen]
impl ClientVault {
    // /// Folder operations
    // pub fn folders(self: Arc<Self>) -> Arc<folders::ClientFolders> {
    //     Arc::new(folders::ClientFolders(self.0.clone()))
    // }

    // /// Collections operations
    // pub fn collections(self: Arc<Self>) -> Arc<collections::ClientCollections> {
    //     Arc::new(collections::ClientCollections(self.0.clone()))
    // }

    // /// Ciphers operations
    // pub fn ciphers(self: Arc<Self>) -> Arc<ciphers::ClientCiphers> {
    //     Arc::new(ciphers::ClientCiphers(self.0.clone()))
    // }

    // /// Password history operations
    // pub fn password_history(self: Arc<Self>) -> Arc<password_history::ClientPasswordHistory> {
    //     Arc::new(password_history::ClientPasswordHistory(self.0.clone()))
    // }

    // /// Attachment file operations
    // pub fn attachments(self: Arc<Self>) -> Arc<attachments::ClientAttachments> {
    //     Arc::new(attachments::ClientAttachments(self.0.clone()))
    // }

    /// Generate a TOTP code from a provided key.
    ///
    /// The key can be either:
    /// - A base32 encoded string
    /// - OTP Auth URI
    /// - Steam URI
    #[wasm_bindgen(skip_typescript)]
    pub async fn generate_totp(&self, key: String, time: Option<String>) -> JsValue {
        // TODO: Fix time
        // let time = time.map(|time| {
        //     // TODO: fix error
        //     DateTime::parse_from_rfc3339(&time).map_err(|_| Error::Totp(TotpError::InvalidOtpauth))
        // });
        let result = self
            .0
            .vault()
            .generate_totp(key, None)
            .map_err(Error::Totp)
            .unwrap();

        to_value(&result).unwrap()

        // JsTotpResult {
        //     code: result.code,
        //     period: result.period.into(),
        // }
    }

    // Generate a TOTP code from a provided cipher list view.
    // pub fn generate_totp_cipher_view(
    //     &self,
    //     view: CipherListView,
    //     time: Option<DateTime<Utc>>,
    // ) -> Result<TotpResponse> {
    //     Ok(self
    //         .0
    //          .0
    //         .vault()
    //         .generate_totp_cipher_view(view, time)
    //         .map_err(Error::Totp)?)
    // }
}
