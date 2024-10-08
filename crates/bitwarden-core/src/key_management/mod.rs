use bitwarden_crypto::{key_refs, service::CryptoService, SymmetricCryptoKey};

key_refs! {
    #[symmetric]
    pub enum SymmetricKeyRef {
        Master,
        User,
        Organization(uuid::Uuid),
        #[local]
        Local(&'static str),
    }

    #[asymmetric]
    pub enum AsymmetricKeyRef {
        UserPrivateKey,
        #[local]
        Local(&'static str),
    }
}

pub fn create_test_crypto_with_user_key(
    key: SymmetricCryptoKey,
) -> CryptoService<SymmetricKeyRef, AsymmetricKeyRef> {
    let service = CryptoService::new();

    #[allow(deprecated)]
    service
        .context_mut()
        .set_symmetric_key(SymmetricKeyRef::User, key.clone())
        .expect("Mutable context");

    service
}

pub fn create_test_crypto_with_user_and_org_key(
    key: SymmetricCryptoKey,
    org_id: uuid::Uuid,
    org_key: SymmetricCryptoKey,
) -> CryptoService<SymmetricKeyRef, AsymmetricKeyRef> {
    let service = CryptoService::new();

    #[allow(deprecated)]
    service
        .context_mut()
        .set_symmetric_key(SymmetricKeyRef::User, key.clone())
        .expect("Mutable context");

    #[allow(deprecated)]
    service
        .context_mut()
        .set_symmetric_key(SymmetricKeyRef::Organization(org_id), org_key.clone())
        .expect("Mutable context");

    service
}
