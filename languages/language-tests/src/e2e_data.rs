use std::{fs::File, io::BufReader};

use anyhow::{Context, Result};
use bitwarden::secrets_manager::projects::ProjectResponse;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
struct E2EData {
    projects: Vec<TestProjectData>,
    secrets: Vec<TestSecretData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TestProjectData {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TestSecretData {
    pub key: String,
    pub value: String,
    pub note: String,
    pub project_name: String,
}

#[derive(Debug, Clone)]
pub struct RealizedTestSecretData {
    pub key: String,
    pub value: String,
    pub note: String,
    pub project_id: Uuid,
}

impl TestSecretData {
    pub fn project_id(&self, projects: &[ProjectResponse]) -> Result<Uuid> {
        let id = projects
            .iter()
            .find(|p| p.name == self.project_name).context(format!("Project, {}, not found", self.project_name))?.id;
        Ok(id)
    }

    pub fn realize(&self, projects: &[ProjectResponse]) -> Result<RealizedTestSecretData> {
        Ok(RealizedTestSecretData {
            key: self.key.clone(),
            value: self.value.clone(),
            note: self.note.clone(),
            project_id: self.project_id(projects)?,
        })
    }
}

pub fn load_projects(run_id: &str) -> Result<Vec<TestProjectData>> {
    Ok(load_data()?.projects.iter().map(|project| project.with_run_id(run_id)).collect())
}

pub fn load_secrets(run_id: &str) -> Result<Vec<TestSecretData>> {
    Ok(load_data()?.secrets.iter().map(|secret| secret.with_run_id(run_id)).collect())
}

pub fn load_realized_secrets(run_id: &str, loaded_projects: &[ProjectResponse]) -> Result<Vec<RealizedTestSecretData>> {
    load_secrets(run_id)?.iter().map(|secret| secret.realize(loaded_projects)).collect()
}

fn load_data() -> Result<E2EData> {
    // read e2e data from file
    let file = File::open("e2e_data.json").context("Failed to open e2e data file")?;
    let reader = BufReader::new(file);

    let data: E2EData = serde_json::from_reader(reader).context("Failed to parse e2e data")?;
    Ok(data)
}

trait RunIdNotation {
    fn with_run_id(&self, run_id: &str) -> Self;
}

impl RunIdNotation for String {
    fn with_run_id(&self, run_id: &str) -> Self {
        format!("{}-{}", self, run_id)
    }
}

impl RunIdNotation for TestProjectData {
    fn with_run_id(&self, run_id: &str) -> Self {
        Self {
            name: self.name.with_run_id(run_id)
        }
    }
}

impl RunIdNotation for TestSecretData {
    fn with_run_id(&self, run_id: &str) -> Self {
        Self {
            key: self.key.with_run_id(run_id),
            value: self.value.clone(),
            note: self.note.clone(),
            project_name: self.project_name.with_run_id(run_id),
        }
    }
}
