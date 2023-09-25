use crate::crypto::SymmetricCryptoKey;

use super::{AsymmetricKeyPairGeneration, KeyPurpose};

pub struct OrganizationEncryption {}
impl KeyPurpose for OrganizationEncryption {}
impl AsymmetricKeyPairGeneration for OrganizationEncryption {}

pub type OrganizationKey = SymmetricCryptoKey<OrganizationEncryption>;
