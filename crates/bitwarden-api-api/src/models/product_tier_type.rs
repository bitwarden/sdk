/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

///
#[repr(i64)]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
)]
pub enum ProductTierType {
    Free = 0,
    Families = 1,
    Teams = 2,
    Enterprise = 3,
    TeamsStarter = 4,
}

impl std::fmt::Display for ProductTierType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Free => write!(f, "0"),
            Self::Families => write!(f, "1"),
            Self::Teams => write!(f, "2"),
            Self::Enterprise => write!(f, "3"),
            Self::TeamsStarter => write!(f, "4"),
        }
    }
}

impl Default for ProductTierType {
    fn default() -> ProductTierType {
        Self::Free
    }
}
