use crate::{
    admin_console::auth_requests::{approve_auth_request, AuthApproveRequest},
    admin_console::auth_requests::{
        list_pending_requests, PendingAuthRequestsRequest, PendingAuthRequestsResponse,
    },
    error::Result,
    Client,
};

pub struct ClientAuthRequests<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientAuthRequests<'a> {
    pub async fn list(
        &mut self,
        input: &PendingAuthRequestsRequest,
    ) -> Result<PendingAuthRequestsResponse> {
        list_pending_requests(self.client, input).await
    }

    pub async fn approve(&mut self, input: &AuthApproveRequest) -> Result<()> {
        approve_auth_request(self.client, input).await
    }
}

impl<'a> Client {
    pub fn client_auth_requests(&'a mut self) -> ClientAuthRequests<'a> {
        ClientAuthRequests { client: self }
    }
}
