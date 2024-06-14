use std::{path::PathBuf, str::FromStr};

use bitwarden::{
    auth::{login::AccessTokenLoginRequest, AccessToken},
    client::client_settings::ClientSettings,
};
use bitwarden_cli::install_color_eyre;
use clap::{CommandFactory, Parser};
use color_eyre::eyre::{bail, Result};
use command::secret::{SecretCreateCommandModel, SecretEditCommandModel};
use log::error;
use render::OutputSettings;

mod cli;
mod command;
mod config;
mod render;
mod state;

use crate::cli::*;

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

    let client = bitwarden::Client::new(settings);

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

    let output_settings = OutputSettings::new(cli.output, color);

    // And finally we process all the commands which require authentication
    match command {
        Commands::Project {
            cmd: ProjectCommand::List,
        }
        | Commands::List {
            cmd: ListCommand::Projects,
        } => command::project::list(client, organization_id, output_settings).await,

        Commands::Project {
            cmd: ProjectCommand::Get { project_id },
        }
        | Commands::Get {
            cmd: GetCommand::Project { project_id },
        } => command::project::get(client, project_id, output_settings).await,

        Commands::Project {
            cmd: ProjectCommand::Create { name },
        }
        | Commands::Create {
            cmd: CreateCommand::Project { name },
        } => command::project::create(client, organization_id, name, output_settings).await,

        Commands::Project {
            cmd: ProjectCommand::Edit { project_id, name },
        }
        | Commands::Edit {
            cmd: EditCommand::Project { project_id, name },
        } => {
            command::project::edit(client, organization_id, project_id, name, output_settings).await
        }

        Commands::Project {
            cmd: ProjectCommand::Delete { project_ids },
        }
        | Commands::Delete {
            cmd: DeleteCommand::Project { project_ids },
        } => command::project::delete(client, project_ids).await,

        Commands::Secret {
            cmd: SecretCommand::List { project_id },
        }
        | Commands::List {
            cmd: ListCommand::Secrets { project_id },
        } => command::secret::list(client, organization_id, project_id, output_settings).await,

        Commands::Secret {
            cmd: SecretCommand::Get { secret_id },
        }
        | Commands::Get {
            cmd: GetCommand::Secret { secret_id },
        } => command::secret::get(client, secret_id, output_settings).await,

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
            command::secret::create(
                client,
                organization_id,
                SecretCreateCommandModel {
                    key,
                    value,
                    note,
                    project_id,
                },
                output_settings,
            )
            .await
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
            command::secret::edit(
                client,
                organization_id,
                SecretEditCommandModel {
                    id: secret_id,
                    key,
                    value,
                    note,
                    project_id,
                },
                output_settings,
            )
            .await
        }

        Commands::Secret {
            cmd: SecretCommand::Delete { secret_ids },
        }
        | Commands::Delete {
            cmd: DeleteCommand::Secret { secret_ids },
        } => command::secret::delete(client, secret_ids).await,

        Commands::Config { .. } | Commands::Completions { .. } => {
            unreachable!()
        }
    }
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
