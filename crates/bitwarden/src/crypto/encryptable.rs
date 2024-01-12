use crate::{
    client::encryption_settings::EncryptionSettings,
    error::{Error, Result},
};

use super::{KeyDecryptable, KeyEncryptable, KeyPurpose, SymmetricCryptoKey};

pub trait LocateKey<Purpose: KeyPurpose> {
    fn locate_key<'a>(
        &self,
        enc: &'a EncryptionSettings,
    ) -> Option<&'a SymmetricCryptoKey<Purpose>>;
}

/// Deprecated: please use LocateKey and KeyDecryptable instead
pub trait Encryptable<Purpose: KeyPurpose, Output> {
    fn encrypt(self, enc: &EncryptionSettings) -> Result<Output>;
}

/// Deprecated: please use LocateKey and KeyDecryptable instead
pub trait Decryptable<Purpose: KeyPurpose, Output> {
    fn decrypt(&self, enc: &EncryptionSettings) -> Result<Output>;
}

impl<
        T: KeyEncryptable<SymmetricCryptoKey<Purpose>, Purpose, Output> + LocateKey<Purpose>,
        Purpose: KeyPurpose,
        Output,
    > Encryptable<Purpose, Output> for T
{
    fn encrypt(self, enc: &EncryptionSettings) -> Result<Output> {
        let key = self.locate_key(enc).ok_or(Error::VaultLocked)?;
        self.encrypt_with_key(key)
    }
}

impl<
        Purpose: KeyPurpose,
        T: KeyDecryptable<SymmetricCryptoKey<Purpose>, Purpose, Output> + LocateKey<Purpose>,
        Output,
    > Decryptable<Purpose, Output> for T
{
    fn decrypt(&self, enc: &EncryptionSettings) -> Result<Output> {
        let key = self.locate_key(enc).ok_or(Error::VaultLocked)?;
        self.decrypt_with_key(key)
    }
}
