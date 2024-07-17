use bitwarden_crypto::{CryptoError, SymmetricCryptoKey};
use bitwarden_versioning::Migrator;

use crate::cipher_data::v2::migration::migrate_v2;

use super::{CipherDataLatest, Data};

impl Migrator<CipherDataLatest> for Data {
    fn migrate(&self, key: &SymmetricCryptoKey) -> Result<CipherDataLatest, CryptoError> {
        match self {
            Data::V1(data) => Ok(migrate_v2(&data, key)?),
            Data::V2(data) => Ok(data.clone()),
        }
    }
}
