use std::process;

use bitwarden::{
    secrets_manager::secrets::{
        SecretCreateRequest, SecretGetRequest, SecretIdentifiersByProjectRequest,
        SecretIdentifiersRequest, SecretPutRequest, SecretsDeleteRequest, SecretsGetRequest,
    },
    Client,
};
use bitwarden_cli::Color;
use color_eyre::eyre::{bail, Result};
use uuid::Uuid;

use crate::{cli::Output, render::serialize_response};

pub(crate) async fn list(
    mut client: Client,
    organization_id: Uuid,
    project_id: Option<Uuid>,
    output: Output,
    color: Color,
) -> Result<()> {
    let res = if let Some(project_id) = project_id {
        client
            .secrets()
            .list_by_project(&SecretIdentifiersByProjectRequest { project_id })
            .await?
    } else {
        client
            .secrets()
            .list(&SecretIdentifiersRequest { organization_id })
            .await?
    };

    let secret_ids = res.data.into_iter().map(|e| e.id).collect();
    let secrets = client
        .secrets()
        .get_by_ids(SecretsGetRequest { ids: secret_ids })
        .await?
        .data;
    serialize_response(secrets, output, color);

    Ok(())
}

pub(crate) async fn get(
    mut client: Client,
    secret_id: Uuid,
    output: Output,
    color: Color,
) -> Result<()> {
    let secret = client
        .secrets()
        .get(&SecretGetRequest { id: secret_id })
        .await?;
    serialize_response(secret, output, color);

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn create(
    mut client: Client,
    organization_id: Uuid,
    project_id: Uuid,
    key: String,
    value: String,
    note: Option<String>,
    output: Output,
    color: Color,
) -> Result<()> {
    let secret = client
        .secrets()
        .create(&SecretCreateRequest {
            organization_id,
            key,
            value,
            note: note.unwrap_or_default(),
            project_ids: Some(vec![project_id]),
        })
        .await?;
    serialize_response(secret, output, color);

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn edit(
    mut client: Client,
    organization_id: Uuid,
    project_id: Option<Uuid>,
    secret_id: Uuid,
    key: Option<String>,
    value: Option<String>,
    note: Option<String>,
    output: Output,
    color: Color,
) -> Result<()> {
    let old_secret = client
        .secrets()
        .get(&SecretGetRequest { id: secret_id })
        .await?;

    let secret = client
        .secrets()
        .update(&SecretPutRequest {
            id: secret_id,
            organization_id,
            key: key.unwrap_or(old_secret.key),
            value: value.unwrap_or(old_secret.value),
            note: note.unwrap_or(old_secret.note),
            project_ids: match project_id {
                Some(id) => Some(vec![id]),
                None => match old_secret.project_id {
                    Some(id) => Some(vec![id]),
                    None => bail!("Editing a secret requires a project_id."),
                },
            },
        })
        .await?;
    serialize_response(secret, output, color);

    Ok(())
}

#[allow(clippy::comparison_chain)]
pub(crate) async fn delete(mut client: Client, secret_ids: Vec<Uuid>) -> Result<()> {
    let count = secret_ids.len();

    let result = client
        .secrets()
        .delete(SecretsDeleteRequest { ids: secret_ids })
        .await?;

    let secrets_failed: Vec<(Uuid, String)> = result
        .data
        .into_iter()
        .filter_map(|r| r.error.map(|e| (r.id, e)))
        .collect();
    let deleted_secrets = count - secrets_failed.len();

    if deleted_secrets > 1 {
        println!("{} secrets deleted successfully.", deleted_secrets);
    } else if deleted_secrets == 1 {
        println!("{} secret deleted successfully.", deleted_secrets);
    }

    if secrets_failed.len() > 1 {
        eprintln!("{} secrets had errors:", secrets_failed.len());
    } else if secrets_failed.len() == 1 {
        eprintln!("{} secret had an error:", secrets_failed.len());
    }

    for secret in &secrets_failed {
        eprintln!("{}: {}", secret.0, secret.1);
    }

    if !secrets_failed.is_empty() {
        process::exit(1);
    }

    Ok(())
}
