use base64::Engine;
use bitwarden_api_identity::models::{KdfType, PreloginResponseModel};
use std::num::NonZeroU32;

use crate::{
    crypto::{PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE},
    util::{default_kdf_iterations, BASE64_ENGINE},
};

#[derive(Debug)]
pub(crate) struct AuthSettings {
    pub email: String,
    kdf_type: KdfType,
    pub(crate) kdf_iterations: NonZeroU32,
}

impl AuthSettings {
    pub fn new(response: PreloginResponseModel, email: String) -> Self {
        Self {
            email,
            kdf_type: response.kdf.unwrap_or_default(),
            kdf_iterations: response
                .kdf_iterations
                .and_then(|e| NonZeroU32::new(e as u32))
                .unwrap_or_else(default_kdf_iterations),
        }
    }

    pub fn make_user_password_hash(&self, password: &str) -> String {
        self.make_password_hash(password, &self.email)
    }

    pub fn make_password_hash(&self, password: &str, salt: &str) -> String {
        let hash = match self.kdf_type {
            KdfType::_0 => pbkdf2::pbkdf2_array::<PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE>(
                password.as_bytes(),
                salt.as_bytes(),
                self.kdf_iterations.get(),
            ),
        }
        .unwrap();

        // Server expects hash + 1 iteration
        let login_hash = pbkdf2::pbkdf2_array::<PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE>(
            &hash,
            password.as_bytes(),
            1,
        )
        .unwrap();

        BASE64_ENGINE.encode(login_hash)
    }
}

#[cfg(test)]
mod tests {
    use bitwarden_api_identity::models::{KdfType, PreloginResponseModel};

    use super::AuthSettings;

    #[test]
    fn test_password_hash() {
        let res = PreloginResponseModel {
            kdf: Some(KdfType::_0),
            kdf_iterations: Some(100_000),
        };
        let settings = AuthSettings::new(res, "test@bitwarden.com".into());

        assert_eq!(
            settings.make_password_hash("asdfasdf", "test_salt"),
            "ZF6HjxUTSyBHsC+HXSOhZoXN+UuMnygV5YkWXCY4VmM="
        );
        assert_eq!(
            settings.make_user_password_hash("asdfasdf"),
            "wmyadRMyBZOH7P/a/ucTCbSghKgdzDpPqUnu/DAVtSw="
        );
    }
}
