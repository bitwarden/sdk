use bitwarden_crypto::{CryptoError, SymmetricCryptoKey};
use bitwarden_versioning::{Migrator, Versioned};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod migration;

use crate::cipher::cipher::CipherData;
use crate::{UniffiCustomTypeConverter, VaultParseError};

use super::v1::CipherDataV1;
use super::v2::CipherDataV2;

type CipherDataLatest = CipherDataV2;

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub(super) enum Data {
    V1(CipherDataV1),
    V2(CipherDataV2),
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct VersionedCipherData {
    data: Versioned<Data, CipherDataV2>,
}

impl VersionedCipherData {
    pub fn new(data: CipherDataLatest) -> Self {
        Self {
            data: Versioned::new(Data::V2(data)),
        }
    }

    pub fn get_data(&self, key: &SymmetricCryptoKey) -> Result<CipherData, CryptoError> {
        let migrated = self.data.migrate(key)?;
        Ok(migrated.into())
    }
}

impl Default for VersionedCipherData {
    fn default() -> Self {
        Self {
            data: Versioned::new(CipherDataLatest::default().into()),
        }
    }
}

impl From<CipherDataLatest> for CipherData {
    fn from(value: CipherDataLatest) -> Self {
        todo!()
    }
}

impl From<CipherData> for CipherDataLatest {
    fn from(value: CipherData) -> Self {
        todo!()
    }
}

impl From<CipherData> for VersionedCipherData {
    fn from(value: CipherData) -> Self {
        value.into()
    }
}

impl TryFrom<serde_json::Value> for VersionedCipherData {
    type Error = VaultParseError;

    fn try_from(value: serde_json::Value) -> Result<Self, VaultParseError> {
        let version = value["version"].as_str().unwrap_or("0");

        // TODO: Move CipherDataV1 and CipherDataV2 instation to separate
        // try_from implementations
        match version {
            "1" => {
                let data = CipherDataV1 {
                    data: value["data"].clone(),
                };
                Ok(VersionedCipherData {
                    data: Versioned::new(data.into()),
                })
            }
            "2" => {
                let data = CipherDataV2 {
                    data: value["data"].clone(),
                };
                Ok(VersionedCipherData {
                    data: Versioned::new(data.into()),
                })
            }
            _ => Err(VaultParseError::InvalidVersion),
        }
    }
}

uniffi::custom_type!(VersionedCipherData, String);

impl UniffiCustomTypeConverter for VersionedCipherData {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(serde_json::from_str(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        // TODO: Fix unwrap?
        serde_json::to_string(&obj).unwrap()
    }
}
