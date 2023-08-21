use std::str::FromStr;

use base64::Engine;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{
        api::{request::AccessTokenRequest, response::IdentityTokenResponse},
        login::ApiKeyLoginResponse,
    },
    client::{
        encryption_settings::{decrypt, SymmetricCryptoKey},
        AccessToken, LoginMethod,
    },
    crypto::CipherString,
    error::{Error, Result},
    util::{decode_token, BASE64_ENGINE},
    Client,
};

pub(crate) async fn access_token_login(
    client: &mut Client,
    input: &AccessTokenLoginRequest,
) -> Result<ApiKeyLoginResponse> {
    //info!("api key logging in");
    //debug!("{:#?}, {:#?}", client, input);

    let access_token = AccessToken::from_str(&input.access_token)?;

    let response = request_access_token(client, &access_token).await?;

    if let IdentityTokenResponse::Payload(r) = &response {
        // Extract the encrypted payload and use the access token encryption key to decrypt it
        let payload = CipherString::from_str(&r.encrypted_payload)?;

        let decrypted_payload = decrypt(&payload, &access_token.encryption_key)?;

        // Once decrypted, we have to JSON decode to extract the organization encryption key
        #[derive(serde::Deserialize)]
        struct Payload {
            #[serde(rename = "encryptionKey")]
            encryption_key: String,
        }

        let payload: Payload = serde_json::from_slice(&decrypted_payload)?;

        let encryption_key = BASE64_ENGINE.decode(payload.encryption_key)?;

        let encryption_key = SymmetricCryptoKey::try_from(encryption_key.as_slice())?;

        let access_token_obj = decode_token(&r.access_token)?;

        // This should always be Some() when logging in with an access token
        let organization_id = access_token_obj
            .organization
            .ok_or(Error::MissingFields)?
            .parse()
            .map_err(|_| Error::InvalidResponse)?;

        client.set_tokens(
            r.access_token.clone(),
            r.refresh_token.clone(),
            r.expires_in,
            LoginMethod::AccessToken {
                service_account_id: access_token.service_account_id,
                client_secret: access_token.client_secret,
                organization_id,
            },
        );

        client.initialize_crypto_single_key(encryption_key);
    }

    ApiKeyLoginResponse::process_response(response)
}

async fn request_access_token(
    client: &mut Client,
    input: &AccessToken,
) -> Result<IdentityTokenResponse> {
    let config = client.get_api_configurations().await;
    AccessTokenRequest::new(input.service_account_id, &input.client_secret)
        .send(config)
        .await
}

/// Login to Bitwarden with access token
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccessTokenLoginRequest {
    /// Bitwarden service API access token
    pub access_token: String,
}
