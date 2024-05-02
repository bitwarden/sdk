use zeroize::Zeroize;

use crate::{CryptoError, Sensitive, SensitiveString, SensitiveVec};

impl SensitiveString {
    pub fn decode_base64<T: base64::Engine>(self, engine: T) -> Result<SensitiveVec, CryptoError> {
        // Prevent accidental copies by allocating the full size
        let len = base64::decoded_len_estimate(self.len());
        let mut value = SensitiveVec::new(Box::new(Vec::with_capacity(len)));

        engine
            .decode_vec(self.as_str(), &mut value.value)
            .map_err(|_| CryptoError::InvalidKey)?;

        Ok(value)
    }
}

impl<T: Zeroize + AsRef<[u8]>> Sensitive<T> {
    pub fn encode_base64<E: base64::Engine>(self, engine: E) -> SensitiveString {
        use base64::engine::Config;

        let inner: &[u8] = self.value.as_ref().as_ref();

        // Prevent accidental copies by allocating the full size
        let padding = engine.config().encode_padding();
        let len = base64::encoded_len(inner.len(), padding).expect("Valid length");

        let mut value = SensitiveVec::new(Box::new(vec![0u8; len]));
        engine
            .encode_slice(inner, &mut value.value[..len])
            .expect("Valid base64 string length");

        value.try_into().expect("Valid base64 string")
    }
}
