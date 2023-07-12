mod create;
mod delete;
mod get;
mod list;
mod secret_response;
mod update;

pub(crate) use create::create_secret;
pub use create::SecretCreateRequest;
pub(crate) use delete::delete_secrets;
pub use delete::{SecretsDeleteRequest, SecretsDeleteResponse};
pub(crate) use get::get_secret;
pub use get::SecretGetRequest;
pub(crate) use list::{list_secrets, list_secrets_by_project};
pub use list::{
    SecretIdentifiersByProjectRequest, SecretIdentifiersRequest, SecretIdentifiersResponse,
};
pub use secret_response::SecretResponse;
pub(crate) use update::update_secret;
pub use update::SecretPutRequest;

const SECRET_KEY_MAX_LENGTH: usize = 500;
const SECRET_VALUE_MAX_LENGTH: usize = 3000;
const SECRET_NOTE_MAX_LENGTH: usize = 7000;
