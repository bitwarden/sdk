use std::{collections::HashMap, hash::Hash};

use crate::error::Result;

pub trait CryptoKey<Purpose: KeyPurpose> {}

pub trait KeyPurpose {}

macro_rules! key_purpose {
    ( $( $purpose:ident ,)* ) => {
        $(
            pub struct $purpose;
            impl crate::crypto::KeyPurpose for $purpose {}
        )*
    };
}

pub mod purpose {
    key_purpose![
        Testing,
        Master,
        Shareable,
        PayloadEncryption,
        UserEncryption,
        OrgEncryption,
        UserOrOrgEncryption,
        SendEncryption,
        CipherEncryption,
    ];
}

pub trait KeyEncryptable<Key: CryptoKey<Purpose>, Purpose: KeyPurpose, Output> {
    fn encrypt_with_key(self, key: &Key) -> Result<Output>;
}

pub trait KeyDecryptable<Key: CryptoKey<Purpose>, Purpose: KeyPurpose, Output> {
    fn decrypt_with_key(&self, key: &Key) -> Result<Output>;
}

impl<
        T: KeyEncryptable<Key, Purpose, Output>,
        Key: CryptoKey<Purpose>,
        Purpose: KeyPurpose,
        Output,
    > KeyEncryptable<Key, Purpose, Option<Output>> for Option<T>
{
    fn encrypt_with_key(self, key: &Key) -> Result<Option<Output>> {
        self.map(|e| e.encrypt_with_key(key)).transpose()
    }
}

impl<
        T: KeyDecryptable<Key, Purpose, Output>,
        Key: CryptoKey<Purpose>,
        Purpose: KeyPurpose,
        Output,
    > KeyDecryptable<Key, Purpose, Option<Output>> for Option<T>
{
    fn decrypt_with_key(&self, key: &Key) -> Result<Option<Output>> {
        self.as_ref().map(|e| e.decrypt_with_key(key)).transpose()
    }
}

impl<
        T: KeyEncryptable<Key, Purpose, Output>,
        Key: CryptoKey<Purpose>,
        Purpose: KeyPurpose,
        Output,
    > KeyEncryptable<Key, Purpose, Output> for Box<T>
{
    fn encrypt_with_key(self, key: &Key) -> Result<Output> {
        (*self).encrypt_with_key(key)
    }
}

impl<
        T: KeyDecryptable<Key, Purpose, Output>,
        Key: CryptoKey<Purpose>,
        Purpose: KeyPurpose,
        Output,
    > KeyDecryptable<Key, Purpose, Output> for Box<T>
{
    fn decrypt_with_key(&self, key: &Key) -> Result<Output> {
        (**self).decrypt_with_key(key)
    }
}

impl<
        T: KeyEncryptable<Key, Purpose, Output>,
        Key: CryptoKey<Purpose>,
        Purpose: KeyPurpose,
        Output,
    > KeyEncryptable<Key, Purpose, Vec<Output>> for Vec<T>
{
    fn encrypt_with_key(self, key: &Key) -> Result<Vec<Output>> {
        self.into_iter().map(|e| e.encrypt_with_key(key)).collect()
    }
}

impl<
        T: KeyDecryptable<Key, Purpose, Output>,
        Key: CryptoKey<Purpose>,
        Purpose: KeyPurpose,
        Output,
    > KeyDecryptable<Key, Purpose, Vec<Output>> for Vec<T>
{
    fn decrypt_with_key(&self, key: &Key) -> Result<Vec<Output>> {
        self.iter().map(|e| e.decrypt_with_key(key)).collect()
    }
}

impl<
        T: KeyEncryptable<Key, Purpose, Output>,
        Key: CryptoKey<Purpose>,
        Purpose: KeyPurpose,
        Output,
        Id: Hash + Eq,
    > KeyEncryptable<Key, Purpose, HashMap<Id, Output>> for HashMap<Id, T>
{
    fn encrypt_with_key(self, key: &Key) -> Result<HashMap<Id, Output>> {
        self.into_iter()
            .map(|(id, e)| Ok((id, e.encrypt_with_key(key)?)))
            .collect()
    }
}

impl<
        T: KeyDecryptable<Key, Purpose, Output>,
        Key: CryptoKey<Purpose>,
        Purpose: KeyPurpose,
        Output,
        Id: Hash + Eq + Copy,
    > KeyDecryptable<Key, Purpose, HashMap<Id, Output>> for HashMap<Id, T>
{
    fn decrypt_with_key(&self, key: &Key) -> Result<HashMap<Id, Output>> {
        self.iter()
            .map(|(id, e)| Ok((*id, e.decrypt_with_key(key)?)))
            .collect()
    }
}
