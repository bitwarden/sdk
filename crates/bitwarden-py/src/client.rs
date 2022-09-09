use bitwarden_json::client::Client as JsonClient;
use pyo3::prelude::*;

#[pyclass]
pub struct BitwardenClient(JsonClient);

#[pymethods]
impl BitwardenClient {
    #[new]
    pub fn new(settings_string: Option<String>) -> Self {
        pyo3_log::init();
        Self(JsonClient::new(settings_string))
    }

    #[pyo3(text_signature = "($self, command_input)")]
    fn run_command(&mut self, command_input: String) -> String {
        run_command(&mut self.0, &command_input)
    }
}

#[tokio::main]
async fn run_command(client: &mut JsonClient, input_str: &str) -> String {
    client.run_command(input_str).await
}
