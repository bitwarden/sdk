use bitwarden_json::client::Client as JsonClient;
use pyo3::prelude::*;

#[pyclass]
pub struct BitwardenClient(JsonClient);

#[pymethods]
impl BitwardenClient {
    #[new]
    #[pyo3(signature = (settings_string=None))]
    pub fn new(settings_string: Option<String>) -> Self {
        // This will only fail if another logger was already initialized, so we can ignore the
        // result
        let _ = pyo3_log::try_init();

        Self(JsonClient::new(settings_string))
    }

    #[pyo3(text_signature = "($self, command_input)")]
    fn run_command(&self, command_input: String) -> String {
        run_command(&self.0, &command_input)
    }
}

#[tokio::main]
async fn run_command(client: &JsonClient, input_str: &str) -> String {
    client.run_command(input_str).await
}
