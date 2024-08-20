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

/// A trait for downgrading cipher data.
/// This is useful when releasing new features hidden behind feature flags.
/// When don't want to be upgrading all the user's cipher data until they are ready to use the new feature,
/// since some clients may not support the new feature, and we don't want to break their ability to use the vault.
pub(super) trait Downgrader {
    /// Downgrade the input data.
    /// If the data is already downgraded, the original data will be returned.
    /// If the data cannot be downgraded without loss of information, the original data will be returned.
    /// If this downgrader is not applicable to this version, an error will be thrown.
    fn downgrade(
        metadata: &CipherDetailsMetaDataResponseModel,
        input: &serde_json::Value,
        key: &SymmetricCryptoKey,
    ) -> Result<serde_json::Value, CryptoError>;
}

pub fn migrate(
    cipher: super::unmigrated::CipherDetailsResponseModel,
    key: &SymmetricCryptoKey,
) -> Result<super::migrated::CipherDetailsResponseModel, CryptoError> {
    let migrators: Vec<
        fn(
            &CipherDetailsMetaDataResponseModel,
            &serde_json::Value,
            &SymmetricCryptoKey,
        ) -> Result<serde_json::Value, CryptoError>,
    > = vec![
        super::v1::V1Migrator::migrate,
        super::v2::V2Migrator::migrate,
    ];

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
