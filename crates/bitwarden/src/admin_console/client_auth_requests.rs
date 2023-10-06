use crate::{
    error::Result,
    Client,
    admin_console::auth_requests::{PendingAuthRequestsRequest, PendingAuthRequestsResponse, list_pending_requests}
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
}

impl<'a> Client {
    pub fn client_auth_requests(&'a mut self) -> ClientAuthRequests<'a> {
        ClientAuthRequests { client: self }
    }
}
