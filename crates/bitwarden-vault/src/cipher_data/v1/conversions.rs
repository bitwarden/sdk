use super::cipher::CipherDataV1;

struct NoneDataError;

impl TryFrom<bitwarden_api_api::models::CipherDetailsResponseModelExample> for CipherDataV1 {
    type Error = NoneDataError;

    fn try_from(
        value: bitwarden_api_api::models::CipherDetailsResponseModelExample,
    ) -> Result<Self, Self::Error> {
        Ok(CipherDataV1 {
            data: value.data.ok_or(NoneDataError)?,
        })
    }
}
