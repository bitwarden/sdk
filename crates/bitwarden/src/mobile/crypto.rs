use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    client::kdf::Kdf,
    crypto::EncString,
    error::{Error, Result},
    Client,
};

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct InitUserCryptoRequest {
    /// The user's KDF parameters, as received from the prelogin request
    pub kdf_params: Kdf,
    /// The user's email address
    pub email: String,
    /// The user's encrypted private key
    pub private_key: String,
    /// The initialization method to use
    pub method: InitUserCryptoMethod,
}

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum InitUserCryptoMethod {
    Password {
        /// The user's master password
        password: String,
        /// The user's encrypted symmetric crypto key
        user_key: String,
    },
    DecryptedKey {
        /// The user's decrypted encryption key, obtained using `get_user_encryption_key`
        decrypted_user_key: String,
    },
    Pin {
        /// The user's PIN
        pin: String,
        /// The user's symmetric crypto key, encrypted with the PIN. Use `derive_pin_key` to obtain this.
        pin_protected_user_key: EncString,
    },
}

#[cfg(feature = "internal")]
pub async fn initialize_user_crypto(client: &mut Client, req: InitUserCryptoRequest) -> Result<()> {
    use crate::crypto::SymmetricCryptoKey;

    let login_method = crate::client::LoginMethod::User(crate::client::UserLoginMethod::Username {
        client_id: "".to_string(),
        email: req.email,
        kdf: req.kdf_params,
    });
    client.set_login_method(login_method);

    let private_key: EncString = req.private_key.parse()?;

    match req.method {
        InitUserCryptoMethod::Password { password, user_key } => {
            let user_key: EncString = user_key.parse()?;
            client.initialize_user_crypto(&password, user_key, private_key)?;
        }
        InitUserCryptoMethod::DecryptedKey { decrypted_user_key } => {
            let user_key = decrypted_user_key.parse::<SymmetricCryptoKey>()?;
            client.initialize_user_crypto_decrypted_key(user_key, private_key)?;
        }
        InitUserCryptoMethod::Pin {
            pin,
            pin_protected_user_key,
        } => {
            client.initialize_user_crypto_pin(&pin, pin_protected_user_key, private_key)?;
        }
    }

    Ok(())
}

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct InitOrgCryptoRequest {
    /// The encryption keys for all the organizations the user is a part of
    pub organization_keys: HashMap<uuid::Uuid, EncString>,
}

#[cfg(feature = "internal")]
pub async fn initialize_org_crypto(client: &mut Client, req: InitOrgCryptoRequest) -> Result<()> {
    let organization_keys = req.organization_keys.into_iter().collect();
    client.initialize_org_crypto(organization_keys)?;
    Ok(())
}

#[cfg(feature = "internal")]
pub async fn get_user_encryption_key(client: &mut Client) -> Result<String> {
    let user_key = client
        .get_encryption_settings()?
        .get_key(&None)
        .ok_or(Error::VaultLocked)?;

    Ok(user_key.to_base64())
}

#[cfg(feature = "internal")]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct DerivePinKeyResponse {
    pin_protected_user_key: EncString,
    encrypted_pin: EncString,
}

#[cfg(feature = "internal")]
pub fn derive_pin_key(client: &mut Client, pin: String) -> Result<DerivePinKeyResponse> {
    use crate::{
        client::{LoginMethod, UserLoginMethod},
        crypto::{KeyEncryptable, MasterKey},
    };

    let derived_key = match &client.login_method {
        Some(LoginMethod::User(
            UserLoginMethod::Username { email, kdf, .. }
            | UserLoginMethod::ApiKey { email, kdf, .. },
        )) => MasterKey::derive(pin.as_bytes(), email.as_bytes(), kdf)?,
        _ => return Err(Error::NotAuthenticated),
    };

    let user_key = client
        .get_encryption_settings()?
        .get_key(&None)
        .ok_or(Error::VaultLocked)?;

    Ok(DerivePinKeyResponse {
        pin_protected_user_key: derived_key.encrypt_user_key(user_key)?,
        encrypted_pin: pin.encrypt_with_key(user_key)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::kdf::Kdf, Client};

    #[tokio::test]
    async fn test_initialize_user_crypto_pin() {
        let mut client = Client::new(None);

        let priv_key = "2.kmLY8NJVuiKBFJtNd/ZFpA==|qOodlRXER+9ogCe3yOibRHmUcSNvjSKhdDuztLlucs10jLiNoVVVAc+9KfNErLSpx5wmUF1hBOJM8zwVPjgQTrmnNf/wuDpwiaCxNYb/0v4FygPy7ccAHK94xP1lfqq7U9+tv+/yiZSwgcT+xF0wFpoxQeNdNRFzPTuD9o4134n8bzacD9DV/WjcrXfRjbBCzzuUGj1e78+A7BWN7/5IWLz87KWk8G7O/W4+8PtEzlwkru6Wd1xO19GYU18oArCWCNoegSmcGn7w7NDEXlwD403oY8Oa7ylnbqGE28PVJx+HLPNIdSC6YKXeIOMnVs7Mctd/wXC93zGxAWD6ooTCzHSPVV50zKJmWIG2cVVUS7j35H3rGDtUHLI+ASXMEux9REZB8CdVOZMzp2wYeiOpggebJy6MKOZqPT1R3X0fqF2dHtRFPXrNsVr1Qt6bS9qTyO4ag1/BCvXF3P1uJEsI812BFAne3cYHy5bIOxuozPfipJrTb5WH35bxhElqwT3y/o/6JWOGg3HLDun31YmiZ2HScAsUAcEkA4hhoTNnqy4O2s3yVbCcR7jF7NLsbQc0MDTbnjxTdI4VnqUIn8s2c9hIJy/j80pmO9Bjxp+LQ9a2hUkfHgFhgHxZUVaeGVth8zG2kkgGdrp5VHhxMVFfvB26Ka6q6qE/UcS2lONSv+4T8niVRJz57qwctj8MNOkA3PTEfe/DP/LKMefke31YfT0xogHsLhDkx+mS8FCc01HReTjKLktk/Jh9mXwC5oKwueWWwlxI935ecn+3I2kAuOfMsgPLkoEBlwgiREC1pM7VVX1x8WmzIQVQTHd4iwnX96QewYckGRfNYWz/zwvWnjWlfcg8kRSe+68EHOGeRtC5r27fWLqRc0HNcjwpgHkI/b6czerCe8+07TWql4keJxJxhBYj3iOH7r9ZS8ck51XnOb8tGL1isimAJXodYGzakwktqHAD7MZhS+P02O+6jrg7d+yPC2ZCuS/3TOplYOCHQIhnZtR87PXTUwr83zfOwAwCyv6KP84JUQ45+DItrXLap7nOVZKQ5QxYIlbThAO6eima6Zu5XHfqGPMNWv0bLf5+vAjIa5np5DJrSwz9no/hj6CUh0iyI+SJq4RGI60lKtypMvF6MR3nHLEHOycRUQbZIyTHWl4QQLdHzuwN9lv10ouTEvNr6sFflAX2yb6w3hlCo7oBytH3rJekjb3IIOzBpeTPIejxzVlh0N9OT5MZdh4sNKYHUoWJ8mnfjdM+L4j5Q2Kgk/XiGDgEebkUxiEOQUdVpePF5uSCE+TPav/9FIRGXGiFn6NJMaU7aBsDTFBLloffFLYDpd8/bTwoSvifkj7buwLYM+h/qcnfdy5FWau1cKav+Blq/ZC0qBpo658RTC8ZtseAFDgXoQZuksM10hpP9bzD04Bx30xTGX81QbaSTNwSEEVrOtIhbDrj9OI43KH4O6zLzK+t30QxAv5zjk10RZ4+5SAdYndIlld9Y62opCfPDzRy3ubdve4ZEchpIKWTQvIxq3T5ogOhGaWBVYnkMtM2GVqvWV//46gET5SH/MdcwhACUcZ9kCpMnWH9CyyUwYvTT3UlNyV+DlS27LMPvaw7tx7qa+GfNCoCBd8S4esZpQYK/WReiS8=|pc7qpD42wxyXemdNPuwxbh8iIaryrBPu8f/DGwYdHTw=";

        initialize_user_crypto(
            &mut client,
            InitUserCryptoRequest {
                kdf_params: Kdf::PBKDF2 {
                    iterations: 100_000.try_into().unwrap(),
                },
                email: "test@bitwarden.com".into(),
                private_key: priv_key.to_owned(),
                method: InitUserCryptoMethod::Password {
                    password: "asdfasdfasdf".into(),
                    user_key: "2.u2HDQ/nH2J7f5tYHctZx6Q==|NnUKODz8TPycWJA5svexe1wJIz2VexvLbZh2RDfhj5VI3wP8ZkR0Vicvdv7oJRyLI1GyaZDBCf9CTBunRTYUk39DbZl42Rb+Xmzds02EQhc=|rwuo5wgqvTJf3rgwOUfabUyzqhguMYb3sGBjOYqjevc=".into(),
                },
            },
        )
        .await
        .unwrap();

        let pin_key = derive_pin_key(&mut client, "1234".into()).unwrap();

        let mut client = Client::new(None);

        initialize_user_crypto(
            &mut client,
            InitUserCryptoRequest {
                kdf_params: Kdf::PBKDF2 {
                    iterations: 100_000.try_into().unwrap(),
                },
                email: "test@bitwarden.com".into(),
                private_key: priv_key.to_owned(),
                method: InitUserCryptoMethod::Pin {
                    pin: "1234".into(),
                    pin_protected_user_key: pin_key.pin_protected_user_key,
                },
            },
        )
        .await
        .unwrap();
    }
}
