#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fido2ClientCreateCredentialRequest {
    /// WebAuthn-compatible JSON string of the PublicKeyCredentialRequestOptions
    pub webauthn_json: String,
}
