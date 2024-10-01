use bitwarden_api_api::models::cipher_details_response_model::CipherDetailsMetaDataResponseModel;
use bitwarden_crypto::{CryptoError, SymmetricCryptoKey};

pub(super) trait Migrator {
    /// Migrate the input data. If the data is already migrated or if this migrator is not applicable to this version,
    /// the original data will be returned.
    fn migrate(
        metadata: &CipherDetailsMetaDataResponseModel,
        input: &serde_json::Value,
        key: &SymmetricCryptoKey,
    ) -> Result<serde_json::Value, CryptoError>;
}

pub fn migrate(
    cipher: super::unmigrated::CipherDetailsResponseModel,
    key: &SymmetricCryptoKey,
) -> Result<super::migrated::CipherDetailsResponseModel, CryptoError> {
    let migrators = vec![super::v1::V1Migrator::migrate];

    let mut data = None;

    for migrator in migrators {
        data = Some(migrator(&cipher.meta_data, &cipher.data, key)?);
    }

    // TODO: fix placeholder error handling
    let data = data.ok_or(CryptoError::MissingField("data"))?;

    return Ok(super::migrated::CipherDetailsResponseModel {
        object: cipher.object,
        meta_data: cipher.meta_data,
        // TODO: fix placeholder error handling
        data: serde_json::from_value(data).map_err(|_| CryptoError::MissingField("data"))?,
    });
}
