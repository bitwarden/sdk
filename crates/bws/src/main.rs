use std::{path::PathBuf, str::FromStr};

use bitwarden::{
    auth::request::AccessTokenLoginRequest,
    client::AccessToken,
    sdk::request::client_settings::ClientSettings,
    secrets_manager::{
        projects::{
            ProjectCreateRequest, ProjectGetRequest, ProjectPutRequest, ProjectsDeleteRequest,
            ProjectsListRequest,
        },
        secrets::{
            SecretCreateRequest, SecretGetRequest, SecretIdentifiersByProjectRequest,
            SecretIdentifiersRequest, SecretPutRequest, SecretsDeleteRequest,
        },
    },
};
use clap::{ArgGroup, CommandFactory, Parser, Subcommand};
use color_eyre::eyre::{bail, Result};
use log::error;

mod config;
mod render;

use config::ProfileKey;
use render::{serialize_response, Color, Output};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(name = "Bitwarden Secrets CLI", version, about = "Bitwarden Secrets CLI", long_about = None)]
struct Cli {
    // Optional as a workaround for https://github.com/clap-rs/clap/issues/3572
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short = 'o', long, global = true, value_enum, default_value_t = Output::JSON)]
    output: Output,

    #[arg(short = 'c', long, global = true, value_enum, default_value_t = Color::Auto)]
    color: Color,

    #[arg(short = 't', long, global = true, env = ACCESS_TOKEN_KEY_VAR_NAME, hide_env_values = true, help="Specify access token for the service account")]
    access_token: Option<String>,

    #[arg(
        short = 'f',
        long,
        global = true,
        help = format!("[default: ~/{}/{}] Config file to use", config::DIRECTORY, config::FILENAME)
    )]
    config_file: Option<PathBuf>,

    #[arg(short = 'p', long, global = true, env = PROFILE_KEY_VAR_NAME, help="Profile to use from the config file")]
    profile: Option<String>,

    #[arg(short = 'u', long, global = true, env = SERVER_URL_KEY_VAR_NAME, help="Override the server URL from the config file")]
    server_url: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(long_about = "List items")]
    List {
        #[command(subcommand)]
        cmd: ListCommand,
    },
    #[command(long_about = "Retrieve a single item")]
    Get {
        #[command(subcommand)]
        cmd: GetCommand,
    },
    #[command(long_about = "Create a single item")]
    Create {
        #[command(subcommand)]
        cmd: CreateCommand,
    },
    #[command(long_about = "Edit a single item")]
    Edit {
        #[command(subcommand)]
        cmd: EditCommand,
    },
    #[command(long_about = "Delete one or more items")]
    Delete {
        #[command(subcommand)]
        cmd: DeleteCommand,
    },
    #[command(long_about = "Configure the CLI", arg_required_else_help(true))]
    Config {
        name: Option<ProfileKey>,
        value: Option<String>,

        #[arg(short = 'd', long)]
        delete: bool,
    },
}

#[derive(Subcommand, Debug)]
enum ListCommand {
    Projects,
    Secrets { project_id: Option<Uuid> },
}

#[derive(Subcommand, Debug)]
enum GetCommand {
    Project { project_id: Uuid },
    Secret { secret_id: Uuid },
}

#[derive(Subcommand, Debug)]
enum CreateCommand {
    Secret {
        key: String,
        value: String,

        #[arg(long, help = "An optional note to add to the secret")]
        note: Option<String>,

        #[arg(long, help = "The ID of the project this secret will be added to")]
        project_id: Option<Uuid>,
    },
    Project {
        name: String,
    },
}

#[derive(Subcommand, Debug)]
enum EditCommand {
    #[clap(group = ArgGroup::new("edit_field").required(true).multiple(true))]
    Secret {
        secret_id: Uuid,
        #[arg(long, group = "edit_field")]
        key: Option<String>,
        #[arg(long, group = "edit_field")]
        value: Option<String>,
        #[arg(long, group = "edit_field")]
        note: Option<String>,
    },
    #[clap(group = ArgGroup::new("edit_field").required(true).multiple(true))]
    Project {
        project_id: Uuid,
        #[arg(long, group = "edit_field")]
        name: String,
    },
}

#[derive(Subcommand, Debug)]
enum DeleteCommand {
    Secret { secret_ids: Vec<Uuid> },
    Project { project_ids: Vec<Uuid> },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    process_commands().await
}

const ACCESS_TOKEN_KEY_VAR_NAME: &str = "BWS_ACCESS_TOKEN";
const PROFILE_KEY_VAR_NAME: &str = "BWS_PROFILE";
const SERVER_URL_KEY_VAR_NAME: &str = "BWS_SERVER_URL";

async fn process_commands() -> Result<()> {
    let cli = Cli::parse();

    let color = cli.color.is_enabled();
    if color {
        color_eyre::install()?;
    } else {
        // Use an empty theme to disable error coloring
        color_eyre::config::HookBuilder::new()
            .theme(color_eyre::config::Theme::new())
            .install()?;
    }

    let Some(command) = cli.command else {
        let mut cmd = Cli::command();
        cmd.print_help()?;
        return Ok(());
    };

    // Modify profile commands
    if let Commands::Config {
        name,
        value,
        delete,
    } = command
    {
        let profile = if let Some(profile) = cli.profile {
            profile
        } else if let Some(access_token) = cli.access_token {
            AccessToken::from_str(&access_token)?
                .service_account_id
                .to_string()
        } else {
            String::from("default")
        };

        if delete {
            config::delete_profile(cli.config_file.as_deref(), profile)?;
            println!("Profile deleted successfully!");
        } else {
            let (name, value) = match (name, value) {
                (None, None) => bail!("Missing `name` and `value`"),
                (None, Some(_)) => bail!("Missing `value`"),
                (Some(_), None) => bail!("Missing `name`"),
                (Some(name), Some(value)) => (name, value),
            };

            config::update_profile(cli.config_file.as_deref(), profile, name, value)?;
            println!("Profile updated successfully!");
        };

        return Ok(());
    }

    let access_token = match cli.access_token {
        Some(key) => key,
        None => bail!("Missing access token"),
    };

    let profile = get_config_profile(
        &cli.server_url,
        &cli.profile,
        &cli.config_file,
        &access_token,
    )?;

    let settings = profile
        .map(|p| -> Result<_> {
            Ok(ClientSettings {
                identity_url: p.identity_url()?,
                api_url: p.api_url()?,
                ..Default::default()
            })
        })
        .transpose()?;

    let mut client = bitwarden::Client::new(settings);

    // Load session or return if no session exists
    let _ = client
        .access_token_login(&AccessTokenLoginRequest {
            access_token: access_token,
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
        Commands::List {
            cmd: ListCommand::Projects,
        } => {
            let projects = client
                .projects()
                .list(&ProjectsListRequest {
                    organization_id: organization_id.clone(),
                })
                .await?
                .data;
            serialize_response(projects, cli.output, color);
        }

        Commands::List {
            cmd: ListCommand::Secrets { project_id },
        } => {
            let res = if let Some(project_id) = project_id {
                client
                    .secrets()
                    .list_by_project(&SecretIdentifiersByProjectRequest {
                        project_id: project_id,
                    })
                    .await?
            } else {
                client
                    .secrets()
                    .list(&SecretIdentifiersRequest {
                        organization_id: organization_id.clone(),
                    })
                    .await?
            };

            let mut secrets = Vec::new();

            for s in res.data {
                let secret = client.secrets().get(&SecretGetRequest { id: s.id }).await?;
                secrets.push(secret);
            }
            serialize_response(secrets, cli.output, color);
        }

        Commands::Get {
            cmd: GetCommand::Project { project_id },
        } => {
            let project = client
                .projects()
                .get(&ProjectGetRequest { id: project_id })
                .await?;
            serialize_response(project, cli.output, color);
        }

        Commands::Create {
            cmd: CreateCommand::Project { name },
        } => {
            let project = client
                .projects()
                .create(&ProjectCreateRequest {
                    organization_id,
                    name,
                })
                .await?;
            serialize_response(project, cli.output, color);
        }

        Commands::Edit {
            cmd: EditCommand::Project { project_id, name },
        } => {
            let project = client
                .projects()
                .update(&ProjectPutRequest {
                    id: project_id,
                    organization_id,
                    name,
                })
                .await?;
            serialize_response(project, cli.output, color);
        }

        Commands::Delete {
            cmd: DeleteCommand::Project { project_ids },
        } => {
            let project_count = project_ids.len();

            client
                .projects()
                .delete(ProjectsDeleteRequest { ids: project_ids })
                .await?;

            if project_count > 1 {
                println!("Projects deleted successfully.");
            } else {
                println!("Project deleted successfully.");
            }
        }

        Commands::Get {
            cmd: GetCommand::Secret { secret_id },
        } => {
            let secret = client
                .secrets()
                .get(&SecretGetRequest { id: secret_id })
                .await?;
            serialize_response(secret, cli.output, color);
        }

        Commands::Create {
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
                    project_ids: project_id.map(|p| vec![p]),
                })
                .await?;
            serialize_response(secret, cli.output, color);
        }

        Commands::Edit {
            cmd:
                EditCommand::Secret {
                    secret_id,
                    key,
                    value,
                    note,
                },
        } => {
            let old_secret = client
                .secrets()
                .get(&SecretGetRequest {
                    id: secret_id.clone(),
                })
                .await?;

            let secret = client
                .secrets()
                .update(&SecretPutRequest {
                    id: secret_id,
                    organization_id,
                    key: key.unwrap_or(old_secret.key),
                    value: value.unwrap_or(old_secret.value),
                    note: note.unwrap_or(old_secret.note),
                })
                .await?;
            serialize_response(secret, cli.output, color);
        }

        Commands::Delete {
            cmd: DeleteCommand::Secret { secret_ids },
        } => {
            client
                .secrets()
                .delete(SecretsDeleteRequest { ids: secret_ids })
                .await?;

            println!("Secret deleted correctly");
        }

        Commands::Config { .. } => {
            unreachable!()
        }
    }

    Ok(())
}

fn get_config_profile(
    server_url: &Option<String>,
    profile: &Option<String>,
    config_file: &Option<PathBuf>,
    access_token: &String,
) -> Result<Option<config::Profile>, color_eyre::Report> {
    let profile = if let Some(server_url) = server_url {
        Some(config::Profile::from_url(server_url)?)
    } else {
        let profile_defined = profile.is_some();

        let profile_key = if let Some(profile) = profile {
            profile.to_owned()
        } else {
            AccessToken::from_str(access_token)?
                .service_account_id
                .to_string()
        };

        let config = config::load_config(config_file.as_deref(), config_file.is_some())?;
        config.select_profile(&profile_key, profile_defined)?
    };
    Ok(profile)
}
