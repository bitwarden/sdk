use crate::{
    crypto::{Decryptable, Encryptable},
    error::Result,
    Client,
};

use super::{
    client_vault::ClientVault, CipherDecryptRequest, CipherDecryptResponse, CipherEncryptRequest,
    CipherEncryptResponse,
};

pub struct ClientCiphers<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientCiphers<'a> {
    pub async fn encrypt(&self, req: CipherEncryptRequest) -> Result<CipherEncryptResponse> {
        let enc = self.client.get_encryption_settings()?;

        let cipher = req.cipher.encrypt(enc, &None)?;

        Ok(CipherEncryptResponse { cipher })
    }

    pub async fn decrypt(&self, req: CipherDecryptRequest) -> Result<CipherDecryptResponse> {
        let enc = self.client.get_encryption_settings()?;

        let cipher = req.cipher.decrypt(enc, &None)?;

        Ok(CipherDecryptResponse { cipher })
    }
}

impl<'a> ClientVault<'a> {
    pub fn ciphers(&'a self) -> ClientCiphers<'a> {
        ClientCiphers {
            client: self.client,
        }
    }
}
