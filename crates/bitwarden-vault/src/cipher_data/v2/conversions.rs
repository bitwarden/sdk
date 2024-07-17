use crate::cipher_data::{MigrationError, Migrator};

use super::cipher::CipherDataV2;

// struct NoneDataError;

// impl TryFrom<bitwarden_api_api::models::CipherDetailsResponseModelExample> for CipherDataV2 {
//     type Error = NoneDataError;

//     fn try_from(
//         value: bitwarden_api_api::models::CipherDetailsResponseModelExample,
//     ) -> Result<Self, Self::Error> {
//         Ok(CipherDataV2 {
//             data: value.data.ok_or(NoneDataError)?,
//         })
//     }
// }

impl Migrator<crate::cipher_data::v1::CipherDataV1, CipherDataV2> for CipherDataV2 {
    async fn migrate_from(
        &self,
        from: crate::cipher_data::v1::CipherDataV1,
    ) -> Result<CipherDataV2, MigrationError> {
        // TODO: Implement migration
        Ok(CipherDataV2 { data: from.data })
    }
}
