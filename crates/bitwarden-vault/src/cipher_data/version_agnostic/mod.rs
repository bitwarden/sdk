use bitwarden_crypto::{CryptoError, SymmetricCryptoKey};
use bitwarden_versioning::{Migrator, Versioned};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod migration;

use crate::cipher::cipher::CipherData;
use crate::UniffiCustomTypeConverter;

use super::v1::CipherDataV1;
use super::v2::CipherDataV2;

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub(self) enum Data {
    V1(CipherDataV1),
    V2(CipherDataV2),
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct VersionedCipherData {
    data: Versioned<Data, CipherDataV2>,
}

impl VersionedCipherData {
    pub fn new(data: CipherDataV2) -> Self {
        Self {
            data: Versioned::new(Data::V2(data)),
        }
    }

    pub fn get_data(&self, key: &SymmetricCryptoKey) -> Result<CipherData, CryptoError> {
        let migrated = self.data.migrate(key)?;
        Ok(migrated.into())
    }
}

impl From<CipherDataV2> for CipherData {
    fn from(value: CipherDataV2) -> Self {
        todo!()
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
