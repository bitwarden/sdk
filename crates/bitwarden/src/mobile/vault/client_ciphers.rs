use bitwarden_crypto::{CryptoError, KeyDecryptable, KeyEncryptable, LocateKey};
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    vault::{Cipher, CipherListView, CipherView, ClientVault},
    Client,
};

pub struct ClientCiphers<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientCiphers<'a> {
    pub async fn encrypt(&self, mut cipher_view: CipherView) -> Result<Cipher> {
        let enc = self.client.get_encryption_settings()?;

        // TODO: Once this flag is removed, the key generation logic should
        // be moved directly into the KeyEncryptable implementation

        // If any attachments have no keys, they will need to be reuploaded before we can
        // generate them a cipher key, so in that case we skip them even if the flag is set
        let all_attachments_have_keys = cipher_view
            .attachments
            .iter()
            .flatten()
            .all(|a| a.key.is_some());
        if cipher_view.key.is_none()
            && self.client.get_flags().enable_cipher_key_encryption
            && all_attachments_have_keys
        {
            let key = cipher_view
                .locate_key(enc, &None)
                .ok_or(Error::VaultLocked)?;
            cipher_view.generate_cipher_key(key)?;
        }

        let key = cipher_view
            .locate_key(enc, &None)
            .ok_or(Error::VaultLocked)?;
        let cipher = cipher_view.encrypt_with_key(key)?;

        Ok(cipher)
    }

    pub async fn decrypt(&self, cipher: Cipher) -> Result<CipherView> {
        let enc = self.client.get_encryption_settings()?;
        let key = cipher
            .locate_key(enc, &None)
            .ok_or(CryptoError::MissingKey)?;

        let cipher_view = cipher.decrypt_with_key(key)?;

        Ok(cipher_view)
    }

    pub async fn decrypt_list(&self, ciphers: Vec<Cipher>) -> Result<Vec<CipherListView>> {
        let enc = self.client.get_encryption_settings()?;

        let cipher_views: Result<Vec<CipherListView>> = ciphers
            .iter()
            .map(|c| -> Result<CipherListView> {
                let key = c.locate_key(enc, &None).ok_or(CryptoError::MissingKey)?;
                Ok(c.decrypt_with_key(key)?)
            })
            .collect();

        cipher_views
    }

    pub async fn move_to_organization(
        &self,
        mut cipher_view: CipherView,
        organization_id: Uuid,
    ) -> Result<CipherView> {
        let enc = self.client.get_encryption_settings()?;
        cipher_view.move_to_organization(enc, organization_id)?;
        Ok(cipher_view)
    }
}

impl<'a> ClientVault<'a> {
    pub fn ciphers(&'a self) -> ClientCiphers<'a> {
        ClientCiphers {
            client: self.client,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        client::test_accounts::test_bitwarden_com_account,
        vault::{login::Login, CipherRepromptType, CipherType},
    };

    #[tokio::test]
    async fn test_decrypt_list() {
        let mut client = Client::init_test_account(test_bitwarden_com_account()).await;

        let dec = client
            .vault()
            .ciphers()
            .decrypt_list(vec![Cipher {
                id: Some("a1569f46-0797-4d3f-b859-b181009e2e49".parse().unwrap()),
                organization_id: Some("1bc9ac1e-f5aa-45f2-94bf-b181009709b8".parse().unwrap()),
                folder_id: None,
                collection_ids: vec!["66c5ca57-0868-4c7e-902f-b181009709c0".parse().unwrap()],
                key: None,
                name: "2.RTdUGVWYl/OZHUMoy68CMg==|sCaT5qHx8i0rIvzVrtJKww==|jB8DsRws6bXBtXNfNXUmFJ0JLDlB6GON6Y87q0jgJ+0=".parse().unwrap(),
                notes: None,
                r#type: CipherType::Login,
                login: Some(Login{
                    username: Some("2.ouEYEk+SViUtqncesfe9Ag==|iXzEJq1zBeNdDbumFO1dUA==|RqMoo9soSwz/yB99g6YPqk8+ASWRcSdXsKjbwWzyy9U=".parse().unwrap()),
                    password: Some("2.6yXnOz31o20Z2kiYDnXueA==|rBxTb6NK9lkbfdhrArmacw==|ogZir8Z8nLgiqlaLjHH+8qweAtItS4P2iPv1TELo5a0=".parse().unwrap()),
                    password_revision_date: None, uris:None, totp: None, autofill_on_page_load: None, fido2_credentials: None }),
                identity: None,
                card: None,
                secure_note: None,
                favorite: false,
                reprompt: CipherRepromptType::None,
                organization_use_totp: true,
                edit: true,
                view_password: true,
                local_data: None,
                attachments: None,
                fields:  None,
                password_history: None,
                creation_date: "2024-05-31T09:35:55.12Z".parse().unwrap(),
                deleted_date: None,
                revision_date: "2024-05-31T09:35:55.12Z".parse().unwrap(),
            }])
            .await
            .unwrap();

        assert_eq!(dec[0].name, "Test item");
    }
}
