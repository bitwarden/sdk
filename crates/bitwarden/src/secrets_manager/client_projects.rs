use crate::{
    error::Result,
    secrets_manager::projects::{
        create_project, delete_projects, get_project, list_projects, update_project,
        ProjectCreateRequest, ProjectGetRequest, ProjectPutRequest, ProjectResponse,
        ProjectsDeleteRequest, ProjectsDeleteResponse, ProjectsListRequest, ProjectsResponse,
    },
    Client,
};

pub struct ClientProjects<'a> {
    pub(crate) client: &'a crate::Client,
}

impl<'a> ClientProjects<'a> {
    pub async fn get(&self, input: &ProjectGetRequest) -> Result<ProjectResponse> {
        get_project(self.client, input).await
    }

    pub async fn create(&self, input: &ProjectCreateRequest) -> Result<ProjectResponse> {
        create_project(self.client, input).await
    }

    pub async fn list(&self, input: &ProjectsListRequest) -> Result<ProjectsResponse> {
        list_projects(self.client, input).await
    }

    pub async fn update(&self, input: &ProjectPutRequest) -> Result<ProjectResponse> {
        update_project(self.client, input).await
    }

    pub async fn delete(&self, input: ProjectsDeleteRequest) -> Result<ProjectsDeleteResponse> {
        delete_projects(self.client, input).await
    }
}

impl<'a> Client {
    pub fn projects(&'a self) -> ClientProjects<'a> {
        ClientProjects { client: self }
    }
}
