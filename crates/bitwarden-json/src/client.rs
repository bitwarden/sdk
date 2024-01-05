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

        if let Some(cmd_value_map) = cmd_value.as_object_mut() {
            cmd_value_map.retain(|_, v| !v.is_null());

            for &subcommand in SUBCOMMANDS_TO_CLEAN {
                if let Some(cmd_value_secrets) = cmd_value_map
                    .get_mut(subcommand)
                    .and_then(|v| v.as_object_mut())
                {
                    cmd_value_secrets.retain(|_, v| !v.is_null());
                }
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
            Command::Fingerprint(req) => self.0.platform().fingerprint(&req).into_string(),

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
            match serde_json::from_str(input) {
                Ok(settings) => return Some(settings),
                Err(e) => {
                    log::error!("Failed to parse settings: {}", e);
                }
            }
        }
        None
    }
}
