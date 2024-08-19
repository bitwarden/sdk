mod model;

pub use bitwarden_api_api::models::CipherDetailsResponseModel as NonMigratedCipherDetailsResponseModel;

pub mod migrated {
    pub use super::model::CipherDetailsResponseModel as MigratedCipherDetailsResponseModel;
}
