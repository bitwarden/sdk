use crate::UniffiCustomTypeConverter;

pub use super::v2::conversions::*;
pub use super::v2::CipherDataV2 as CipherDataLatest;

impl From<bitwarden_api_api::models::CipherDetailsResponseModel> for CipherDataLatest {
    fn from(value: bitwarden_api_api::models::CipherDetailsResponseModel) -> Self {
        todo!()
    }
}

uniffi::custom_type!(CipherDataLatest, String);

impl UniffiCustomTypeConverter for CipherDataLatest {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(serde_json::from_str(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        // TODO: Fix unwrap?
        serde_json::to_string(&obj).unwrap()
    }
}
