use reqwest::Url;

use super::{
    get_string_name_from_enum,
    types::{
        AuthenticatorAssertionResponse, AuthenticatorAttestationResponse, ClientData,
        ClientExtensionResults, CredPropsResult,
    },
    Fido2Authenticator, PublicKeyCredentialAuthenticatorAssertionResponse,
    PublicKeyCredentialAuthenticatorAttestationResponse,
};
use crate::error::Result;

pub struct Fido2Client<'a> {
    pub(crate) authenticator: Fido2Authenticator<'a>,
}

impl<'a> Fido2Client<'a> {
    pub async fn register(
        &mut self,
        origin: String,
        request: String,
        client_data: ClientData,
    ) -> Result<PublicKeyCredentialAuthenticatorAttestationResponse> {
        let origin = Url::parse(&origin).map_err(|e| format!("Invalid origin: {}", e))?;

        let request: passkey::types::webauthn::CredentialCreationOptions =
            serde_json::from_str(&request)?;

        // Insert the received UV to be able to return it later in check_user
        let uv = request
            .public_key
            .authenticator_selection
            .as_ref()
            .map(|s| s.user_verification.into());
        *self
            .authenticator
            .requested_uv
            .get_mut()
            .expect("Mutex is not poisoned") = uv;

        let rp_id = request.public_key.rp.id.clone();

        let mut client = passkey::client::Client::new(self.authenticator.get_authenticator(true));
        let result = client.register(&origin, request, client_data).await?;

        Ok(PublicKeyCredentialAuthenticatorAttestationResponse {
            id: result.id,
            raw_id: result.raw_id.into(),
            ty: get_string_name_from_enum(result.ty)?,
            authenticator_attachment: result
                .authenticator_attachment
                .map(get_string_name_from_enum)
                .transpose()?,
            client_extension_results: ClientExtensionResults {
                cred_props: result.client_extension_results.cred_props.map(Into::into),
            },
            response: AuthenticatorAttestationResponse {
                client_data_json: result.response.client_data_json.into(),
                authenticator_data: result.response.authenticator_data.into(),
                public_key: result.response.public_key.map(|x| x.into()),
                public_key_algorithm: result.response.public_key_algorithm,
                attestation_object: result.response.attestation_object.into(),
                transports: if rp_id.unwrap_or_default() == "https://google.com" {
                    Some(vec!["internal".to_string(), "usb".to_string()])
                } else {
                    Some(vec!["internal".to_string()])
                },
            },
            selected_credential: self.authenticator.get_selected_credential()?,
        })
    }

    pub async fn authenticate(
        &mut self,
        origin: String,
        request: String,
        client_data: ClientData,
    ) -> Result<PublicKeyCredentialAuthenticatorAssertionResponse> {
        let origin = Url::parse(&origin).map_err(|e| format!("Invalid origin: {}", e))?;

        let request: passkey::types::webauthn::CredentialRequestOptions =
            serde_json::from_str(&request)?;

        // Insert the received UV to be able to return it later in check_user
        let uv = request.public_key.user_verification.into();
        self.authenticator
            .requested_uv
            .get_mut()
            .expect("Mutex is not poisoned")
            .replace(uv);

        let mut client = passkey::client::Client::new(self.authenticator.get_authenticator(false));
        let result = client.authenticate(&origin, request, client_data).await?;

        Ok(PublicKeyCredentialAuthenticatorAssertionResponse {
            id: result.id,
            raw_id: result.raw_id.into(),
            ty: get_string_name_from_enum(result.ty)?,

            authenticator_attachment: result
                .authenticator_attachment
                .map(get_string_name_from_enum)
                .transpose()?,
            client_extension_results: ClientExtensionResults {
                cred_props: result
                    .client_extension_results
                    .cred_props
                    .map(|c| CredPropsResult {
                        rk: c.discoverable,
                        authenticator_display_name: c.authenticator_display_name,
                    }),
            },
            response: AuthenticatorAssertionResponse {
                client_data_json: result.response.client_data_json.into(),
                authenticator_data: result.response.authenticator_data.into(),
                signature: result.response.signature.into(),
                user_handle: result.response.user_handle.unwrap_or_default().into(),
            },
            selected_credential: self.authenticator.get_selected_credential()?,
        })
    }
}
