use std::{path::PathBuf, process, str::FromStr};

use bitwarden::{
    auth::{login::AccessTokenLoginRequest, AccessToken},
    client::client_settings::ClientSettings,
    secrets_manager::secrets::{
        SecretCreateRequest, SecretGetRequest, SecretIdentifiersByProjectRequest,
        SecretIdentifiersRequest, SecretPutRequest, SecretsDeleteRequest, SecretsGetRequest,
    },
};
use bitwarden_cli::install_color_eyre;
use clap::{CommandFactory, Parser};
use color_eyre::eyre::{bail, Result};
use log::error;
use uuid::Uuid;

mod cli;
mod command;
mod config;
mod render;
mod state;

use crate::{cli::*, render::serialize_response};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    process_commands().await
}

#[allow(clippy::comparison_chain)]
async fn process_commands() -> Result<()> {
    let cli = Cli::parse();
    let color = cli.color;

    install_color_eyre(color)?;

    let Some(command) = cli.command else {
        let mut cmd = Cli::command();
        eprintln!("{}", cmd.render_help().ansi());
        std::process::exit(1);
    };

    // These commands don't require authentication, so we process them first
    match command {
        Commands::Completions { shell } => {
            return command::completions(shell);
        }
        Commands::Config {
            name,
            value,
            delete,
        } => {
            return command::config(
                name,
                value,
                delete,
                cli.profile,
                cli.access_token,
                cli.config_file,
            );
        }
        _ => (),
    }

    let access_token = match cli.access_token {
        Some(key) => key,
        None => bail!("Missing access token"),
    };
    let access_token_obj: AccessToken = access_token.parse()?;

    let profile = get_config_profile(
        &cli.server_url,
        &cli.profile,
        &cli.config_file,
        &access_token,
    )?;

    let settings = profile
        .clone()
        .map(|p| -> Result<_> {
            Ok(ClientSettings {
                identity_url: p.identity_url()?,
                api_url: p.api_url()?,
                ..Default::default()
            })
        })
        .transpose()?;

    let state_file_path = state::get_state_file_path(
        profile.and_then(|p| p.state_file_dir).map(Into::into),
        access_token_obj.access_token_id.to_string(),
    )?;

    let mut client = bitwarden::Client::new(settings);

    // Load session or return if no session exists
    let _ = client
        .auth()
        .login_access_token(&AccessTokenLoginRequest {
            access_token,
            state_file: state_file_path,
        })
        .await?;

    let organization_id = match client.get_access_token_organization() {
        Some(id) => id,
        None => {
            error!("Access token isn't associated to an organization.");
            return Ok(());
        }
    };

    // And finally we process all the commands which require authentication
    match command {
        Commands::Project {
            cmd: ProjectCommand::List,
        }
        | Commands::List {
            cmd: ListCommand::Projects,
        } => {
            return command::project::list(client, organization_id, cli.output, color).await;
        }

        Commands::Project {
            cmd: ProjectCommand::Get { project_id },
        }
        | Commands::Get {
            cmd: GetCommand::Project { project_id },
        } => {
            return command::project::get(client, project_id, cli.output, color).await;
        }

        Commands::Project {
            cmd: ProjectCommand::Create { name },
        }
        | Commands::Create {
            cmd: CreateCommand::Project { name },
        } => {
            return command::project::create(client, organization_id, name, cli.output, color)
                .await;
        }

        Commands::Project {
            cmd: ProjectCommand::Edit { project_id, name },
        }
        | Commands::Edit {
            cmd: EditCommand::Project { project_id, name },
        } => {
            return command::project::edit(
                client,
                organization_id,
                project_id,
                name,
                cli.output,
                color,
            )
            .await;
        }

        Commands::Project {
            cmd: ProjectCommand::Delete { project_ids },
        }
        | Commands::Delete {
            cmd: DeleteCommand::Project { project_ids },
        } => {
            return command::project::delete(client, project_ids).await;
        }

        Commands::Secret {
            cmd: SecretCommand::List { project_id },
        }
        | Commands::List {
            cmd: ListCommand::Secrets { project_id },
        } => {
            let res = if let Some(project_id) = project_id {
                client
                    .secrets()
                    .list_by_project(&SecretIdentifiersByProjectRequest { project_id })
                    .await?
            } else {
                client
                    .secrets()
                    .list(&SecretIdentifiersRequest { organization_id })
                    .await?
            };

            let secret_ids = res.data.into_iter().map(|e| e.id).collect();
            let secrets = client
                .secrets()
                .get_by_ids(SecretsGetRequest { ids: secret_ids })
                .await?
                .data;
            serialize_response(secrets, cli.output, color);
        }

        Commands::Secret {
            cmd: SecretCommand::Get { secret_id },
        }
        | Commands::Get {
            cmd: GetCommand::Secret { secret_id },
        } => {
            let secret = client
                .secrets()
                .get(&SecretGetRequest { id: secret_id })
                .await?;
            serialize_response(secret, cli.output, color);
        }

        Commands::Secret {
            cmd:
                SecretCommand::Create {
                    key,
                    value,
                    note,
                    project_id,
                },
        }
        | Commands::Create {
            cmd:
                CreateCommand::Secret {
                    key,
                    value,
                    note,
                    project_id,
                },
        } => {
            let secret = client
                .secrets()
                .create(&SecretCreateRequest {
                    organization_id,
                    key,
                    value,
                    note: note.unwrap_or_default(),
                    project_ids: Some(vec![project_id]),
                })
                .await?;
            serialize_response(secret, cli.output, color);
        }

        Commands::Secret {
            cmd:
                SecretCommand::Edit {
                    secret_id,
                    key,
                    value,
                    note,
                    project_id,
                },
        }
        | Commands::Edit {
            cmd:
                EditCommand::Secret {
                    secret_id,
                    key,
                    value,
                    note,
                    project_id,
                },
        } => {
            let old_secret = client
                .secrets()
                .get(&SecretGetRequest { id: secret_id })
                .await?;

            let secret = client
                .secrets()
                .update(&SecretPutRequest {
                    id: secret_id,
                    organization_id,
                    key: key.unwrap_or(old_secret.key),
                    value: value.unwrap_or(old_secret.value),
                    note: note.unwrap_or(old_secret.note),
                    project_ids: match project_id {
                        Some(id) => Some(vec![id]),
                        None => match old_secret.project_id {
                            Some(id) => Some(vec![id]),
                            None => bail!("Editing a secret requires a project_id."),
                        },
                    },
                })
                .await?;
            serialize_response(secret, cli.output, color);
        }

        Commands::Secret {
            cmd: SecretCommand::Delete { secret_ids },
        }
        | Commands::Delete {
            cmd: DeleteCommand::Secret { secret_ids },
        } => {
            let count = secret_ids.len();

            let result = client
                .secrets()
                .delete(SecretsDeleteRequest { ids: secret_ids })
                .await?;

            let secrets_failed: Vec<(Uuid, String)> = result
                .data
                .into_iter()
                .filter_map(|r| r.error.map(|e| (r.id, e)))
                .collect();
            let deleted_secrets = count - secrets_failed.len();

            if deleted_secrets > 1 {
                println!("{} secrets deleted successfully.", deleted_secrets);
            } else if deleted_secrets == 1 {
                println!("{} secret deleted successfully.", deleted_secrets);
            }

            if secrets_failed.len() > 1 {
                eprintln!("{} secrets had errors:", secrets_failed.len());
            } else if secrets_failed.len() == 1 {
                eprintln!("{} secret had an error:", secrets_failed.len());
            }

            for secret in &secrets_failed {
                eprintln!("{}: {}", secret.0, secret.1);
            }

            if !secrets_failed.is_empty() {
                process::exit(1);
            }
        }

        Commands::Config { .. } | Commands::Completions { .. } => {
            unreachable!()
        }
    }

    Ok(())
}

fn get_config_profile(
    server_url: &Option<String>,
    profile: &Option<String>,
    config_file: &Option<PathBuf>,
    access_token: &str,
) -> Result<Option<config::Profile>, color_eyre::Report> {
    let profile = if let Some(server_url) = server_url {
        Some(config::Profile::from_url(server_url)?)
    } else {
        let profile_defined = profile.is_some();

        let profile_key = if let Some(profile) = profile {
            profile.to_owned()
        } else {
            AccessToken::from_str(access_token)?
                .access_token_id
                .to_string()
        };

        let config = config::load_config(config_file.as_deref(), config_file.is_some())?;
        config.select_profile(&profile_key, profile_defined)?
    };
    Ok(profile)
}
