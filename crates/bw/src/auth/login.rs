use bitwarden::{
    auth::request::{
        ApiKeyLoginRequest, PasswordLoginRequest, TwoFactorEmailRequest, TwoFactorProvider,
        TwoFactorRequest,
    },
    Client,
};
use color_eyre::eyre::Result;
use inquire::{Password, Text};
use log::error;

pub(crate) async fn password_login(mut client: Client, email: Option<String>) -> Result<()> {
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
        error!("{:?}", two_factor);

        let two_factor = if let Some(tf) = two_factor.authenticator {
            error!("{:?}", tf);

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

            error!("Two factor code sent to {:?}", tf);
            let token = Text::new("Two factor code").prompt()?;

            Some(TwoFactorRequest {
                token,
                provider: TwoFactorProvider::Email,
                remember: false,
            })
        } else {
            error!("Not supported: {:?}", two_factor);

            None
        };

        let result = client
            .password_login(&PasswordLoginRequest {
                email: email,
                password: password,
                two_factor: two_factor,
            })
            .await?;

        error!("{:?}", result);
    } else {
        error!("{:?}", result);
    }

    Ok(())
}

pub(crate) async fn api_key_login(
    mut client: Client,
    client_id: Option<String>,
    client_secret: Option<String>,
) -> Result<()> {
    let client_id = text_prompt(client_id, "Client ID")?;
    let client_secret = text_prompt(client_secret, "Client Secret")?;

    let password = Password::new("Password").without_confirmation().prompt()?;

    let result = client
        .api_key_login(&ApiKeyLoginRequest {
            client_id,
            client_secret,
            password,
        })
        .await?;

    error!("{:?}", result);

    Ok(())
}

fn text_prompt(val: Option<String>, prompt: &str) -> Result<String> {
    Ok(if let Some(val) = val {
        val
    } else {
        Text::new(prompt).prompt()?
    })
}
