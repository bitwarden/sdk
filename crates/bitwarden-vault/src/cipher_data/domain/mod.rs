// TODO: Come up with a better name than "domain"

pub(crate) mod attachment;
pub(crate) mod card;
#[allow(clippy::module_inception)]
pub(crate) mod cipher;
pub(crate) mod field;
pub(crate) mod identity;
pub(crate) mod login;
pub(crate) mod secure_note;

// use crate::UniffiCustomTypeConverter;

// pub use super::v2::conversions::*;
// pub use super::v2::CipherDataV2 as CipherDataLatest;

// // impl TryFrom<bitwarden_api_api::models::CipherDetailsResponseModel> for CipherDataLatest {
// //     fn try_from(value: bitwarden_api_api::models::CipherDetailsResponseModel) -> Self {
// //         CipherDataV2::try_from
// //     }
// // }

// // impl TryFrom<bitwarden_api_api::models::CipherDetailsResponseModelExample> for CipherDataLatest {
// //     type Error = super::v2::conversions::NoneDataError;

// //     fn try_from(
// //         value: bitwarden_api_api::models::CipherDetailsResponseModelExample,
// //     ) -> Result<Self, Self::Error> {
// //         Ok(CipherDataV2 {
// //             data: value.data.ok_or(NoneDataError)?,
// //         })
// //     }
// // }

// uniffi::custom_type!(CipherDataLatest, String);

// impl UniffiCustomTypeConverter for CipherDataLatest {
//     type Builtin = String;

//     fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
//         Ok(serde_json::from_str(&val)?)
//     }

//     fn from_custom(obj: Self) -> Self::Builtin {
//         // TODO: Fix unwrap?
//         serde_json::to_string(&obj).unwrap()
//     }
// }
