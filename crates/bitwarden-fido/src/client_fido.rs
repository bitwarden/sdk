use bitwarden_core::Client;

use crate::{Fido2Authenticator, Fido2Client, Fido2CredentialStore, Fido2UserInterface};

pub struct ClientFido2<'a> {
    #[allow(dead_code)]
    pub(crate) client: &'a Client,
}

impl<'a> ClientFido2<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create_authenticator(
        &'a self,
        user_interface: &'a dyn Fido2UserInterface,
        credential_store: &'a dyn Fido2CredentialStore,
    ) -> Fido2Authenticator<'a> {
        Fido2Authenticator::new(self.client, user_interface, credential_store)
    }

    pub fn create_client(
        &'a self,
        user_interface: &'a dyn Fido2UserInterface,
        credential_store: &'a dyn Fido2CredentialStore,
    ) -> Fido2Client<'a> {
        Fido2Client {
            authenticator: self.create_authenticator(user_interface, credential_store),
        }
    }
}

pub trait ClientFido2Ext<'a> {
    fn fido2(&'a self) -> ClientFido2<'a>;
}

impl<'a> ClientFido2Ext<'a> for Client {
    fn fido2(&'a self) -> ClientFido2<'a> {
        ClientFido2::new(self)
    }
}
