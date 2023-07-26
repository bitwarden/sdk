use crate::Client;

pub struct ClientVault<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> Client {
    pub fn vault(&'a mut self) -> ClientVault<'a> {
        ClientVault { client: self }
    }
}
