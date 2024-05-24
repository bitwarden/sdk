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
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientProjects<'a> {
    pub async fn get(&mut self, input: &ProjectGetRequest) -> Result<ProjectResponse> {
        get_project(self.client, input).await
    }

    pub async fn create(&mut self, input: &ProjectCreateRequest) -> Result<ProjectResponse> {
        create_project(self.client, input).await
    }

    pub async fn list(&mut self, input: &ProjectsListRequest) -> Result<ProjectsResponse> {
        list_projects(self.client, input).await
    }

    pub async fn update(&mut self, input: &ProjectPutRequest) -> Result<ProjectResponse> {
        update_project(self.client, input).await
    }

    pub async fn delete(&mut self, input: ProjectsDeleteRequest) -> Result<ProjectsDeleteResponse> {
        delete_projects(self.client, input).await
    }
}

impl<'a> Client {
    pub fn projects(&'a mut self) -> ClientProjects<'a> {
        ClientProjects { client: self }
    }
}
