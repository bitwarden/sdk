use bitwarden::{
    auth::login::{
        ApiKeyLoginRequest, PasswordLoginRequest, TwoFactorEmailRequest, TwoFactorProvider,
        TwoFactorRequest,
    },
    Client,
};
use color_eyre::eyre::Result;
use inquire::{Password, Text};
use log::error;

pub(crate) async fn password_login(mut client: Client, email: Option<String>) -> Result<()> {
    let email = text_prompt_when_none("Email", email)?;

    let password = Password::new("Password").without_confirmation().prompt()?;

    let result = client
        .password_login(&PasswordLoginRequest {
            email: email.clone(),
            password: password.clone(),
            two_factor: None,
        })
        .await?;

    if result.captcha.is_some() {
        // TODO: We should build a web captcha solution
        error!("Captcha required");
    } else if let Some(two_factor) = result.two_factor {
        error!("{:?}", two_factor);

        let two_factor = if let Some(tf) = two_factor.authenticator {
            debug!("{:?}", tf);

            let token = Text::new("Authenticator code").prompt()?;

            Some(TwoFactorRequest {
                token,
                provider: TwoFactorProvider::Authenticator,
                remember: false,
            })
        } else if let Some(tf) = two_factor.email {
            // Send token
            client
                .send_two_factor_email(&TwoFactorEmailRequest {
                    email: email.clone(),
                    password: password.clone(),
                })
                .await?;

            info!("Two factor code sent to {:?}", tf);
            let token = Text::new("Two factor code").prompt()?;

            Some(TwoFactorRequest {
                token,
                provider: TwoFactorProvider::Email,
                remember: false,
            })
        } else {
            bail!("Not supported: {:?}", two_factor);

            None
        };

        let result = client
            .password_login(&PasswordLoginRequest {
                email,
                password,
                two_factor,
            })
            .await?;

        debug!("{:?}", result);
    } else {
        debug!("{:?}", result);
    }

    Ok(())
}

pub(crate) async fn api_key_login(
    mut client: Client,
    client_id: Option<String>,
    client_secret: Option<String>,
) -> Result<()> {
    let client_id = text_prompt_when_none("Client ID", client_id)?;
    let client_secret = text_prompt_when_none("Client Secret", client_secret)?;

    let password = Password::new("Password").without_confirmation().prompt()?;

    let result = client
        .api_key_login(&ApiKeyLoginRequest {
            client_id,
            client_secret,
            password,
        })
        .await?;

    debug!("{:?}", result);

    Ok(())
}

/// Prompt the user for input if the value is None
///
/// Typically used when the user can provide a value via CLI or prompt
fn text_prompt_when_none(prompt: &str, val: Option<String>) -> Result<String> {
    Ok(if let Some(val) = val {
        val
    } else {
        Text::new(prompt).prompt()?
    })
}
