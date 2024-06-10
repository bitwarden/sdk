use crate::require;
use bitwarden_api_api::{
    apis::auth_requests_api::{auth_requests_id_response_get, auth_requests_post},
    models::{AuthRequestCreateRequestModel, AuthRequestType},
};
use bitwarden_crypto::Kdf;
use uuid::Uuid;

use crate::{
    auth::{
        api::{request::AuthRequestTokenRequest, response::IdentityTokenResponse},
        auth_request::new_auth_request,
    },
    client::{LoginMethod, UserLoginMethod},
    error::Result,
    mobile::crypto::{AuthRequestMethod, InitUserCryptoMethod, InitUserCryptoRequest},
    Client,
};

pub struct NewAuthRequestResponse {
    pub fingerprint: String,
    email: String,
    device_identifier: String,
    auth_request_id: Uuid,
    access_code: String,
    private_key: String,
}

pub(crate) async fn send_new_auth_request(
    client: &mut Client,
    email: String,
    device_identifier: String,
) -> Result<NewAuthRequestResponse> {
    let config = client.internal.get_api_configurations().await;

    let auth = new_auth_request(&email)?;

    let req = AuthRequestCreateRequestModel {
        email: email.clone(),
        public_key: auth.public_key,
        device_identifier: device_identifier.clone(),
        access_code: auth.access_code.clone(),
        r#type: AuthRequestType::AuthenticateAndUnlock,
    };

    let res = auth_requests_post(&config.api, Some(req)).await?;

    Ok(NewAuthRequestResponse {
        fingerprint: auth.fingerprint,
        email,
        device_identifier,
        auth_request_id: require!(res.id),
        access_code: auth.access_code,
        private_key: auth.private_key,
    })
}

pub(crate) async fn complete_auth_request(
    client: &mut Client,
    auth_req: NewAuthRequestResponse,
) -> Result<()> {
    let config = client.internal.get_api_configurations().await;

    let res = auth_requests_id_response_get(
        &config.api,
        auth_req.auth_request_id,
        Some(&auth_req.access_code),
    )
    .await?;

    let approved = res.request_approved.unwrap_or(false);

    if !approved {
        return Err("Auth request was not approved".into());
    }

    let response = AuthRequestTokenRequest::new(
        &auth_req.email,
        &auth_req.auth_request_id,
        &auth_req.access_code,
        config.device_type,
        &auth_req.device_identifier,
    )
    .send(config)
    .await?;

    if let IdentityTokenResponse::Authenticated(r) = response {
        let kdf = Kdf::default();

        client.internal.set_tokens(
            r.access_token.clone(),
            r.refresh_token.clone(),
            r.expires_in,
        );
        client
            .internal
            .set_login_method(LoginMethod::User(UserLoginMethod::Username {
                client_id: "web".to_owned(),
                email: auth_req.email.to_owned(),
                kdf: kdf.clone(),
            }));

        let method = match res.master_password_hash {
            Some(_) => AuthRequestMethod::MasterKey {
                protected_master_key: require!(res.key).parse()?,
                auth_request_key: require!(r.key).parse()?,
            },
            None => AuthRequestMethod::UserKey {
                protected_user_key: require!(res.key).parse()?,
            },
        };

        client
            .crypto()
            .initialize_user_crypto(InitUserCryptoRequest {
                kdf_params: kdf,
                email: auth_req.email,
                private_key: require!(r.private_key),
                method: InitUserCryptoMethod::AuthRequest {
                    request_private_key: auth_req.private_key,
                    method,
                },
            })
            .await?;

        Ok(())
    } else {
        Err("Failed to authenticate".into())
    }
}
