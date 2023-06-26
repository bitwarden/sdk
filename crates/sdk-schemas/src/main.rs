use std::{fs::File, io::Write};

use anyhow::Result;
use bitwarden::sdk::*;
use itertools::Itertools;
use schemars::schema::RootSchema;
use schemars::schema_for;

/// Creates a json schema file for any type passed in using Schemars. The filename and path of the generated
/// schema file is derived from the namespace passed into the macro or supplied as the first argument.
///
/// The schema filename is given by the last namespace element and trims off any `>` characters.
/// This means the filename will represent the last _generic_ type of the type given.
///
/// The schema path is rooted at the current working directory.
///
/// # Usage
///
/// ## Fully generated
///
/// Subpath is equal to the namespace except the last two elements, which are assumed to be
/// a filename and struct name.
///
/// Min namespace length is currently 3.
///
/// ### Examples
///
/// ```
/// write_schema_for!(request::command::Command);
/// ```
/// will generate `Command.json` at `{{pwd}}/request/Command.json`
///
/// ```
/// write_schema_for!(response::two_factor_login_response::two_factor_providers::TwoFactorProviders);
/// ```
/// will generate `TwoFactorProviders.json` at `{{pwd}}/response/two_factor_login_response/TwoFactorProviders.json`
///
/// ## Path specified
///
/// You can also specify a custom path and type, separated by a comman
///
/// ### Examples
///
/// ```
/// write_schema_for!("path/to/folder", Request<Response>);
/// ```
/// will generate `Response.json` at `{{pwd}}/path/to/folder/Response.json`
macro_rules! write_schema_for {
    ($type:ty) => {
        let schema = schema_for!($type);

        let type_name = stringify!($type);
        let path: Vec<&str> = type_name.split("::").collect();
        let dir_path =
            String::from("support/schemas/") + &path.iter().take(path.len() - 2).join("/");

        write_schema(schema, dir_path, type_name.to_string())?;
    };
    ($path:literal, $type:ty) => {
        let schema = schema_for!($type);

        write_schema(
            schema,
            String::from("support/schemas/") + $path,
            stringify!($type).to_string(),
        )?;
    };
}

macro_rules! write_schema_for_response {
    ( $($type:ty),+ $(,)? ) => {
        $( write_schema_for!("response", bitwarden_json::response::Response<$type>); )+
    };
}

fn write_schema(schema: RootSchema, dir_path: String, type_name: String) -> Result<()> {
    let file_name = type_name
        .split("::")
        .last()
        .unwrap()
        .to_string()
        .trim_end_matches('>')
        .to_string()
        + ".json";

    let content = serde_json::to_string_pretty(&schema)?;
    let _ = std::fs::create_dir_all(&dir_path);
    let mut file = File::create(format!("{}/{}", dir_path, file_name))?;
    writeln!(&mut file, "{}", &content)?;
    Ok(())
}

fn main() -> Result<()> {
    write_schema_for!(request::client_settings::ClientSettings);
    write_schema_for!(request::command::Command);

    write_schema_for!(
        "response",
        bitwarden_json::response::Response<bitwarden::auth::response::PasswordLoginResponse>
    );
    write_schema_for!(
        "response",
        bitwarden_json::response::Response<bitwarden::auth::response::ApiKeyLoginResponse>
    );
    write_schema_for!(
        "response",
        bitwarden_json::response::Response<response::user_api_key_response::UserApiKeyResponse>
    );

    write_schema_for_response! {
        bitwarden::auth::response::ApiKeyLoginResponse,
        bitwarden::auth::response::PasswordLoginResponse,
        bitwarden::secrets_manager::secrets::SecretIdentifiersResponse,
        bitwarden::secrets_manager::secrets::SecretResponse,
        bitwarden::secrets_manager::secrets::SecretsDeleteResponse,
        response::sync_response::SyncResponse,
        response::user_api_key_response::UserApiKeyResponse,
    };

    Ok(())
}
