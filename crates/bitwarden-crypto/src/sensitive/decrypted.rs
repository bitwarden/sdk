use zeroize::Zeroize;

use crate::{CryptoError, CryptoKey, KeyEncryptable, Sensitive, SensitiveString};

/// Type alias for a [`Sensitive`] value to denote decrypted data.
pub type Decrypted<V> = Sensitive<V>;
pub type DecryptedVec = Decrypted<Vec<u8>>;
pub type DecryptedString = SensitiveString;

impl<T: KeyEncryptable<Key, Output> + Zeroize + Clone, Key: CryptoKey, Output>
    KeyEncryptable<Key, Output> for Decrypted<T>
{
    fn encrypt_with_key(self, key: &Key) -> Result<Output, CryptoError> {
        self.value.clone().encrypt_with_key(key)
    }
}
