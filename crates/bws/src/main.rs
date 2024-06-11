use std::{io::Read, path::PathBuf, process, str::FromStr};

use atty::Stream;
use bitwarden::{
    auth::{login::AccessTokenLoginRequest, AccessToken},
    client::client_settings::ClientSettings,
    secrets_manager::{
        projects::{
            ProjectCreateRequest, ProjectGetRequest, ProjectPutRequest, ProjectsDeleteRequest,
            ProjectsListRequest,
        },
        secrets::{
            SecretCreateRequest, SecretGetRequest, SecretIdentifiersByProjectRequest,
            SecretIdentifiersRequest, SecretPutRequest, SecretsDeleteRequest, SecretsGetRequest,
        },
    },
};
use bitwarden_cli::install_color_eyre;
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use color_eyre::eyre::{bail, Result};
use log::error;
use uuid::Uuid;

mod cli;
mod config;
mod render;
mod state;
mod util;

use util::is_valid_posix_name;
use which::which;

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
            let Some(shell) = shell.or_else(Shell::from_env) else {
                eprintln!("Couldn't autodetect a valid shell. Run `bws completions --help` for more info.");
                std::process::exit(1);
            };

            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
            return Ok(());
        }
        Commands::Config {
            name,
            value,
            delete,
        } => {
            let profile = if let Some(profile) = cli.profile {
                profile
            } else if let Some(access_token) = cli.access_token {
                AccessToken::from_str(&access_token)?
                    .access_token_id
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
            let projects = client
                .projects()
                .list(&ProjectsListRequest { organization_id })
                .await?
                .data;
            serialize_response(projects, cli.output, color);
        }

        Commands::Project {
            cmd: ProjectCommand::Get { project_id },
        }
        | Commands::Get {
            cmd: GetCommand::Project { project_id },
        } => {
            let project = client
                .projects()
                .get(&ProjectGetRequest { id: project_id })
                .await?;
            serialize_response(project, cli.output, color);
        }

        Commands::Project {
            cmd: ProjectCommand::Create { name },
        }
        | Commands::Create {
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

        Commands::Project {
            cmd: ProjectCommand::Edit { project_id, name },
        }
        | Commands::Edit {
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

        Commands::Project {
            cmd: ProjectCommand::Delete { project_ids },
        }
        | Commands::Delete {
            cmd: DeleteCommand::Project { project_ids },
        } => {
            let count = project_ids.len();

            let result = client
                .projects()
                .delete(ProjectsDeleteRequest { ids: project_ids })
                .await?;

            let projects_failed: Vec<(Uuid, String)> = result
                .data
                .into_iter()
                .filter_map(|r| r.error.map(|e| (r.id, e)))
                .collect();
            let deleted_projects = count - projects_failed.len();

            if deleted_projects > 1 {
                println!("{} projects deleted successfully.", deleted_projects);
            } else if deleted_projects == 1 {
                println!("{} project deleted successfully.", deleted_projects);
            }

            if projects_failed.len() > 1 {
                eprintln!("{} projects had errors:", projects_failed.len());
            } else if projects_failed.len() == 1 {
                eprintln!("{} project had an error:", projects_failed.len());
            }

            for project in &projects_failed {
                eprintln!("{}: {}", project.0, project.1);
            }

            if !projects_failed.is_empty() {
                process::exit(1);
            }
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

        Commands::Run {
            command,
            shell,
            no_inherit_env,
            project_id,
        } => {
            let shell = match std::env::consts::OS {
                "windows" => shell.unwrap_or_else(|| "powershell".to_string()),
                _ => shell.unwrap_or_else(|| "sh".to_string()),
            };

            if which(&shell).is_err() {
                eprintln!("Error: shell '{}' not found", shell);
                std::process::exit(1);
            }

            let user_command = if command.is_empty() {
                if atty::is(Stream::Stdin) {
                    eprintln!("{}", Cli::command().render_help().ansi());
                    std::process::exit(1);
                }

                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer)?;
                buffer
            } else {
                command.join(" ")
            };

            if user_command.is_empty() {
                let mut cmd = Cli::command();
                eprintln!("{}", cmd.render_help().ansi());
                std::process::exit(1);
            }

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

            let mut environment = std::collections::HashMap::new();

            secrets.into_iter().for_each(|s| {
                if !is_valid_posix_name(&s.key) {
                    eprintln!(
                        "Warning: secret '{}' does not have a POSIX-compliant name",
                        s.key
                    );
                }
                if let Some(_) = environment.insert(s.key.clone(), s.value) {
                    eprintln!("Error: multiple secrets with name '{}' found. Use --uuids-as-keynames or use unique names for secrets.",
                    s.key);
                    std::process::exit(1);
                };
            });

            let mut command = process::Command::new(shell);
            command
                .arg("-c")
                .arg(&user_command)
                .stdout(process::Stdio::inherit())
                .stderr(process::Stdio::inherit());

            if no_inherit_env {
                let path = std::env::var("PATH").unwrap_or_else(|_| "/bin:/usr/bin".to_string());
                command.env_clear();
                command.env("PATH", path); // PATH is always necessary
                command.envs(&environment);
            } else {
                command.env_remove(ACCESS_TOKEN_KEY_VAR_NAME);
                command.envs(&environment);
            }

            let mut child = command.spawn().expect("failed to execute process");

            let exit_status = child.wait().expect("process failed to execute");
            let exit_code = exit_status.code().unwrap_or(1);

            if exit_code != 0 {
                process::exit(exit_code);
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
