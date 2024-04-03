use std::path::PathBuf;

use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use clap_complete::Shell;
use uuid::Uuid;

pub(crate) const ACCESS_TOKEN_KEY_VAR_NAME: &str = "BWS_ACCESS_TOKEN";
pub(crate) const CONFIG_FILE_KEY_VAR_NAME: &str = "BWS_CONFIG_FILE";
pub(crate) const PROFILE_KEY_VAR_NAME: &str = "BWS_PROFILE";
pub(crate) const SERVER_URL_KEY_VAR_NAME: &str = "BWS_SERVER_URL";

pub(crate) const DEFAULT_CONFIG_FILENAME: &str = "config";
pub(crate) const DEFAULT_CONFIG_DIRECTORY: &str = ".bws";

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub(crate) enum ProfileKey {
    server_base,
    server_api,
    server_identity,
    state_file_dir,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum Output {
    JSON,
    YAML,
    Env,
    Table,
    TSV,
    None,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub(crate) enum Color {
    No,
    Yes,
    Auto,
}

#[derive(Parser, Debug)]
#[command(name = "bws", version, about = "Bitwarden Secrets CLI", long_about = None)]
pub(crate) struct Cli {
    // Optional as a workaround for https://github.com/clap-rs/clap/issues/3572
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,

    #[arg(short = 'o', long, global = true, value_enum, default_value_t = Output::JSON, help="Output format")]
    pub(crate) output: Output,

    #[arg(short = 'c', long, global = true, value_enum, default_value_t = Color::Auto, help="Use colors in the output")]
    pub(crate) color: Color,

    #[arg(short = 't', long, global = true, env = ACCESS_TOKEN_KEY_VAR_NAME, hide_env_values = true, help="Specify access token for the service account")]
    pub(crate) access_token: Option<String>,

    #[arg(
        short = 'f',
        long,
        global = true,
        env = CONFIG_FILE_KEY_VAR_NAME,
        help = format!("[default: ~/{}/{}] Config file to use", DEFAULT_CONFIG_DIRECTORY, DEFAULT_CONFIG_FILENAME)
    )]
    pub(crate) config_file: Option<PathBuf>,

    #[arg(short = 'p', long, global = true, env = PROFILE_KEY_VAR_NAME, help="Profile to use from the config file")]
    pub(crate) profile: Option<String>,

    #[arg(short = 'u', long, global = true, env = SERVER_URL_KEY_VAR_NAME, help="Override the server URL from the config file")]
    pub(crate) server_url: Option<String>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    #[command(long_about = "Configure the CLI", arg_required_else_help(true))]
    Config {
        name: Option<ProfileKey>,
        value: Option<String>,

        #[arg(short = 'd', long)]
        delete: bool,
    },

    #[command(long_about = "Generate shell completion files")]
    Completions { shell: Option<Shell> },

    #[command(long_about = "Commands available on Projects")]
    Project {
        #[command(subcommand)]
        cmd: ProjectCommand,
    },
    #[command(long_about = "Commands available on Secrets")]
    Secret {
        #[command(subcommand)]
        cmd: SecretCommand,
    },
    #[command(long_about = "Create a single item (deprecated)", hide(true))]
    Create {
        #[command(subcommand)]
        cmd: CreateCommand,
    },
    #[command(long_about = "Delete one or more items (deprecated)", hide(true))]
    Delete {
        #[command(subcommand)]
        cmd: DeleteCommand,
    },
    #[command(long_about = "Edit a single item (deprecated)", hide(true))]
    Edit {
        #[command(subcommand)]
        cmd: EditCommand,
    },
    #[command(long_about = "Retrieve a single item (deprecated)", hide(true))]
    Get {
        #[command(subcommand)]
        cmd: GetCommand,
    },
    #[command(long_about = "List items (deprecated)", hide(true))]
    List {
        #[command(subcommand)]
        cmd: ListCommand,
    },
}

#[derive(Subcommand, Debug)]
pub(crate) enum SecretCommand {
    Create {
        key: String,
        value: String,

        #[arg(help = "The ID of the project this secret will be added to")]
        project_id: Uuid,

        #[arg(long, help = "An optional note to add to the secret")]
        note: Option<String>,
    },
    Delete {
        secret_ids: Vec<Uuid>,
    },
    #[clap(group = ArgGroup::new("edit_field").required(true).multiple(true))]
    Edit {
        secret_id: Uuid,
        #[arg(long, group = "edit_field")]
        key: Option<String>,
        #[arg(long, group = "edit_field")]
        value: Option<String>,
        #[arg(long, group = "edit_field")]
        note: Option<String>,
        #[arg(long, group = "edit_field")]
        project_id: Option<Uuid>,
    },
    Get {
        secret_id: Uuid,
    },
    List {
        project_id: Option<Uuid>,
    },
}

#[derive(Subcommand, Debug)]
pub(crate) enum ProjectCommand {
    Create {
        name: String,
    },
    Delete {
        project_ids: Vec<Uuid>,
    },
    Edit {
        project_id: Uuid,
        #[arg(long, group = "edit_field")]
        name: String,
    },
    Get {
        project_id: Uuid,
    },
    List,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ListCommand {
    Projects,
    Secrets { project_id: Option<Uuid> },
}

#[derive(Subcommand, Debug)]
pub(crate) enum GetCommand {
    Project { project_id: Uuid },
    Secret { secret_id: Uuid },
}

#[derive(Subcommand, Debug)]
pub(crate) enum CreateCommand {
    Project {
        name: String,
    },
    Secret {
        key: String,
        value: String,

        #[arg(long, help = "An optional note to add to the secret")]
        note: Option<String>,

        #[arg(long, help = "The ID of the project this secret will be added to")]
        project_id: Uuid,
    },
}

#[derive(Subcommand, Debug)]
pub(crate) enum EditCommand {
    #[clap(group = ArgGroup::new("edit_field").required(true).multiple(true))]
    Project {
        project_id: Uuid,
        #[arg(long, group = "edit_field")]
        name: String,
    },
    #[clap(group = ArgGroup::new("edit_field").required(true).multiple(true))]
    Secret {
        secret_id: Uuid,
        #[arg(long, group = "edit_field")]
        key: Option<String>,
        #[arg(long, group = "edit_field")]
        value: Option<String>,
        #[arg(long, group = "edit_field")]
        note: Option<String>,
        #[arg(long, group = "edit_field")]
        project_id: Option<Uuid>,
    },
}

#[derive(Subcommand, Debug)]
pub(crate) enum DeleteCommand {
    Project { project_ids: Vec<Uuid> },
    Secret { secret_ids: Vec<Uuid> },
}
