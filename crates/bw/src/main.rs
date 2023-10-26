use bitwarden::{
    admin_console::auth_requests::{AuthApproveRequest, PendingAuthRequestsRequest},
    auth::RegisterRequest,
    client::client_settings::ClientSettings,
    tool::PasswordGeneratorRequest,
    Client,
};
use bitwarden_cli::{install_color_eyre, text_prompt_when_none, Color};
use clap::{command, Args, CommandFactory, Parser, Subcommand};
use color_eyre::eyre::Result;
use inquire::Password;
use render::{serialize_response, Output};
use uuid::Uuid;

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

    #[command(long_about = "Register")]
    Register {
        #[arg(short = 'e', long, help = "Email address")]
        email: Option<String>,

        name: Option<String>,

        password_hint: Option<String>,

        #[arg(short = 's', long, global = true, help = "Server URL")]
        server: Option<String>,
    },

    #[command(long_about = "Manage vault items")]
    Item {
        #[command(subcommand)]
        command: ItemCommands,
    },

    #[command(long_about = "Pull the latest vault data from the server")]
    Sync {},

    #[command(long_about = "Password and passphrase generators")]
    Generate {
        #[command(subcommand)]
        command: GeneratorCommands,
    },

    #[command(long_about = "Manage your organization")]
    AdminConsole {
        #[command(subcommand)]
        command: AdminConsoleCommands,
    },
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

#[derive(Subcommand, Clone)]
enum GeneratorCommands {
    Password(PasswordGeneratorArgs),
    Passphrase {},
}

#[derive(Subcommand, Clone)]
enum AdminConsoleCommands {
    ListDevices {
        organization_id: Uuid,
    },
    ApproveDevice {
        organization_id: Uuid,
        organization_user_id: Uuid,
    },
}

#[derive(Args, Clone)]
struct PasswordGeneratorArgs {
    #[arg(short = 'l', long, action, help = "Include lowercase characters (a-z)")]
    lowercase: bool,

    #[arg(short = 'u', long, action, help = "Include uppercase characters (A-Z)")]
    uppercase: bool,

    #[arg(short = 'n', long, action, help = "Include numbers (0-9)")]
    numbers: bool,

    #[arg(
        short = 's',
        long,
        action,
        help = "Include special characters (!@#$%^&*)"
    )]
    special: bool,

    #[arg(long, default_value = "16", help = "Length of generated password")]
    length: u8,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    process_commands().await
}

async fn hack_login() -> Client {
    // hack login
    let server = "https://vault.qa.bitwarden.pw";
    let settings = ClientSettings {
        api_url: format!("{}/api", server),
        identity_url: format!("{}/identity", server),
        ..Default::default()
    };
    let client = bitwarden::Client::new(Some(settings));

    auth::api_key_login(client, None, None).await.unwrap()
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
                } => {
                    auth::api_key_login(client, client_id, client_secret).await?;
                }
            }
            return Ok(());
        }
        Commands::Register {
            email,
            name,
            password_hint,
            server,
        } => {
            let settings = server.map(|server| ClientSettings {
                api_url: format!("{}/api", server),
                identity_url: format!("{}/identity", server),
                ..Default::default()
            });
            let mut client = bitwarden::Client::new(settings);

            let email = text_prompt_when_none("Email", email)?;
            let password = Password::new("Password").prompt()?;

            client
                .auth()
                .register(&RegisterRequest {
                    email,
                    name,
                    password,
                    password_hint,
                })
                .await?;
        }
        _ => {}
    }

    // Not login, assuming we have a config
    let mut client = bitwarden::Client::new(None);

    // And finally we process all the commands which require authentication
    match command {
        Commands::Login(_) => unreachable!(),
        Commands::Register { .. } => unreachable!(),
        Commands::Item { command: _ } => todo!(),
        Commands::Sync {} => todo!(),
        Commands::Generate { command } => match command {
            GeneratorCommands::Password(args) => {
                let password = client
                    .generator()
                    .password(PasswordGeneratorRequest {
                        lowercase: args.lowercase,
                        uppercase: args.uppercase,
                        numbers: args.numbers,
                        special: args.special,
                        length: Some(args.length),
                        ..Default::default()
                    })
                    .await?;

                println!("{}", password);
            }
            GeneratorCommands::Passphrase {} => todo!(),
        },
        Commands::AdminConsole { command } => match command {
            AdminConsoleCommands::ListDevices { organization_id } => {
                let mut client = hack_login().await;
                let auth_requests = client
                    .client_auth_requests()
                    .list(&PendingAuthRequestsRequest { organization_id })
                    .await?;

                serialize_response(auth_requests.data, cli.output, false);
            }
            AdminConsoleCommands::ApproveDevice {
                organization_id,
                organization_user_id,
            } => {
                let mut client = hack_login().await;
                client
                    .client_auth_requests()
                    .approve(&AuthApproveRequest {
                        organization_id,
                        organization_user_id,
                    })
                    .await
                    .unwrap(); // error handling?
            }
        },
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
