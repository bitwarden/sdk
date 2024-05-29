use bitwarden::{secrets_manager::projects::ProjectsListRequest, Client};
use bitwarden_cli::Color;
use color_eyre::eyre::Result;
use uuid::Uuid;

use crate::{cli::Output, render::serialize_response};

pub(crate) async fn list(
    mut client: Client,
    organization_id: Uuid,
    output: Output,
    color: Color,
) -> Result<()> {
    let projects = client
        .projects()
        .list(&ProjectsListRequest { organization_id })
        .await?
        .data;

    serialize_response(projects, output, color);

    Ok(())
}
