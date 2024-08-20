mod migration;
mod model;

mod v1;
mod v2;

pub use migration::migrate;

pub(self) mod unmigrated {
    pub use bitwarden_api_api::models::CipherDetailsResponseModel;
}

pub mod migrated {
    pub use super::model::*;
}
