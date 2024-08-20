mod migration;
mod model;

mod v1;

pub(self) mod unmigrated {
    pub use bitwarden_api_api::models::CipherDetailsResponseModel;
}

pub(self) mod migrated {
    pub use super::model::*;
}
