use bitwarden_crypto::{service::CryptoService, AsymmetricCryptoKey, SymmetricCryptoKey};
#[cfg(feature = "internal")]
use bitwarden_crypto::{AsymmetricEncString, EncString};
use thiserror::Error;
use uuid::Uuid;

#[cfg(feature = "internal")]
use crate::error::Result;
use crate::{
    key_management::{AsymmetricKeyRef, SymmetricKeyRef},
    VaultLocked,
};

#[derive(Debug, Error)]
pub enum EncryptionSettingsError {
    #[error("Cryptography error, {0}")]
    Crypto(#[from] bitwarden_crypto::CryptoError),

    #[error(transparent)]
    InvalidBase64(#[from] base64::DecodeError),

    #[error(transparent)]
    VaultLocked(#[from] VaultLocked),

    #[error("Invalid private key")]
    InvalidPrivateKey,

    #[error("Missing private key")]
    MissingPrivateKey,
}

pub struct EncryptionSettings {}

impl EncryptionSettings {
    /// Initialize the encryption settings with the decrypted user key and the encrypted user
    /// private key This should only be used when unlocking the vault via biometrics or when the
    /// vault is set to lock: "never" Otherwise handling the decrypted user key is dangerous and
    /// discouraged
    #[cfg(feature = "internal")]
    pub(crate) fn new_decrypted_key(
        user_key: SymmetricCryptoKey,
        private_key: EncString,
        crypto_service: &CryptoService<SymmetricKeyRef, AsymmetricKeyRef>,
    ) -> Result<(), EncryptionSettingsError> {
        use bitwarden_crypto::KeyDecryptable;
        use log::warn;

        let private_key = {
            let dec: Vec<u8> = private_key.decrypt_with_key(&user_key)?;

            // FIXME: [PM-11690] - Temporarily ignore invalid private keys until we have a recovery
            // process in place.
            AsymmetricCryptoKey::from_der(&dec)
                .map_err(|_| {
                    warn!("Invalid private key");
                })
                .ok()

            // Some(
            //     AsymmetricCryptoKey::from_der(&dec)
            //         .map_err(|_| EncryptionSettingsError::InvalidPrivateKey)?,
            // )
        };

        #[allow(deprecated)]
        {
            let mut ctx = crypto_service.context_mut();
            ctx.set_symmetric_key(SymmetricKeyRef::User, user_key)?;
            if let Some(private_key) = private_key {
                ctx.set_asymmetric_key(AsymmetricKeyRef::UserPrivateKey, private_key)?;
            }
        }

        Ok(())
    }

    /// Initialize the encryption settings with only a single decrypted key.
    /// This is used only for logging in Secrets Manager with an access token
    #[cfg(feature = "secrets")]
    pub(crate) fn new_single_key(
        key: SymmetricCryptoKey,
        crypto_service: &CryptoService<SymmetricKeyRef, AsymmetricKeyRef>,
    ) {
        #[allow(deprecated)]
        crypto_service
            .context_mut()
            .set_symmetric_key(SymmetricKeyRef::User, key)
            .expect("Mutable context");
    }

    #[cfg(feature = "internal")]
    pub(crate) fn set_org_keys(
        org_enc_keys: Vec<(Uuid, AsymmetricEncString)>,
        crypto_service: &CryptoService<SymmetricKeyRef, AsymmetricKeyRef>,
    ) -> Result<(), EncryptionSettingsError> {
        let mut ctx = crypto_service.context_mut();

        if !ctx.has_asymmetric_key(AsymmetricKeyRef::UserPrivateKey) {
            return Err(VaultLocked.into());
        }

        // Make sure we only keep the keys given in the arguments and not any of the previous
        // ones, which might be from organizations that the user is no longer a part of anymore
        ctx.retain_symmetric_keys(|key_ref| !matches!(key_ref, SymmetricKeyRef::Organization(_)));

        // FIXME: [PM-11690] - Early abort to handle private key being corrupt
        if org_enc_keys.is_empty() {
            return Ok(());
        }

        // Decrypt the org keys with the private key
        for (org_id, org_enc_key) in org_enc_keys {
            ctx.decrypt_symmetric_key_with_asymmetric_key(
                AsymmetricKeyRef::UserPrivateKey,
                SymmetricKeyRef::Organization(org_id),
                &org_enc_key,
            )?;
        }

        Ok(())
    }
}
