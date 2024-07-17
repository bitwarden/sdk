use bitwarden_crypto::{CryptoError, SymmetricCryptoKey};
use bitwarden_versioning::Migrator;

use crate::cipher_data::{v1::CipherDataV1, v2::CipherDataV2};

use super::Data;

impl Migrator<CipherDataV2> for Data {
    fn migrate(&self, _key: &SymmetricCryptoKey) -> Result<CipherDataV2, CryptoError> {
        match self {
            Data::V1(CipherDataV1 { data }) => Ok(CipherDataV2 { data: data.clone() }),
            Data::V2(data) => Ok(data.clone()),
        }
    }
}
