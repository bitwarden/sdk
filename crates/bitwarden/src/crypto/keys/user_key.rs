use crate::crypto::SymmetricCryptoKey;

use super::{AsymmetricKeyPairGeneration, KeyPurpose};

pub struct UserEncryption {}
impl KeyPurpose for UserEncryption {}
impl AsymmetricKeyPairGeneration for UserEncryption {}

pub type UserKey = SymmetricCryptoKey<UserEncryption>;
