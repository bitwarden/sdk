use std::{env, sync::Arc};

use anyhow::{bail, Context, Result};
use bitwarden::{
    auth::login::AccessTokenLoginRequest,
    secrets_manager::{
        projects::{
            ProjectCreateRequest, ProjectResponse, ProjectsDeleteRequest, ProjectsListRequest,
        },
        secrets::{
            SecretCreateRequest, SecretIdentifiersRequest, SecretResponse, SecretsDeleteRequest,
        },
        ClientProjectsExt, ClientSecretsExt,
    },
    Client,
};
use e2e_data::{
    load_projects, load_realized_secrets, load_secrets, DataKind, RealizedTestSecretData,
    TestProjectData,
};
use tokio::task::JoinHandle;
use uuid::Uuid;

mod e2e_data;

struct RunData {
    run_id: String,
    organization_id: Uuid,
    client: Arc<Client>,
    mutable_client: Arc<Client>,
    projects_created: Vec<ProjectResponse>,
    secrets_created: Vec<SecretResponse>,
}

const STATE_FILE_IMMUTABLE: &str = "state_immutable.json";
const STATE_FILE_MUTABLE: &str = "state_mutable.json";

#[tokio::main]
async fn main() -> Result<()> {
    // Get the pipeline owner credentials from runtime args
    let run_id = env::var("RUN_ID").expect("RUN_ID not set");

    // Determine if we are setting up or tearing down test data
    let action = env::args()
        .nth(1)
        .expect("action not set, please provide 'setup' or 'teardown'");

    let organization_id =
        Uuid::parse_str(&env::var("ORGANIZATION_ID").expect("ORGANIZATION_ID not set"))
            .expect("Invalid organization ID");

    let (client, mutable_client) = build_clients().await.expect("Failed to build client");

    let mut run_data = RunData {
        run_id: run_id.clone(),
        organization_id,
        client: Arc::new(client),
        mutable_client: Arc::new(mutable_client),
        projects_created: Vec::new(),
        secrets_created: Vec::new(),
    };

    let result = match action.as_str() {
        "setup" => match set_up(&mut run_data).await {
            Ok(_) => {
                println!("Test data set up successfully");
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to set up test data: {:?}", e);
                clean_up(&run_data).await
            }
        },
        "teardown" => clean_up(&run_data).await,
        _ => {
            bail!(
                "Invalid action: {}, please specify either 'setup' or 'teardown'",
                action
            )
        }
    };

    if let Err(e) = result {
        eprintln!("Failed to clean up test data: {:?}", e);
    };

    // clean up state file
    std::fs::remove_file(STATE_FILE_IMMUTABLE).context("Failed to delete immutable state file")?;
    std::fs::remove_file(STATE_FILE_MUTABLE).context("Failed to delete mutable state file")?;

    Ok(())
}

async fn build_clients() -> Result<(Client, Client)> {
    // Read env vars
    let access_token = env::var("ACCESS_TOKEN").context("ACCESS_TOKEN not set")?;
    let mutable_access_token =
        env::var("MUTABLE_ACCESS_TOKEN").context("MUTABLE_ACCESS_TOKEN not set")?;

    // initialize clients
    let settings = bitwarden::ClientSettings {
        api_url: "https://vault.qa.bitwarden.pw/api".to_owned(),
        identity_url: "https://vault.qa.bitwarden.pw/identity".to_owned(),
        ..Default::default()
    };
    let client = Client::new(Some(settings.clone()));
    let mutable_client = Client::new(Some(settings));

    let auth_response = client
        .auth()
        .login_access_token(&AccessTokenLoginRequest {
            access_token,
            state_file: Some(STATE_FILE_IMMUTABLE.into()),
        })
        .await
        .context("Failed to authenticate with access token")?;
    assert!(auth_response.authenticated);

    let mutable_auth_response = mutable_client
        .auth()
        .login_access_token(&AccessTokenLoginRequest {
            access_token: mutable_access_token,
            state_file: Some(STATE_FILE_MUTABLE.into()),
        })
        .await
        .context("Failed to authenticate with mutable access token")?;
    assert!(mutable_auth_response.authenticated);

    Ok((client, mutable_client))
}

fn write_projects(
    client: &Arc<Client>,
    run_data: &RunData,
    projects: Vec<TestProjectData>,
) -> Vec<JoinHandle<Result<ProjectResponse, anyhow::Error>>> {
    projects
        .iter()
        .cloned()
        .map(|project| {
            let org_id = run_data.organization_id;
            let client = client.clone();
            tokio::spawn(async move {
                client
                    .projects()
                    .create(&ProjectCreateRequest {
                        name: project.name.to_owned(),
                        organization_id: org_id,
                    })
                    .await
                    .context("Failed to create project")
            })
        })
        .collect()
}

fn write_secrets(
    client: &Arc<Client>,
    run_data: &RunData,
    secrets: Vec<RealizedTestSecretData>,
) -> Vec<JoinHandle<Result<SecretResponse, anyhow::Error>>> {
    secrets
        .iter()
        .cloned()
        .map(|secret| {
            let org_id = run_data.organization_id;
            let client = client.clone();
            tokio::spawn(async move {
                client
                    .secrets()
                    .create(&SecretCreateRequest {
                        organization_id: org_id,
                        key: secret.key.to_owned(),
                        value: secret.value.to_owned(),
                        note: secret.note.to_owned(),
                        project_ids: Some(vec![secret.project_id]),
                    })
                    .await
                    .context("Failed to create secret")
            })
        })
        .collect()
}

async fn set_up(run_data: &mut RunData) -> Result<()> {
    // set up projects
    let projects = load_projects(&run_data.run_id, DataKind::Immutable)?;
    let mutable_projects = load_projects(&run_data.run_id, DataKind::Mutable)?;
    let tasks: Vec<_> = write_projects(&run_data.client, run_data, projects)
        .into_iter()
        .chain(write_projects(
            &run_data.mutable_client,
            run_data,
            mutable_projects,
        ))
        .collect();
    for result in futures::future::join_all(tasks).await {
        let project = result.context("Failed to join create project task")??;
        run_data.projects_created.push(project);
    }
    println!(
        "Projects created: {:?}",
        run_data
            .projects_created
            .iter()
            .map(|p| p.id)
            .collect::<Vec<_>>()
    );

    // Set up secrets
    let secrets = load_realized_secrets(
        &run_data.run_id,
        &run_data.projects_created,
        DataKind::Immutable,
    )?;
    let mutable_secrets = load_realized_secrets(
        &run_data.run_id,
        &run_data.projects_created,
        DataKind::Mutable,
    )?;
    let tasks: Vec<_> = write_secrets(&run_data.client, run_data, secrets)
        .into_iter()
        .chain(write_secrets(
            &run_data.mutable_client,
            run_data,
            mutable_secrets,
        ))
        .collect();
    for result in futures::future::join_all(tasks).await {
        let secret = result.context("Failed to join create secret task")??;
        run_data.secrets_created.push(secret);
    }
    println!(
        "Secrets created: {:?}",
        run_data
            .secrets_created
            .iter()
            .map(|s| s.id)
            .collect::<Vec<_>>()
    );

    Ok(())
}

async fn clean_up(run_data: &RunData) -> Result<()> {
    clean_up_data(run_data, DataKind::Immutable).await?;
    clean_up_data(run_data, DataKind::Mutable).await?;

    Ok(())
}

async fn clean_up_data(run_data: &RunData, data_kind: DataKind) -> Result<()> {
    let client = match data_kind {
        DataKind::Immutable => &run_data.client,
        DataKind::Mutable => &run_data.mutable_client,
    };

    let secrets: Vec<_> = client
        .secrets()
        .list(&SecretIdentifiersRequest {
            organization_id: run_data.organization_id,
        })
        .await
        .context("Failed to list secrets")?
        .data
        .iter()
        .filter(|s| s.key.ends_with(&run_data.run_id))
        .map(|s| s.id)
        .collect();
    let projects: Vec<_> = client
        .projects()
        .list(&ProjectsListRequest {
            organization_id: run_data.organization_id,
        })
        .await
        .context("Failed to list projects")?
        .data
        .iter()
        .filter(|p| p.name.ends_with(&run_data.run_id))
        .map(|p| p.id)
        .collect();

    println!("Deleting secrets: {:?}", secrets);
    client
        .secrets()
        .delete(SecretsDeleteRequest { ids: secrets })
        .await
        .context("Failed to delete secrets")?;
    println!("Deleting projects: {:?}", projects);
    client
        .projects()
        .delete(ProjectsDeleteRequest { ids: projects })
        .await
        .context("Failed to delete projects")?;

    Ok(())
}
