use bitwarden_crypto::{CryptoError, KeyDecryptable, LocateKey};

use crate::{
    error::Result,
    vault::{ClientVault, Collection, CollectionView},
    Client,
};

pub struct ClientCollections<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientCollections<'a> {
    pub async fn decrypt(&self, collection: Collection) -> Result<CollectionView> {
        let enc = self.client.get_encryption_settings()?;
        let key = collection
            .locate_key(enc, &None)
            .ok_or(CryptoError::MissingKey)?;

        let view = collection.decrypt_with_key(key)?;

        Ok(view)
    }

    pub async fn decrypt_list(&self, collections: Vec<Collection>) -> Result<Vec<CollectionView>> {
        let enc = self.client.get_encryption_settings()?;

        let views: Result<Vec<CollectionView>> = collections
            .iter()
            .map(|c| -> Result<CollectionView> {
                let key = c.locate_key(enc, &None).ok_or(CryptoError::MissingKey)?;
                Ok(c.decrypt_with_key(key)?)
            })
            .collect();

        views
    }
}

impl<'a> ClientVault<'a> {
    pub fn collections(&'a self) -> ClientCollections<'a> {
        ClientCollections {
            client: self.client,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::test_accounts::test_bitwarden_com_account, vault::Collection, Client};

    #[tokio::test]
    async fn test_decrypt_list() {
        let mut client = Client::init_test_account(test_bitwarden_com_account()).await;

        let dec = client.vault().collections().decrypt_list(vec![Collection {
            id: Some("66c5ca57-0868-4c7e-902f-b181009709c0".parse().unwrap()),
            organization_id: "1bc9ac1e-f5aa-45f2-94bf-b181009709b8".parse().unwrap(),
            name: "2.EI9Km5BfrIqBa1W+WCccfA==|laWxNnx+9H3MZww4zm7cBSLisjpi81zreaQntRhegVI=|x42+qKFf5ga6DIL0OW5pxCdLrC/gm8CXJvf3UASGteI=".parse().unwrap(),
            external_id: None,
            hide_passwords: false,
            read_only: false,
        }]).await.unwrap();

        assert_eq!(dec[0].name, "Default collection");
    }

    #[tokio::test]
    async fn test_decrypt() {
        let mut client = Client::init_test_account(test_bitwarden_com_account()).await;

        let dec = client.vault().collections().decrypt(Collection {
            id: Some("66c5ca57-0868-4c7e-902f-b181009709c0".parse().unwrap()),
            organization_id: "1bc9ac1e-f5aa-45f2-94bf-b181009709b8".parse().unwrap(),
            name: "2.EI9Km5BfrIqBa1W+WCccfA==|laWxNnx+9H3MZww4zm7cBSLisjpi81zreaQntRhegVI=|x42+qKFf5ga6DIL0OW5pxCdLrC/gm8CXJvf3UASGteI=".parse().unwrap(),
            external_id: None,
            hide_passwords: false,
            read_only: false,
        }).await.unwrap();

        assert_eq!(dec.name, "Default collection");
    }
}
