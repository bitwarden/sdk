use bitwarden::{
    auth::RegisterRequest,
    generators::{ClientGeneratorExt, PassphraseGeneratorRequest, PasswordGeneratorRequest},
    ClientSettings,
};
use bitwarden_cli::{install_color_eyre, text_prompt_when_none, Color};
use clap::{command, Args, CommandFactory, Parser, Subcommand};
use color_eyre::eyre::Result;
use inquire::Password;
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
    Device {
        #[arg(short = 'e', long, help = "Email address")]
        email: Option<String>,
        device_identifier: Option<String>,
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
    Passphrase(PassphraseGeneratorArgs),
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

#[derive(Args, Clone)]
struct PassphraseGeneratorArgs {
    #[arg(long, default_value = "3", help = "Number of words in the passphrase")]
    words: u8,
    #[arg(long, default_value = " ", help = "Separator between words")]
    separator: char,
    #[arg(long, action, help = "Capitalize the first letter of each word")]
    capitalize: bool,
    #[arg(long, action, help = "Include a number in one of the words")]
    include_number: bool,
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
                    auth::login_password(client, email).await?;
                }
                LoginCommands::ApiKey {
                    client_id,
                    client_secret,
                } => auth::login_api_key(client, client_id, client_secret).await?,
                LoginCommands::Device {
                    email,
                    device_identifier,
                } => {
                    auth::login_device(client, email, device_identifier).await?;
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
            let client = bitwarden::Client::new(settings);

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
    let client = bitwarden::Client::new(None);

    // And finally we process all the commands which require authentication
    match command {
        Commands::Login(_) => unreachable!(),
        Commands::Register { .. } => unreachable!(),
        Commands::Item { command: _ } => todo!(),
        Commands::Sync {} => todo!(),
        Commands::Generate { command } => match command {
            GeneratorCommands::Password(args) => {
                let password = client.generator().password(PasswordGeneratorRequest {
                    lowercase: args.lowercase,
                    uppercase: args.uppercase,
                    numbers: args.numbers,
                    special: args.special,
                    length: args.length,
                    ..Default::default()
                })?;

                println!("{}", password);
            }
            GeneratorCommands::Passphrase(args) => {
                let passphrase = client.generator().passphrase(PassphraseGeneratorRequest {
                    num_words: args.words,
                    word_separator: args.separator.to_string(),
                    capitalize: args.capitalize,
                    include_number: args.include_number,
                })?;

                println!("{}", passphrase);
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
