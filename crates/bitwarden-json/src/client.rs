#[cfg(feature = "secrets")]
use bitwarden::secrets_manager::{ClientProjectsExt, ClientSecretsExt};
#[cfg(feature = "internal")]
use bitwarden::vault::ClientVaultExt;
use bitwarden::{
    vault::{Cipher, CipherRepromptType, CipherType},
    ClientSettings,
};
use uuid::Uuid;

#[cfg(feature = "secrets")]
use crate::command::{ProjectsCommand, SecretsCommand};
use crate::{
    command::Command,
    response::{Response, ResponseIntoString},
};

pub struct Client(bitwarden::Client);

impl Client {
    pub async fn new(settings_input: Option<String>) -> Self {
        let settings = Self::parse_settings(settings_input);
        Self(bitwarden::Client::new(settings).await)
    }

    pub async fn run_command(&self, input_str: &str) -> String {
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

        let client = &self.0;

        let ciphers: Vec<Cipher> = (0..70000).map(|_| Cipher {
            id: Some(Uuid::new_v4()),
            organization_id: None,
            folder_id: None,
            collection_ids: vec![],
            key: None,
            name: "2.pMS6/icTQABtulw52pq2lg==|XXbxKxDTh+mWiN1HjH2N1w==|Q6PkuT+KX/axrgN9ubD5Ajk2YNwxQkgs3WJM0S0wtG8=".parse().unwrap(),
            notes: None,
            r#type: CipherType::Login,
            login: None,
            identity: None,
            card: None,
            secure_note: None,
            favorite: false,
            reprompt: CipherRepromptType::None,
            organization_use_totp: false,
            edit: true,
            view_password: true,
            local_data: None,
            attachments: None,
            fields: None,
            password_history: None,
            creation_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
            deleted_date: None,
            revision_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
        }).collect();
        client.vault().cipher_repository.replace_all(&ciphers).await;

        match cmd {
            #[cfg(feature = "internal")]
            Command::PasswordLogin(req) => client.auth().login_password(&req).await.into_string(),
            #[cfg(feature = "secrets")]
            Command::AccessTokenLogin(req) => {
                client.auth().login_access_token(&req).await.into_string()
            }
            #[cfg(feature = "internal")]
            Command::GetUserApiKey(req) => {
                client.platform().get_user_api_key(req).await.into_string()
            }
            #[cfg(feature = "internal")]
            Command::ApiKeyLogin(req) => client.auth().login_api_key(&req).await.into_string(),
            #[cfg(feature = "internal")]
            Command::Sync(req) => client.vault().sync(&req).await.into_string(),
            #[cfg(feature = "internal")]
            Command::Fingerprint(req) => client.platform().fingerprint(&req).into_string(),

            #[cfg(feature = "secrets")]
            Command::Secrets(cmd) => match cmd {
                SecretsCommand::Get(req) => client.secrets().get(&req).await.into_string(),
                SecretsCommand::GetByIds(req) => {
                    client.secrets().get_by_ids(req).await.into_string()
                }
                SecretsCommand::Create(req) => client.secrets().create(&req).await.into_string(),
                SecretsCommand::List(req) => client.secrets().list(&req).await.into_string(),
                SecretsCommand::Update(req) => client.secrets().update(&req).await.into_string(),
                SecretsCommand::Delete(req) => client.secrets().delete(req).await.into_string(),
                SecretsCommand::Sync(req) => client.secrets().sync(&req).await.into_string(),
            },

            #[cfg(feature = "secrets")]
            Command::Projects(cmd) => match cmd {
                ProjectsCommand::Get(req) => client.projects().get(&req).await.into_string(),
                ProjectsCommand::Create(req) => client.projects().create(&req).await.into_string(),
                ProjectsCommand::List(req) => client.projects().list(&req).await.into_string(),
                ProjectsCommand::Update(req) => client.projects().update(&req).await.into_string(),
                ProjectsCommand::Delete(req) => client.projects().delete(req).await.into_string(),
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
