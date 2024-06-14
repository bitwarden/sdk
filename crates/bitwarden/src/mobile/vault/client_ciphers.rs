use bitwarden_core::VaultLocked;
use bitwarden_crypto::{CryptoError, KeyDecryptable, KeyEncryptable, LocateKey};
use bitwarden_vault::{Cipher, CipherListView, CipherView};
use uuid::Uuid;

use crate::{error::Result, vault::ClientVault, Client};

pub struct ClientCiphers<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientCiphers<'a> {
    pub fn encrypt(&self, mut cipher_view: CipherView) -> Result<Cipher> {
        let enc = self.client.get_encryption_settings()?;

        // TODO: Once this flag is removed, the key generation logic should
        // be moved directly into the KeyEncryptable implementation
        if cipher_view.key.is_none() && self.client.get_flags().enable_cipher_key_encryption {
            let key = cipher_view.locate_key(&enc, &None).ok_or(VaultLocked)?;
            cipher_view.generate_cipher_key(key)?;
        }

        let key = cipher_view.locate_key(&enc, &None).ok_or(VaultLocked)?;
        let cipher = cipher_view.encrypt_with_key(key)?;

        Ok(cipher)
    }

    pub fn decrypt(&self, cipher: Cipher) -> Result<CipherView> {
        let enc = self.client.get_encryption_settings()?;
        let key = cipher
            .locate_key(&enc, &None)
            .ok_or(CryptoError::MissingKey)?;

        let cipher_view = cipher.decrypt_with_key(key)?;

        Ok(cipher_view)
    }

    pub fn decrypt_list(&self, ciphers: Vec<Cipher>) -> Result<Vec<CipherListView>> {
        let enc = self.client.get_encryption_settings()?;

        let cipher_views: Result<Vec<CipherListView>> = ciphers
            .iter()
            .map(|c| -> Result<CipherListView> {
                let key = c.locate_key(&enc, &None).ok_or(CryptoError::MissingKey)?;
                Ok(c.decrypt_with_key(key)?)
            })
            .collect();

        cipher_views
    }

    pub fn move_to_organization(
        &self,
        mut cipher_view: CipherView,
        organization_id: Uuid,
    ) -> Result<CipherView> {
        let enc = self.client.get_encryption_settings()?;
        cipher_view.move_to_organization(&enc, organization_id)?;
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

    use bitwarden_vault::{Attachment, CipherRepromptType, CipherType, Login};

    use super::*;
    use crate::client::test_accounts::test_bitwarden_com_account;

    #[tokio::test]
    async fn test_decrypt_list() {
        let client = Client::init_test_account(test_bitwarden_com_account()).await;

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

            .unwrap();

        assert_eq!(dec[0].name, "Test item");
    }

    fn test_cipher() -> Cipher {
        Cipher {
    id: Some("358f2b2b-9326-4e5e-94a8-b18100bb0908".parse().unwrap()),
            organization_id: None,
            folder_id: None,
            collection_ids: vec![],
            key: None,
            name: "2.+oPT8B4xJhyhQRe1VkIx0A==|PBtC/bZkggXR+fSnL/pG7g==|UkjRD0VpnUYkjRC/05ZLdEBAmRbr3qWRyJey2bUvR9w=".parse().unwrap(),
            notes: None,
            r#type: CipherType::Login,
            login: Some(Login{
                username: None,
                password: None,
                password_revision_date: None,
                uris:None,
                totp: None,
                autofill_on_page_load: None,
                fido2_credentials: None,
            }),
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
            creation_date: "2024-05-31T11:20:58.4566667Z".parse().unwrap(),
            deleted_date: None,
            revision_date: "2024-05-31T11:20:58.4566667Z".parse().unwrap(),
        }
    }

    fn test_attachment_legacy() -> Attachment {
        Attachment {
            id: Some("uf7bkexzag04d3cw04jsbqqkbpbwhxs0".to_string()),
            url: Some("http://localhost:4000/attachments//358f2b2b-9326-4e5e-94a8-b18100bb0908/uf7bkexzag04d3cw04jsbqqkbpbwhxs0".to_string()),
            file_name: Some("2.mV50WiLq6duhwGbhM1TO0A==|dTufWNH8YTPP0EMlNLIpFA==|QHp+7OM8xHtEmCfc9QPXJ0Ro2BeakzvLgxJZ7NdLuDc=".parse().unwrap()),
            key: None,
            size: Some("65".to_string()),
            size_name: Some("65 Bytes".to_string()),
        }
    }

    fn test_attachment_v2() -> Attachment {
        Attachment {
            id: Some("a77m56oerrz5b92jm05lq5qoyj1xh2t9".to_string()),
            url: Some("http://localhost:4000/attachments//358f2b2b-9326-4e5e-94a8-b18100bb0908/uf7bkexzag04d3cw04jsbqqkbpbwhxs0".to_string()),
            file_name: Some("2.GhazFdCYQcM5v+AtVwceQA==|98bMUToqC61VdVsSuXWRwA==|bsLByMht9Hy5QO9pPMRz0K4d0aqBiYnnROGM5YGbNu4=".parse().unwrap()),
            key: Some("2.6TPEiYULFg/4+3CpDRwCqw==|6swweBHCJcd5CHdwBBWuRN33XRV22VoroDFDUmiM4OzjPEAhgZK57IZS1KkBlCcFvT+t+YbsmDcdv+Lqr+iJ3MmzfJ40MCB5TfYy+22HVRA=|rkgFDh2IWTfPC1Y66h68Diiab/deyi1p/X0Fwkva0NQ=".parse().unwrap()),
            size: Some("65".to_string()),
            size_name: Some("65 Bytes".to_string()),
        }
    }

    #[tokio::test]
    async fn test_move_user_cipher_with_attachment_without_key_to_org_fails() {
        let client = Client::init_test_account(test_bitwarden_com_account()).await;

        let mut cipher = test_cipher();
        cipher.attachments = Some(vec![test_attachment_legacy()]);

        let view = client.vault().ciphers().decrypt(cipher.clone()).unwrap();

        //  Move cipher to organization
        let res = client.vault().ciphers().move_to_organization(
            view,
            "1bc9ac1e-f5aa-45f2-94bf-b181009709b8".parse().unwrap(),
        );

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_encrypt_cipher_with_legacy_attachment_without_key() {
        let client = Client::init_test_account(test_bitwarden_com_account()).await;

        let mut cipher = test_cipher();
        let attachment = test_attachment_legacy();
        cipher.attachments = Some(vec![attachment.clone()]);

        let view = client.vault().ciphers().decrypt(cipher.clone()).unwrap();

        assert!(cipher.key.is_none());

        // Assert the cipher has a key, and the attachment is still readable
        let new_cipher = client.vault().ciphers().encrypt(view).unwrap();
        assert!(new_cipher.key.is_some());

        let view = client.vault().ciphers().decrypt(new_cipher).unwrap();
        let attachments = view.clone().attachments.unwrap();
        let attachment_view = attachments.first().unwrap().clone();
        assert!(attachment_view.key.is_none());

        assert_eq!(attachment_view.file_name.unwrap(), "h.txt");

        let buf = vec![
            2, 100, 205, 148, 152, 77, 184, 77, 53, 80, 38, 240, 83, 217, 251, 118, 254, 27, 117,
            41, 148, 244, 216, 110, 216, 255, 104, 215, 23, 15, 176, 239, 208, 114, 95, 159, 23,
            211, 98, 24, 145, 166, 60, 197, 42, 204, 131, 144, 253, 204, 195, 154, 27, 201, 215,
            43, 10, 244, 107, 226, 152, 85, 167, 66, 185,
        ];

        let content = client
            .vault()
            .attachments()
            .decrypt_buffer(cipher, attachment, buf.as_slice())
            .unwrap();

        assert_eq!(content, b"Hello");
    }

    #[tokio::test]
    async fn test_encrypt_cipher_with_v1_attachment_without_key() {
        let client = Client::init_test_account(test_bitwarden_com_account()).await;

        let mut cipher = test_cipher();
        let attachment = test_attachment_v2();
        cipher.attachments = Some(vec![attachment.clone()]);

        let view = client.vault().ciphers().decrypt(cipher.clone()).unwrap();

        assert!(cipher.key.is_none());

        // Assert the cipher has a key, and the attachment is still readable
        let new_cipher = client.vault().ciphers().encrypt(view).unwrap();
        assert!(new_cipher.key.is_some());

        let view = client.vault().ciphers().decrypt(new_cipher).unwrap();
        let attachments = view.clone().attachments.unwrap();
        let attachment_view = attachments.first().unwrap().clone();
        assert!(attachment_view.key.is_some());

        // Ensure attachment key is updated since it's now protected by the cipher key
        assert_ne!(
            attachment.clone().key.unwrap().to_string(),
            attachment_view.clone().key.unwrap().to_string()
        );

        assert_eq!(attachment_view.file_name.unwrap(), "h.txt");

        let buf = vec![
            2, 114, 53, 72, 20, 82, 18, 46, 48, 137, 97, 1, 100, 142, 120, 187, 28, 36, 180, 46,
            189, 254, 133, 23, 169, 58, 73, 212, 172, 116, 185, 127, 111, 92, 112, 145, 99, 28,
            158, 198, 48, 241, 121, 218, 66, 37, 152, 197, 122, 241, 110, 82, 245, 72, 47, 230, 95,
            188, 196, 170, 127, 67, 44, 129, 90,
        ];

        let content = client
            .vault()
            .attachments()
            .decrypt_buffer(cipher, attachment, buf.as_slice())
            .unwrap();

        assert_eq!(content, b"Hello");

        // Move cipher to organization
        let new_view = client
            .vault()
            .ciphers()
            .move_to_organization(
                view,
                "1bc9ac1e-f5aa-45f2-94bf-b181009709b8".parse().unwrap(),
            )
            .unwrap();
        let new_cipher = client.vault().ciphers().encrypt(new_view).unwrap();

        let attachment = new_cipher
            .clone()
            .attachments
            .unwrap()
            .first()
            .unwrap()
            .clone();

        // Ensure attachment key is still the same since it's protected by the cipher key
        assert_eq!(
            attachment.clone().key.unwrap().to_string(),
            attachment_view.key.unwrap().to_string()
        );

        let content = client
            .vault()
            .attachments()
            .decrypt_buffer(new_cipher, attachment, buf.as_slice())
            .unwrap();

        assert_eq!(content, b"Hello");
    }
}
