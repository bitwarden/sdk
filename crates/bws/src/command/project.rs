use std::process;

use bitwarden::{
    secrets_manager::projects::{
        ProjectCreateRequest, ProjectGetRequest, ProjectPutRequest, ProjectsDeleteRequest,
        ProjectsListRequest,
    },
    Client,
};
use color_eyre::eyre::Result;
use uuid::Uuid;

use super::OutputSettings;
use crate::{render::serialize_response, ProjectCommand};

pub(crate) async fn process_command(
    command: ProjectCommand,
    client: Client,
    organization_id: Uuid,
    output_settings: OutputSettings,
) -> Result<()> {
    match command {
        ProjectCommand::List => list(client, organization_id, output_settings).await,
        ProjectCommand::Get { project_id } => get(client, project_id, output_settings).await,
        ProjectCommand::Create { name } => {
            create(client, organization_id, name, output_settings).await
        }
        ProjectCommand::Edit { project_id, name } => {
            edit(client, organization_id, project_id, name, output_settings).await
        }
        ProjectCommand::Delete { project_ids } => delete(client, project_ids).await,
    }
}

pub(crate) async fn list(
    mut client: Client,
    organization_id: Uuid,
    output_settings: OutputSettings,
) -> Result<()> {
    let projects = client
        .projects()
        .list(&ProjectsListRequest { organization_id })
        .await?
        .data;
    serialize_response(projects, output_settings.output, output_settings.color);

    Ok(())
}

pub(crate) async fn get(
    mut client: Client,
    project_id: Uuid,
    output_settings: OutputSettings,
) -> Result<()> {
    let project = client
        .projects()
        .get(&ProjectGetRequest { id: project_id })
        .await?;
    serialize_response(project, output_settings.output, output_settings.color);

    Ok(())
}

pub(crate) async fn create(
    mut client: Client,
    organization_id: Uuid,
    name: String,
    output_settings: OutputSettings,
) -> Result<()> {
    let project = client
        .projects()
        .create(&ProjectCreateRequest {
            organization_id,
            name,
        })
        .await?;
    serialize_response(project, output_settings.output, output_settings.color);

    Ok(())
}

pub(crate) async fn edit(
    mut client: Client,
    organization_id: Uuid,
    project_id: Uuid,
    name: String,
    output_settings: OutputSettings,
) -> Result<()> {
    let project = client
        .projects()
        .update(&ProjectPutRequest {
            id: project_id,
            organization_id,
            name,
        })
        .await?;
    serialize_response(project, output_settings.output, output_settings.color);

    Ok(())
}

pub(crate) async fn delete(mut client: Client, project_ids: Vec<Uuid>) -> Result<()> {
    let count = project_ids.len();

    let result = client
        .projects()
        .delete(ProjectsDeleteRequest { ids: project_ids })
        .await?;

    let projects_failed: Vec<(Uuid, String)> = result
        .data
        .into_iter()
        .filter_map(|r| r.error.map(|e| (r.id, e)))
        .collect();
    let deleted_projects = count - projects_failed.len();

    match deleted_projects {
        2.. => println!("{} projects deleted successfully.", deleted_projects),
        1 => println!("{} project deleted successfully.", deleted_projects),
        _ => (),
    }

    match projects_failed.len() {
        2.. => eprintln!("{} projects had errors:", projects_failed.len()),
        1 => eprintln!("{} project had an error:", projects_failed.len()),
        _ => (),
    }

    for project in &projects_failed {
        eprintln!("{}: {}", project.0, project.1);
    }

    if !projects_failed.is_empty() {
        process::exit(1);
    }

    Ok(())
}
