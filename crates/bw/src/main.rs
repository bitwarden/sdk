use bitwarden::client::client_settings::ClientSettings;
use bitwarden_cli::{install_color_eyre, Color};
use clap::{command, Args, CommandFactory, Parser, Subcommand};
use color_eyre::eyre::Result;
use render::Output;

mod auth;
mod render;

#[derive(Parser, Clone)]
#[command(name = "Bitwarden CLI", version, about = "Bitwarden CLI", long_about = None)]
struct Cli {
    // Optional as a workaround for https://github.com/clap-rs/clap/issues/3572
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short = 'o', long, global = true, value_enum, default_value_t = Output::JSON)]
    output: Output,

    #[arg(short = 'c', long, global = true, value_enum, default_value_t = Color::Auto)]
    color: Color,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Login(LoginArgs),

    #[command(long_about = "Manage vault items")]
    Item {
        #[command(subcommand)]
        command: ItemCommands,
    },

    #[command(long_about = "Pull the latest vault data from the server")]
    Sync {},
}

#[derive(Args, Clone)]
struct LoginArgs {
    #[command(subcommand)]
    command: LoginCommands,

    #[arg(short = 's', long, global = true, help = "Server URL")]
    server: Option<String>,
}

#[derive(Subcommand, Clone)]
enum LoginCommands {
    Password {
        #[arg(short = 'e', long, help = "Email address")]
        email: Option<String>,
    },
    ApiKey {
        client_id: Option<String>,
        client_secret: Option<String>,
    },
}

#[derive(Subcommand, Clone)]
enum ItemCommands {
    Get { id: String },
    Create {},
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    process_commands().await
}

async fn process_commands() -> Result<()> {
    let cli = Cli::parse();

    install_color_eyre(cli.color)?;

    let Some(command) = cli.command else {
        let mut cmd = Cli::command();
        cmd.print_help()?;
        return Ok(());
    };

    match command.clone() {
        Commands::Login(args) => {
            let settings = args.server.map(|server| ClientSettings {
                api_url: format!("{}/api", server),
                identity_url: format!("{}/identity", server),
                ..Default::default()
            });
            let client = bitwarden::Client::new(settings);

            match args.command {
                // FIXME: Rust CLI will not support password login!
                LoginCommands::Password { email } => {
                    auth::password_login(client, email).await?;
                }
                LoginCommands::ApiKey {
                    client_id,
                    client_secret,
                } => auth::api_key_login(client, client_id, client_secret).await?,
            }
            return Ok(());
        }
        _ => {}
    };

    // Not login, assuming we have a config
    let mut _client = bitwarden::Client::new(None);

    // And finally we process all the commands which require authentication
    match command {
        Commands::Login(_) => unreachable!(),
        Commands::Item { command: _ } => todo!(),
        Commands::Sync {} => todo!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
