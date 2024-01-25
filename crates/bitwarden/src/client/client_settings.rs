use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Basic client behavior settings. These settings specify the various targets and behavior of the
/// Bitwarden Client. They are optional and uneditable once the client is initialized.
///
/// Defaults to
///
/// ```
/// # use bitwarden::client::client_settings::{ClientSettings, DeviceType};
/// let settings = ClientSettings {
///     identity_url: "https://identity.bitwarden.com".to_string(),
///     api_url: "https://api.bitwarden.com".to_string(),
///     user_agent: "Bitwarden Rust-SDK".to_string(),
///     device_type: DeviceType::SDK,
/// };
/// let default = ClientSettings::default();
/// ```
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct ClientSettings {
    /// The identity url of the targeted Bitwarden instance. Defaults to `https://identity.bitwarden.com`
    pub identity_url: String,
    /// The api url of the targeted Bitwarden instance. Defaults to `https://api.bitwarden.com`
    pub api_url: String,
    /// The user_agent to sent to Bitwarden. Defaults to `Bitwarden Rust-SDK`
    pub user_agent: String,
    /// Device type to send to Bitwarden. Defaults to SDK
    pub device_type: DeviceType,
}

impl Default for ClientSettings {
    fn default() -> Self {
        Self {
            identity_url: "https://identity.bitwarden.com".into(),
            api_url: "https://api.bitwarden.com".into(),
            user_agent: "Bitwarden Rust-SDK".into(),
            device_type: DeviceType::SDK,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, JsonSchema)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum DeviceType {
    Android = 0,
    iOS = 1,
    ChromeExtension = 2,
    FirefoxExtension = 3,
    OperaExtension = 4,
    EdgeExtension = 5,
    WindowsDesktop = 6,
    MacOsDesktop = 7,
    LinuxDesktop = 8,
    ChromeBrowser = 9,
    FirefoxBrowser = 10,
    OperaBrowser = 11,
    EdgeBrowser = 12,
    IEBrowser = 13,
    UnknownBrowser = 14,
    AndroidAmazon = 15,
    UWP = 16,
    SafariBrowser = 17,
    VivaldiBrowser = 18,
    VivaldiExtension = 19,
    SafariExtension = 20,

    SDK = 21,
}
