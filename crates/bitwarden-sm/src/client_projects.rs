use bitwarden_core::{Client, Error};

use crate::projects::{
    create_project, delete_projects, get_project, list_projects, update_project,
    ProjectCreateRequest, ProjectGetRequest, ProjectPutRequest, ProjectResponse,
    ProjectsDeleteRequest, ProjectsDeleteResponse, ProjectsListRequest, ProjectsResponse,
};

pub struct ClientProjects<'a> {
    pub client: &'a Client,
}

impl<'a> ClientProjects<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn get(&self, input: &ProjectGetRequest) -> Result<ProjectResponse, Error> {
        get_project(self.client, input).await
    }

    pub async fn create(&self, input: &ProjectCreateRequest) -> Result<ProjectResponse, Error> {
        create_project(self.client, input).await
    }

    pub async fn list(&self, input: &ProjectsListRequest) -> Result<ProjectsResponse, Error> {
        list_projects(self.client, input).await
    }

    pub async fn update(&self, input: &ProjectPutRequest) -> Result<ProjectResponse, Error> {
        update_project(self.client, input).await
    }

    pub async fn delete(
        &self,
        input: ProjectsDeleteRequest,
    ) -> Result<ProjectsDeleteResponse, Error> {
        delete_projects(self.client, input).await
    }
}

pub trait ClientProjectsExt<'a> {
    fn projects(&'a self) -> ClientProjects<'a>;
}

impl<'a> ClientProjectsExt<'a> for Client {
    fn projects(&'a self) -> ClientProjects<'a> {
        ClientProjects::new(self)
    }
}
