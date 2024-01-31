use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::api::response::IdentityTokenResponse,
    client::{client_settings::DeviceType, ApiConfigurations},
    error::Result,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequestTokenRequest {
    scope: String,
    client_id: String,
    #[serde(rename = "deviceType")]
    device_type: u8,
    #[serde(rename = "deviceIdentifier")]
    device_identifier: String,
    #[serde(rename = "deviceName")]
    device_name: String,
    grant_type: String,
    #[serde(rename = "username")]
    email: String,
    #[serde(rename = "authRequest")]
    auth_request_id: Uuid,
    #[serde(rename = "password")]
    access_code: String,
}

impl AuthRequestTokenRequest {
    pub fn new(
        email: &str,
        auth_request_id: &Uuid,
        access_code: &str,
        device_type: DeviceType,
        device_identifier: &str,
    ) -> Self {
        let obj = Self {
            scope: "api offline_access".to_string(),
            client_id: "web".to_string(),
            device_type: device_type as u8,
            device_identifier: device_identifier.to_string(),
            device_name: "chrome".to_string(),
            grant_type: "password".to_string(),
            email: email.to_string(),
            auth_request_id: *auth_request_id,
            access_code: access_code.to_string(),
        };
        debug!("initializing {:?}", obj);
        obj
    }

    pub(crate) async fn send(
        &self,
        configurations: &ApiConfigurations,
    ) -> Result<IdentityTokenResponse> {
        super::send_identity_connect_request(configurations, Some(&self.email), &self).await
    }
}
