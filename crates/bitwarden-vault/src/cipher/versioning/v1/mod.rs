use bitwarden_api_api::models::cipher_details_response_model::CipherDetailsMetaDataResponseModel;
use bitwarden_crypto::{CryptoError, SymmetricCryptoKey};

use super::migration::Migrator;

/// Migrator for version 0 -> 1 of the cipher details response model.
pub struct V1Migrator {}

impl Migrator for V1Migrator {
    /// Mock implementation of the migration from version 0 to 1.
    fn migrate(
        _metadata: &CipherDetailsMetaDataResponseModel,
        input: &serde_json::Value,
        _key: &SymmetricCryptoKey,
    ) -> Result<serde_json::Value, CryptoError> {
        // TODO: Fix clone
        let mut input = input.clone();

        if (input["version"].as_i64().unwrap_or(0)) != 0 {
            return Ok(input);
        }

        input["version"] = 1.into();

        Ok(input)
    }
}
