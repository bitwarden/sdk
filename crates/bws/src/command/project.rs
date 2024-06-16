use bitwarden::{
    secrets_manager::projects::{
        ProjectCreateRequest, ProjectGetRequest, ProjectPutRequest, ProjectsDeleteRequest,
        ProjectsListRequest,
    },
    Client,
};
use color_eyre::eyre::{eyre, Result};
use uuid::Uuid;

use crate::render::{serialize_response, OutputSettings};

pub(crate) async fn list(
    client: Client,
    organization_id: Uuid,
    output_settings: OutputSettings,
) -> Result<()> {
    let projects = client
        .projects()
        .list(&ProjectsListRequest { organization_id })
        .await?
        .data;
    serialize_response(projects, output_settings);

    Ok(())
}

pub(crate) async fn get(
    client: Client,
    project_id: Uuid,
    output_settings: OutputSettings,
) -> Result<()> {
    let project = client
        .projects()
        .get(&ProjectGetRequest { id: project_id })
        .await?;
    serialize_response(project, output_settings);

    Ok(())
}

pub(crate) async fn create(
    client: Client,
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
    serialize_response(project, output_settings);

    Ok(())
}

pub(crate) async fn edit(
    client: Client,
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
    serialize_response(project, output_settings);

    Ok(())
}

pub(crate) async fn delete(client: Client, project_ids: Vec<Uuid>) -> Result<()> {
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
        return Err(eyre!("Errors when attempting to delete projects."));
    }

    Ok(())
}
