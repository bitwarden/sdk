pub(crate) mod project;
pub(crate) mod secret;

use std::{path::PathBuf, str::FromStr};

use bitwarden::auth::AccessToken;
use clap::CommandFactory;
use clap_complete::Shell;
use color_eyre::eyre::{bail, eyre, Result};

use crate::{config, Cli, ProfileKey};

pub(crate) fn completions(shell: Option<Shell>) -> Result<()> {
    let Some(shell) = shell.or_else(Shell::from_env) else {
        bail!("Couldn't autodetect a valid shell. Run `bws completions --help` for more info.");
    };

    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());

    Ok(())
}

pub(crate) fn config(
    name: Option<ProfileKey>,
    value: Option<String>,
    delete: bool,
    profile: Option<String>,
    access_token: Option<String>,
    config_file: Option<PathBuf>,
) -> Result<()> {
    let profile = if let Some(profile) = profile {
        profile
    } else if let Some(access_token) = access_token {
        AccessToken::from_str(&access_token)?
            .access_token_id
            .to_string()
    } else {
        String::from("default")
    };

    if delete {
        config::delete_profile(config_file.as_deref(), profile)?;
        println!("Profile deleted successfully!");
    } else {
        let (name, mut value) = match (name, value) {
            (None, None) => bail!("Missing `name` and `value`"),
            (None, Some(_)) => bail!("Missing `value`"),
            (Some(_), None) => bail!("Missing `name`"),
            (Some(name), Some(value)) => (name, value),
        };

        // If state_opt_out is being set,
        // verify it's a boolean or 1 or 0, otherwise bail
        if let ProfileKey::state_opt_out = name {
            value = match string_to_bool_string(value) {
                Ok(value) => value,
                Err(_) => bail!("Profile key \"state_opt_out\" must be \"true\" or \"false\""),
            }
        }

        config::update_profile(config_file.as_deref(), profile, name, value)?;
        println!("Profile updated successfully!");
    };

    Ok(())
}

fn string_to_bool_string(value: String) -> Result<String> {
    match value.trim().to_lowercase().as_str() {
        "true" | "1" => Ok(String::from("true")),
        "false" | "0" => Ok(String::from("false")),
        _ => Err(eyre!("Failed to convert string to bool")),
    }
}
