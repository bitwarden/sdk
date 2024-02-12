use base64::{engine::general_purpose::STANDARD, Engine};
use bitwarden_crypto::fingerprint;
use log::info;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct FingerprintRequest {
    /// The input material, used in the fingerprint generation process.
    pub fingerprint_material: String,
    /// The user's public key encoded with base64.
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FingerprintResponse {
    pub fingerprint: String,
}

pub(crate) fn generate_fingerprint(input: &FingerprintRequest) -> Result<FingerprintResponse> {
    info!("Generating fingerprint");

    let key = STANDARD.decode(&input.public_key)?;

    Ok(FingerprintResponse {
        fingerprint: fingerprint(&input.fingerprint_material, &key)?,
    })
}

pub(crate) fn generate_user_fingerprint(
    client: &mut crate::Client,
    fingerprint_material: String,
) -> Result<String> {
    info!("Generating fingerprint");

    let enc_settings = client.get_encryption_settings()?;
    let private_key = enc_settings
        .private_key
        .as_ref()
        .ok_or("Missing private key")?;

    let public_key = private_key.to_public_der()?;
    let fingerprint = fingerprint(&fingerprint_material, &public_key)?;

    Ok(fingerprint)
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::*;
    use crate::{
        client::{Kdf, LoginMethod, UserLoginMethod},
        Client,
    };

    #[test]
    fn test_generate_user_fingerprint() {
        let user_key = "2.oZg5RYpU2HjUAKI1DUQCkg==|PyRzI9kZpt66P2OedH8CHOeU0/lgKLkhIJiKDijdyFqIemBSIBoslhfQh/P1TK9xgZp0smgD6+5+yNbZfOpBaCVrsT3WWAO78xOWizduRe4=|xfDLDZSJ+yZAdh388flVg7SMDBJuMs0+CHTjutKs4uQ=";
        let private_key = "2.tY6WsWKUbBwNU8wROuipiQ==|DNFL1d19xVojUKTTy2gxT+9J1VXbMQLcbMnx1HSeA6U3yZhsLR6DPaGibb3Bp8doIHtrsxzL/JeLb4gLDZ8RnDhFfE4iLRaPakX14kbBXrKH9/uW/zc7TqIVciWhI1PaeFlu8wnVuGt3e5Ysx6Y7Uw7RS8pRT5aE3sX3aDPGZTAdTutLn1VUfkShS5OK5HJl9CdiwV2wOcrf4w/WqtaNUUqGdsJ8C4ELlpBzHxqs+lEm+8pGPYmuGQIjVc0eOR9Tza9GTk3ih1XGc1znOCoKUZbtA29RfbwfmJy/yGi/3RLWZFQGCCij4cLC5OpldiX4JWL5Dhox44p/5IVF3rfxTVz3GCyDOoHevRG/06sUBq6nhbdCQf3lJvxwcQJhoQg4rsapM3rgol+u+TbXRiwWPbfswuLkRlvGFKtKUWMa4S57gj0CFYgSBPdTyhZTB44D7JQ2bd901Ur1dYWcDe4Kn3ZawpxL0cX2ZPlE3v8FXFJf2s8DJytL8yu73GasDzVmaGHxueWWVz7EHjh+pmB4oaAHARcY8d3LActAyl/+bcFRPYQJ68ae6DJhYYJGHIBWMImf2BifGgUX8vUFfUAYjne3D82lRyZQHs3xbl+ZxEPgWiPYRWUtxGXLLP4f9mbl+LeJdehtHNjC8kOduBL0CsP4gmugzNNUXI+Izc/9svno6kFr6SU0LA3MGrOU8ao7UCQbf/Pj/RKnG1gRmBDQqf7IMm6jOyTwdde9NpfQb32iH11PkuAKBvEtUuq9BeAKWjoZku+ycsN2jZH0hzd/QrU2c+E4+yHwX3wSxxorNOXt5EZkJbEDBlpRyE1zWoyy0wIYfcChYLvFN8QFHchlw5wmHxL+OOgdgndAtV/2DCx+NB6caY31qLictME+1GPPlQ7QvicMLgmpSWq83rs4ex/My6p3hCRSrJJiLvjEDZLYWKHHLd5tsPRAjX8ADNWB1VeIeiJrj1wpOCc1PbWpbljbbTsBmVPo6iKm/UDGAHBdQ//0j3FQg8f5w/j+McsoaMpDNHNTiLvjWERR+RBmsEA0lEL00wZz/DHlzOAYHLYYqFMT7GBCQD+Wk/l1TL+X2agUy7Irlk7QbZ4ivfdNIpSW8Ct9MGE6o4wV+nIpXURojgBBTcP85RTBLXXGrIprnK1G/VE8ONag3+nkqIyChjYyk5QMsxqOqSHsbiOxhCdXypbCbY4g9yKJtBJ/ADjxmELj0X7pqsTFqC0eRT7rk9qTBcYBBu6rwlAfq8AKjDB7WjNjzLaMi6lBoe4petBn1xcLkXD5hHra0TULxcYrq8MIb+Vk4CBZZdwwyVm/28SwSjHBIBpRysPAonDDsp3KlahwXEFvRDQR/oFww172GI7cx8SoPn93Qh0JfpTAAowsO3meR8bzUSyd7v3rmtaBPsWHE9zUXye/6nloMU5joEcD6uJaxd0kdaWWIoKLH++zHW1R776wJrS6u+TIWZgHqiIJoCd9fV25BnQcbZRKd6mnfNQkchJ6c6ozXKrFaa8DLdERdfh84+isw5mzW2zMJwHEwtKt6LUTyieC2exzPAwPxJT1+IMjuzuwiLnvGKOq+kwE/LWBSB0ZfGuCP/3jMM8OCfe7Hbpt1TfXcUxUzj6sSjkjQB6qBt+TINRdOFA=|fppguME86utsAOKrBYn6XU95q7daVbZ+3dD9OVkQlAw=";
        let fingerprint_material = "a09726a0-9590-49d1-a5f5-afe300b6a515";

        let mut client = Client::new(None);
        client.set_login_method(LoginMethod::User(UserLoginMethod::Username {
            client_id: "a09726a0-9590-49d1-a5f5-afe300b6a515".to_owned(),
            email: "robb@stark.com".to_owned(),
            kdf: Kdf::PBKDF2 {
                iterations: NonZeroU32::new(600_000).unwrap(),
            },
        }));
        client
            .initialize_user_crypto(
                "asdfasdfasdf",
                user_key.parse().unwrap(),
                private_key.parse().unwrap(),
            )
            .unwrap();

        let fingerprint =
            generate_user_fingerprint(&mut client, fingerprint_material.to_string()).unwrap();

        assert_eq!(fingerprint, "turban-deftly-anime-chatroom-unselfish");
    }
}
