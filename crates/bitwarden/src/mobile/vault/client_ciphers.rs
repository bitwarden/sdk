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
        if cipher_view.key.is_none() && self.client.get_flags().enable_cipher_key_encryption {
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
    use std::collections::HashMap;

    use bitwarden_crypto::{Kdf, SensitiveString};

    use super::*;
    use crate::{
        mobile::crypto::{
            initialize_org_crypto, initialize_user_crypto, InitOrgCryptoRequest,
            InitUserCryptoMethod, InitUserCryptoRequest,
        },
        vault::{login::Login, CipherRepromptType, CipherType},
    };

    #[tokio::test]
    async fn test_decrypt_list() {
        let mut client = Client::new(None);

        initialize_user_crypto(
            &mut client,
            InitUserCryptoRequest {
                kdf_params: Kdf::PBKDF2 {
                    iterations: 600_000.try_into().unwrap(),
                },
                email: "test@bitwarden.com".into(),
                private_key: "2.yN7l00BOlUE0Sb0M//Q53w==|EwKG/BduQRQ33Izqc/ogoBROIoI5dmgrxSo82sgzgAMIBt3A2FZ9vPRMY+GWT85JiqytDitGR3TqwnFUBhKUpRRAq4x7rA6A1arHrFp5Tp1p21O3SfjtvB3quiOKbqWk6ZaU1Np9HwqwAecddFcB0YyBEiRX3VwF2pgpAdiPbSMuvo2qIgyob0CUoC/h4Bz1be7Qa7B0Xw9/fMKkB1LpOm925lzqosyMQM62YpMGkjMsbZz0uPopu32fxzDWSPr+kekNNyLt9InGhTpxLmq1go/pXR2uw5dfpXc5yuta7DB0EGBwnQ8Vl5HPdDooqOTD9I1jE0mRyuBpWTTI3FRnu3JUh3rIyGBJhUmHqGZvw2CKdqHCIrQeQkkEYqOeJRJVdBjhv5KGJifqT3BFRwX/YFJIChAQpebNQKXe/0kPivWokHWwXlDB7S7mBZzhaAPidZvnuIhalE2qmTypDwHy22FyqV58T8MGGMchcASDi/QXI6kcdpJzPXSeU9o+NC68QDlOIrMVxKFeE7w7PvVmAaxEo0YwmuAzzKy9QpdlK0aab/xEi8V4iXj4hGepqAvHkXIQd+r3FNeiLfllkb61p6WTjr5urcmDQMR94/wYoilpG5OlybHdbhsYHvIzYoLrC7fzl630gcO6t4nM24vdB6Ymg9BVpEgKRAxSbE62Tqacxqnz9AcmgItb48NiR/He3n3ydGjPYuKk/ihZMgEwAEZvSlNxYONSbYrIGDtOY+8Nbt6KiH3l06wjZW8tcmFeVlWv+tWotnTY9IqlAfvNVTjtsobqtQnvsiDjdEVtNy/s2ci5TH+NdZluca2OVEr91Wayxh70kpM6ib4UGbfdmGgCo74gtKvKSJU0rTHakQ5L9JlaSDD5FamBRyI0qfL43Ad9qOUZ8DaffDCyuaVyuqk7cz9HwmEmvWU3VQ+5t06n/5kRDXttcw8w+3qClEEdGo1KeENcnXCB32dQe3tDTFpuAIMLqwXs6FhpawfZ5kPYvLPczGWaqftIs/RXJ/EltGc0ugw2dmTLpoQhCqrcKEBDoYVk0LDZKsnzitOGdi9mOWse7Se8798ib1UsHFUjGzISEt6upestxOeupSTOh0v4+AjXbDzRUyogHww3V+Bqg71bkcMxtB+WM+pn1XNbVTyl9NR040nhP7KEf6e9ruXAtmrBC2ah5cFEpLIot77VFZ9ilLuitSz+7T8n1yAh1IEG6xxXxninAZIzi2qGbH69O5RSpOJuJTv17zTLJQIIc781JwQ2TTwTGnx5wZLbffhCasowJKd2EVcyMJyhz6ru0PvXWJ4hUdkARJs3Xu8dus9a86N8Xk6aAPzBDqzYb1vyFIfBxP0oO8xFHgd30Cgmz8UrSE3qeWRrF8ftrI6xQnFjHBGWD/JWSvd6YMcQED0aVuQkuNW9ST/DzQThPzRfPUoiL10yAmV7Ytu4fR3x2sF0Yfi87YhHFuCMpV/DsqxmUizyiJuD938eRcH8hzR/VO53Qo3UIsqOLcyXtTv6THjSlTopQ+JOLOnHm1w8dzYbLN44OG44rRsbihMUQp+wUZ6bsI8rrOnm9WErzkbQFbrfAINdoCiNa6cimYIjvvnMTaFWNymqY1vZxGztQiMiHiHYwTfwHTXrb9j0uPM=|09J28iXv9oWzYtzK2LBT6Yht4IT4MijEkk0fwFdrVQ4=".to_owned(),
                method: InitUserCryptoMethod::Password {
                    password: SensitiveString::test("asdfasdfasdf"),
                    user_key: "2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=".to_owned(),
                },
            },
        )
        .await
        .unwrap();

        let mut organization_keys = HashMap::new();
        organization_keys.insert(
            Uuid::parse_str("1bc9ac1e-f5aa-45f2-94bf-b181009709b8").unwrap(),
            "4.rY01mZFXHOsBAg5Fq4gyXuklWfm6mQASm42DJpx05a+e2mmp+P5W6r54WU2hlREX0uoTxyP91bKKwickSPdCQQ58J45LXHdr9t2uzOYyjVzpzebFcdMw1eElR9W2DW8wEk9+mvtWvKwu7yTebzND+46y1nRMoFydi5zPVLSlJEf81qZZ4Uh1UUMLwXz+NRWfixnGXgq2wRq1bH0n3mqDhayiG4LJKgGdDjWXC8W8MMXDYx24SIJrJu9KiNEMprJE+XVF9nQVNijNAjlWBqkDpsfaWTUfeVLRLctfAqW1blsmIv4RQ91PupYJZDNc8nO9ZTF3TEVM+2KHoxzDJrLs2Q==".parse().unwrap()
        );

        initialize_org_crypto(&mut client, InitOrgCryptoRequest { organization_keys })
            .await
            .unwrap();

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
