use crate::{
    commands::{get_project, list_projects},
    error::Result,
    sdk::{
        request::projects_request::{ProjectGetRequest, ProjectsListRequest},
        response::projects_response::{ProjectResponse, ProjectsResponse},
    },
};

pub struct ClientProjects<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientProjects<'a> {
    pub async fn get(&mut self, input: &ProjectGetRequest) -> Result<ProjectResponse> {
        get_project(self.client, input).await
    }

    pub async fn list(&mut self, input: &ProjectsListRequest) -> Result<ProjectsResponse> {
        list_projects(self.client, input).await
    }
}
