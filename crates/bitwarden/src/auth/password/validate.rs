use bitwarden_crypto::HashPurpose;

use crate::{
    auth::determine_password_hash,
    client::{LoginMethod, UserLoginMethod},
    error::{Error, Result},
    Client,
};

/// Validate if the provided password matches the password hash stored in the client.
pub(crate) async fn validate_password(
    client: &Client,
    password: String,
    password_hash: String,
) -> Result<bool> {
    let login_method = client
        .login_method
        .as_ref()
        .ok_or(Error::NotAuthenticated)?;

    if let LoginMethod::User(login_method) = login_method {
        match login_method {
            UserLoginMethod::Username { email, kdf, .. }
            | UserLoginMethod::ApiKey { email, kdf, .. } => {
                let hash =
                    determine_password_hash(email, kdf, &password, HashPurpose::LocalAuthorization)
                        .await?;

                Ok(hash == password_hash)
            }
        }
    } else {
        Err(Error::NotAuthenticated)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_validate_password() {
        use std::num::NonZeroU32;

        use super::validate_password;
        use crate::client::{kdf::Kdf, Client, LoginMethod, UserLoginMethod};

        let mut client = Client::new(None);
        client.set_login_method(LoginMethod::User(UserLoginMethod::Username {
            email: "test@bitwarden.com".to_string(),
            kdf: Kdf::PBKDF2 {
                iterations: NonZeroU32::new(100_000).unwrap(),
            },
            client_id: "1".to_string(),
        }));

        let password = "password123".to_string();
        let password_hash = "7kTqkF1pY/3JeOu73N9kR99fDDe9O1JOZaVc7KH3lsU=".to_string();

        let result = validate_password(&client, password, password_hash).await;

        assert!(result.unwrap());
    }
}
