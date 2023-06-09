use bitwarden::sdk::{
    auth::request::{
        PasswordLoginRequest, TwoFactorEmailRequest, TwoFactorProvider, TwoFactorRequest,
    },
    request::client_settings::ClientSettings,
};
use clap::{command, CommandFactory, Parser, Subcommand};
use color_eyre::eyre::{bail, Result};
use inquire::{Password, Text};
use log::error;
use render::{Color, Output};
use std::path::PathBuf;

mod config;
mod render;

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
    Login {
        #[arg(short = 'e', long, help = "Email address")]
        email: Option<String>,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    process_commands().await
}

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

    let profile = get_config_profile(&cli.server_url, &cli.profile, &cli.config_file)?;

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

    // And finally we process all the commands which require authentication
    match command {
        // FIXME: Rust CLI will not support password login!
        Commands::Login { email } => {
            let email = if let Some(email) = email {
                email
            } else {
                Text::new("Email").prompt()?
            };

            let password = Password::new("Password").without_confirmation().prompt()?;

            let result = client
                .password_login(&PasswordLoginRequest {
                    email: email.clone(),
                    password: password.clone(),
                    two_factor: None,
                })
                .await?;

            if let Some(_) = result.captcha {
                // TODO: We should build a web captcha solution
                error!("Captcha required");
            } else if let Some(two_factor) = result.two_factor {
                if let Some(tf) = two_factor.email {
                    // Send token
                    client
                        .send_two_factor_email(&TwoFactorEmailRequest {
                            email: email.clone(),
                            password: password.clone(),
                        })
                        .await?;

                    error!("Two factor code sent to {:?}", tf);
                    let token = Text::new("Two factor code").prompt()?;

                    let result = client
                        .password_login(&PasswordLoginRequest {
                            email: email,
                            password: password,
                            two_factor: Some(TwoFactorRequest {
                                token: token,
                                provider: TwoFactorProvider::Email,
                                remember: false,
                            }),
                        })
                        .await?;

                    error!("{:?}", result);
                } else {
                    error!("Not supported: {:?}", two_factor);
                }
            } else {
                error!("{:?}", result);
            }
        }
    }

    Ok(())
}

fn get_config_profile(
    server_url: &Option<String>,
    profile: &Option<String>,
    config_file: &Option<PathBuf>,
) -> Result<Option<config::Profile>, color_eyre::Report> {
    let profile = if let Some(server_url) = server_url {
        Some(config::Profile::from_url(server_url)?)
    } else {
        let profile_defined = profile.is_some();

        let profile_key = if let Some(profile) = profile {
            profile.to_owned()
        } else {
            "".to_string()
        };

        let config = config::load_config(config_file.as_deref(), config_file.is_some())?;
        config.select_profile(&profile_key, profile_defined)?
    };
    Ok(profile)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
