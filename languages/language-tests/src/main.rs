use anyhow::{Context, Result};
use bitwarden::{
    auth::login::ApiKeyLoginRequest,
    secrets_manager::{
        projects::{
            ProjectCreateRequest, ProjectResponse, ProjectsDeleteRequest, ProjectsListRequest,
        },
        secrets::{
            self, SecretCreateRequest, SecretIdentifiersRequest, SecretResponse,
            SecretsDeleteRequest,
        },
        ClientProjectsExt, ClientSecretsExt,
    },
    vault::{ClientVaultExt, SyncRequest},
    Client,
};
use e2e_data::{load_projects, load_realized_secrets, load_secrets};
use std::{env, sync::Arc};
use uuid::Uuid;

mod e2e_data;

struct RunData {
    run_id: String,
    organization_id: Uuid,
    client: Arc<Client>,
    projects_created: Vec<ProjectResponse>,
    secrets_created: Vec<SecretResponse>,
}

#[tokio::main]
async fn main() {
    // Get the pipeline owner credentials from runtime args
    let run_id = env::var("RUN_ID").expect("RUN_ID not set");

    // Determine if we are setting up or tearing down test data
    let action = env::args()
        .nth(1)
        .expect("action not set, please provide 'setup' or 'teardown'");

    let organization_id =
        Uuid::parse_str(&env::var("ORGANIZATION_ID").expect("ORGANIZATION_ID not set"))
            .expect("Invalid organization ID");

    let client = build_client().await.expect("Failed to build client");

    let mut run_data = RunData {
        run_id: run_id.clone(),
        organization_id,
        client: Arc::new(client),
        projects_created: Vec::new(),
        secrets_created: Vec::new(),
    };

    match action.as_str() {
        "setup" => match set_up(&mut run_data).await {
            Ok(_) => println!("Test data set up successfully"),
            Err(e) => {
                eprintln!("Failed to set up test data: {:?}", e);
                clean_up(&run_data)
                    .await
                    .expect("Failed to clean up test data");
            }
        },
        "teardown" => clean_up(&run_data)
            .await
            .expect("Failed to clean up test data"),
        _ => eprintln!("Invalid action: {}, please specify either 'setup' or 'teardown'", action),
    }
}

async fn build_client() -> Result<Client> {
    // Read necessary env vars
    let client_id = env::var("OWNER_CLIENT_ID").context("OWNER_CLIENT_ID not set")?;
    let client_secret = env::var("OWNER_CLIENT_SECRET").context("OWNER_CLIENT_SECRET not set")?;
    let password = env::var("OWNER_PASSWORD").context("OWNER_PASSWORD not set")?;

    // initialize client
    let client = Client::new(Some(bitwarden::ClientSettings {
        api_url: "https://vault.qa.bitwarden.pw/api".to_owned(),
        identity_url: "https://vault.qa.bitwarden.pw/identity".to_owned(),
        ..Default::default()
    }));

    // Authenticate as pipeline owner
    let auth_response = client
        .auth()
        .login_api_key(&ApiKeyLoginRequest {
            client_id,
            client_secret,
            password,
        })
        .await
        .context("Pipeline owner authentication failure")?;
    assert!(auth_response.authenticated);
    // Need to sync vault for now to grab organization keys
    client
        .vault()
        .sync(&SyncRequest {
            exclude_subdomains: Some(true),
        })
        .await
        .context("Failed to sync vault")?;

    Ok(client)
}

async fn set_up(run_data: &mut RunData) -> Result<()> {
    // set up projects
    let projects = load_projects(&run_data.run_id)?;
    let tasks: Vec<_> = projects
        .iter()
        .cloned()
        .map(|project| {
            let org_id = run_data.organization_id.clone();
            let client = run_data.client.clone();
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
        .collect();
    for result in futures::future::join_all(tasks).await {
        let project = result.context("Failed to join create project task")??;
        run_data.projects_created.push(project);
    }
    println!("Projects created: {:?}", run_data.projects_created.iter().map(|p| p.id).collect::<Vec<_>>());

    // Set up secrets
    let secrets = load_realized_secrets(&run_data.run_id, &run_data.projects_created)?;
    // create secrets
    let tasks: Vec<_> = secrets
        .iter()
        .cloned()
        .map(|secret| {
            let org_id = run_data.organization_id;
            let client = run_data.client.clone();
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
        .collect();
    for result in futures::future::join_all(tasks).await {
        let secret = result.context("Failed to join create secret task")??;
        run_data.secrets_created.push(secret);
    }
    println!("Secrets created: {:?}", run_data.secrets_created.iter().map(|s| s.id).collect::<Vec<_>>());

    Ok(())
}

async fn clean_up(run_data: &RunData) -> Result<()> {
    let to_delete = match run_data.secrets_created.is_empty() {
        // Clean up by Id
        false => run_data.secrets_created.iter().map(|s| s.id).collect(),
        // Clean up by name
        true => {
            let test_secrets = load_secrets(&run_data.run_id)?;
            let server_secrets = run_data
                .client
                .secrets()
                .list(&SecretIdentifiersRequest {
                    organization_id: run_data.organization_id,
                })
                .await
                .context("Failed to list secrets")?;
            server_secrets
                .data
                .iter()
                .filter(|s| {
                    test_secrets
                        .iter()
                        .any(|ts| ts.key == s.key)
                })
                .map(|s| s.id)
                .collect()
        }
    };

    println!("Deleting secrets: {:?}", to_delete);

    let secret_clean_up = run_data
        .client
        .secrets()
        .delete(SecretsDeleteRequest { ids: to_delete })
        .await
        .context("Failed to delete secrets");

    let to_delete: Vec<_> = match run_data.projects_created.is_empty() {
        // Clean up by id
        false => run_data.projects_created.iter().map(|p| p.id).collect(),
        // Clean up by name
        true => {
            let test_projects = load_projects(&run_data.run_id)?;
            let server_projects = run_data
                .client
                .projects()
                .list(&ProjectsListRequest {
                    organization_id: run_data.organization_id,
                })
                .await
                .context("Failed to list projects")?;
            server_projects
                .data
                .iter()
                .filter(|p| {
                    test_projects
                        .iter()
                        .any(|tp| tp.name == p.name)
                })
                .map(|p| p.id)
                .collect()
        }
    };

    println!("Deleting projects: {:?}", to_delete);

    let project_clean_up = run_data
        .client
        .projects()
        .delete(ProjectsDeleteRequest { ids: to_delete })
        .await
        .context("Failed to delete projects");

    secret_clean_up.and(project_clean_up)?;
    Ok(())
}
