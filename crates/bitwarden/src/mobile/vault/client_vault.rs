use crate::Client;

pub struct ClientVault<'a> {
    pub(crate) client: &'a crate::Client,
}

impl<'a> Client {
    pub fn vault(&'a self) -> ClientVault<'a> {
        ClientVault { client: self }
    }
}
