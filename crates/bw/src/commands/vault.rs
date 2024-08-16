use bitwarden::{
    vault::{CipherListView, ClientVaultExt},
    Client, Error,
};
use bitwarden_cli::Color;
use clap::Subcommand;

use crate::render::{serialize_response, Output, OutputSettings, TableSerialize};

#[derive(Subcommand, Clone)]
pub(crate) enum VaultCommands {
    Get { id: String },
    List {},
    Create {},
}

pub(crate) async fn process_command(
    command: VaultCommands,
    client: Client,
    password: Option<String>,
) -> Result<(), Error> {
    client.auth().unlock(password.unwrap()).await.unwrap();
    match command {
        VaultCommands::Get { id: _ } => todo!(),
        VaultCommands::List {} => {
            let ciphers = client.vault().cipher_repository.get_all().await.unwrap();

            let dec = client.vault().ciphers().decrypt_list(ciphers)?;

            /*for cipher in dec {
                println!("{}", cipher.name);
            }*/

            let output_settings = OutputSettings::new(Output::Table, Color::Auto);
            serialize_response(dec, output_settings);

            Ok(())
        }
        VaultCommands::Create {} => todo!(),
    }
}

impl TableSerialize<2> for CipherListView {
    fn get_headers() -> [&'static str; 2] {
        ["ID", "Name"]
    }

    fn get_values(&self) -> Vec<[String; 2]> {
        vec![[self.id.unwrap_or_default().to_string(), self.name.clone()]]
    }
}
