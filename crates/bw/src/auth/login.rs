use bitwarden::{
    auth::login::{
        ApiKeyLoginRequest, PasswordLoginRequest, TwoFactorEmailRequest, TwoFactorProvider,
        TwoFactorRequest,
    },
    platform::SyncRequest,
    Client,
};
use bitwarden_cli::text_prompt_when_none;
use color_eyre::eyre::{bail, Result};
use inquire::{Password, Text};
use log::{debug, error, info};

pub(crate) async fn login_password(client: &mut Client, email: Option<String>) -> Result<()> {
    let email = text_prompt_when_none("Email", email)?;

    let password = Password::new("Password").without_confirmation().prompt()?;

    let kdf = client.auth().prelogin(email.clone()).await?;

    let result = client
        .auth()
        .login_password(&PasswordLoginRequest {
            email: email.clone(),
            password: password.clone(),
            two_factor: None,
            kdf: kdf.clone(),
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
                .auth()
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
        };

        let result = client
            .auth()
            .login_password(&PasswordLoginRequest {
                email,
                password,
                two_factor,
                kdf,
            })
            .await?;

        debug!("{:?}", result);
    } else {
        debug!("{:?}", result);
    }

    let res = client
        .sync(&SyncRequest {
            exclude_subdomains: Some(true),
        })
        .await?;
    info!("{:#?}", res);

    Ok(())
}

pub(crate) async fn login_api_key(
    mut client: Client,
    client_id: Option<String>,
    client_secret: Option<String>,
) -> Result<()> {
    let client_id = text_prompt_when_none("Client ID", client_id)?;
    let client_secret = text_prompt_when_none("Client Secret", client_secret)?;

    let password = Password::new("Password").without_confirmation().prompt()?;

    let result = client
        .auth()
        .login_api_key(&ApiKeyLoginRequest {
            client_id,
            client_secret,
            password,
        })
        .await?;

    debug!("{:?}", result);

    Ok(())
}

pub(crate) async fn login_device(
    mut client: Client,
    email: Option<String>,
    device_identifier: Option<String>,
) -> Result<()> {
    let email = text_prompt_when_none("Email", email)?;
    let device_identifier = text_prompt_when_none("Device Identifier", device_identifier)?;

    let auth = client
        .auth()
        .login_device(email, device_identifier)
        .await
        .unwrap();

    println!("Fingerprint: {}", auth.fingerprint);

    Text::new("Press enter once approved").prompt()?;

    client.auth().login_device_complete(auth).await.unwrap();

    Ok(())
}
