use bitwarden::client::client_settings::ClientSettings;

#[cfg(feature = "secrets")]
use crate::command::{ProjectsCommand, SecretsCommand};
use crate::{
    command::Command,
    response::{Response, ResponseIntoString},
};

pub struct Client(bitwarden::Client);

impl Client {
    pub fn new(settings_input: Option<String>) -> Self {
        let settings = Self::parse_settings(settings_input);
        Self(bitwarden::Client::new(settings))
    }

    pub async fn run_command(&mut self, input_str: &str) -> String {
        const SUBCOMMANDS_TO_CLEAN: &[&str] = &["Secrets"];
        let mut cmd_value: serde_json::Value = match serde_json::from_str(input_str) {
            Ok(cmd) => cmd,
            Err(e) => {
                return Response::error(format!("Invalid command string: {}", e)).into_string()
            }
        };

        clean_null_fields(&mut cmd_value);
        for &subcommand in SUBCOMMANDS_TO_CLEAN {
            if let Some(v) = cmd_value.get_mut(subcommand) {
                clean_null_fields(v)
            }
        }

        let cmd: Command = match serde_json::from_value(cmd_value) {
            Ok(cmd) => cmd,
            Err(e) => {
                return Response::error(format!("Invalid command value: {}", e)).into_string()
            }
        };

        match cmd {
            #[cfg(feature = "internal")]
            Command::PasswordLogin(req) => self.0.auth().login_password(&req).await.into_string(),
            #[cfg(feature = "secrets")]
            Command::AccessTokenLogin(req) => {
                self.0.auth().login_access_token(&req).await.into_string()
            }
            #[cfg(feature = "internal")]
            Command::GetUserApiKey(req) => self.0.get_user_api_key(&req).await.into_string(),
            #[cfg(feature = "internal")]
            Command::ApiKeyLogin(req) => self.0.auth().login_api_key(&req).await.into_string(),
            #[cfg(feature = "internal")]
            Command::Sync(req) => self.0.sync(&req).await.into_string(),
            #[cfg(feature = "internal")]
            Command::Fingerprint(req) => self.0.fingerprint(&req).into_string(),

            #[cfg(feature = "secrets")]
            Command::Secrets(cmd) => match cmd {
                SecretsCommand::Get(req) => self.0.secrets().get(&req).await.into_string(),
                SecretsCommand::GetByIds(req) => {
                    self.0.secrets().get_by_ids(req).await.into_string()
                }
                SecretsCommand::Create(req) => self.0.secrets().create(&req).await.into_string(),
                SecretsCommand::List(req) => self.0.secrets().list(&req).await.into_string(),
                SecretsCommand::Update(req) => self.0.secrets().update(&req).await.into_string(),
                SecretsCommand::Delete(req) => self.0.secrets().delete(req).await.into_string(),
            },

            #[cfg(feature = "secrets")]
            Command::Projects(cmd) => match cmd {
                ProjectsCommand::Get(req) => self.0.projects().get(&req).await.into_string(),
                ProjectsCommand::Create(req) => self.0.projects().create(&req).await.into_string(),
                ProjectsCommand::List(req) => self.0.projects().list(&req).await.into_string(),
                ProjectsCommand::Update(req) => self.0.projects().update(&req).await.into_string(),
                ProjectsCommand::Delete(req) => self.0.projects().delete(req).await.into_string(),
            },
        }
    }

    fn parse_settings(settings_input: Option<String>) -> Option<ClientSettings> {
        if let Some(input) = settings_input.as_ref() {
            let mut value: serde_json::Value = match serde_json::from_str(input) {
                Ok(value) => value,
                Err(e) => {
                    log::error!("Failed to parse settings: {}", e);
                    return None;
                }
            };

            clean_null_fields(&mut value);

            match serde_json::from_value(value) {
                Ok(settings) => return Some(settings),
                Err(e) => {
                    log::error!("Failed to parse settings: {}", e);
                }
            }
        }
        None
    }
}

/// Removes null fields from a json object value, note that this isn't recursive.
/// This is needed because some of the language bindings will send null values when a value is not set,
/// and this causes problems with some deserializations, for example:
///
/// ClientSettings is using #[serde(default)] and non-optional fields, which means any missing
/// field will be deserialized as the value from ClientSettings::default(). This does not work if the value
/// is explicitly defined as null, which will produce a deserialization error.
///
/// The Command enum is serialized/deserialized as '{ "variant_name": {/* Variant contents */} }', serde
/// is expecting only a single variant field in the root object, but some of the bindings are sending the
/// rest of fields with null values, which will produce a deserialization error.
/// '{ "other_variant": null, "variant_name": {/* Variant contents */}, "another_variant": null }'
///
fn clean_null_fields(value: &mut serde_json::Value) {
    if let Some(object) = value.as_object_mut() {
        object.retain(|_, v| !v.is_null());
    }
}
