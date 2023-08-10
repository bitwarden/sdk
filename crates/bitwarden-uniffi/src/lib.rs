use bitwarden::client::client_settings::ClientSettings;

uniffi::include_scaffolding!("sdk");

pub struct Client(bitwarden::Client);

impl Client {
    pub fn new(_settings_input: String) -> Self {
        let settings = Self::parse_settings(None);
        Self(bitwarden::Client::new(settings))
    }

    pub fn run_command(&self, input_str: String) -> String {
        input_str
    }

    fn parse_settings(_settings_input: Option<String>) -> Option<ClientSettings> {
        None
    }
}
