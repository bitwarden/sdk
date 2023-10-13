use log::{debug, info};
use schemars::JsonSchema;

use crate::{
    crypto::Decryptable,
    error::Result,
    platform::SyncRequest,
    vault::{download_attachment, Cipher, CipherView, Collection, Folder},
    Client,
};

mod client_exporter;

#[derive(JsonSchema)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum ExportFormat {
    Csv,
    Json,
    AccountEncryptedJson, // TODO: Should we deprecate this option completely?
    EncryptedJson { password: String },
}

pub(super) fn export_vault(
    _folders: Vec<Folder>,
    _ciphers: Vec<Cipher>,
    _format: ExportFormat,
) -> Result<String> {
    todo!();
}

pub(super) fn export_organization_vault(
    _collections: Vec<Collection>,
    _ciphers: Vec<Cipher>,
    _format: ExportFormat,
) -> Result<String> {
    todo!();
}

pub(super) async fn export_vault_attachments(client: &mut Client) -> Result<()> {
    info!("Syncing vault");
    let sync = client
        .sync(&SyncRequest {
            exclude_subdomains: None,
        })
        .await?;

    debug!("{:#?}", sync);

    info!("Vault synced got {} ciphers", sync.ciphers.len());

    let ciphers_with_attachments = sync.ciphers.iter().filter(|c| !c.attachments.is_empty());

    info!(
        "Found {} ciphers with attachments",
        ciphers_with_attachments.count()
    );

    info!("Decrypting ciphers");

    let enc_settings = client.get_encryption_settings()?;

    let decrypted: Vec<CipherView> = sync
        .ciphers
        .iter()
        .map(|c| c.decrypt(enc_settings, &None).unwrap())
        .collect();

    let num_attachments = decrypted.iter().flat_map(|c| &c.attachments).count();

    info!("Found {} attachments, starting export", num_attachments);

    for cipher in decrypted {
        for attachment in cipher.attachments {
            download_attachment(client, cipher.id.unwrap(), &attachment.id.unwrap()).await?;
        }
    }

    Ok(())
}
