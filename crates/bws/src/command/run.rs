use std::{
    io::{IsTerminal, Read},
    process,
};

use bitwarden::{
    secrets_manager::{
        secrets::{SecretIdentifiersByProjectRequest, SecretIdentifiersRequest, SecretsGetRequest},
        ClientSecretsExt,
    },
    Client,
};
use clap::CommandFactory;
use color_eyre::eyre::Result;
use uuid::Uuid;
use which::which;

use crate::{
    util::{is_valid_posix_name, uuid_to_posix},
    Cli, ACCESS_TOKEN_KEY_VAR_NAME,
};

pub(crate) async fn run(
    client: Client,
    organization_id: Uuid,
    project_id: Option<Uuid>,
    uuids_as_keynames: bool,
    no_inherit_env: bool,
    shell: Option<String>,
    command: Vec<String>,
) -> Result<()> {
    let shell = match std::env::consts::OS {
        "windows" => shell.unwrap_or_else(|| "powershell".to_string()),
        _ => shell.unwrap_or_else(|| "sh".to_string()),
    };

    if which(&shell).is_err() {
        eprintln!("Error: shell '{}' not found", shell);
        std::process::exit(1);
    }

    let user_command = if command.is_empty() {
        if std::io::stdin().is_terminal() {
            eprintln!("{}", Cli::command().render_help().ansi());
            std::process::exit(1);
        }

        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        command.join(" ")
    };

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
        let key = if uuids_as_keynames {
            uuid_to_posix(&s.id)
        } else {
            s.key
        };

        if !is_valid_posix_name(&key) {
            eprintln!(
                "Warning: secret '{}' does not have a POSIX-compliant name",
                key
            );
        }

        match environment.contains_key(&key) {
            true => {
                eprintln!(
                    "Error: multiple secrets with name '{}' found. Use --uuids-as-keynames or use unique names for secrets.",
                    key
                );
                std::process::exit(1);
            }
            false => {
                environment.insert(key, s.value);
            }
        }
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

    Ok(())
}
