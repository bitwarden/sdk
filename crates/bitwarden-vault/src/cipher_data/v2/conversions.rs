use crate::cipher_data::{MigrationError, Migrator};

use super::{cipher::CipherCardDataV2, CipherDataV2};

impl From<bitwarden_api_api::models::CipherDetailsResponseModel> for CipherCardDataV2 {
    fn from(value: bitwarden_api_api::models::CipherDetailsResponseModel) -> Self {
        todo!()
    }
}

impl Migrator<crate::cipher_data::v1::CipherDataV1, CipherDataV2> for CipherDataV2 {
    async fn migrate_from(
        &self,
        from: crate::cipher_data::v1::CipherDataV1,
    ) -> Result<CipherDataV2, MigrationError> {
        todo!()
    }
}
