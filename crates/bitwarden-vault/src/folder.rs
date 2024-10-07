use bitwarden_api_api::models::FolderResponseModel;
use bitwarden_core::{
    key_management::{AsymmetricKeyRef, SymmetricKeyRef},
    require,
};
use bitwarden_crypto::{
    service::CryptoServiceContext, CryptoError, Decryptable, EncString, Encryptable, UsesKey,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::VaultParseError;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Folder {
    id: Option<Uuid>,
    name: EncString,
    revision_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct FolderView {
    pub id: Option<Uuid>,
    pub name: String,
    pub revision_date: DateTime<Utc>,
}

impl Encryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, Folder> for FolderView {
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<Folder, CryptoError> {
        Ok(Folder {
            id: self.id,
            name: self.name.encrypt(ctx, key)?,
            revision_date: self.revision_date,
        })
    }
}

impl Decryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, FolderView> for Folder {
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<FolderView, CryptoError> {
        Ok(FolderView {
            id: self.id,
            name: self.name.decrypt(ctx, key).ok().unwrap_or_default(),
            revision_date: self.revision_date,
        })
    }
}

impl UsesKey<SymmetricKeyRef> for Folder {
    fn uses_key(&self) -> SymmetricKeyRef {
        SymmetricKeyRef::User
    }
}

impl UsesKey<SymmetricKeyRef> for FolderView {
    fn uses_key(&self) -> SymmetricKeyRef {
        SymmetricKeyRef::User
    }
}

impl TryFrom<FolderResponseModel> for Folder {
    type Error = VaultParseError;

    fn try_from(folder: FolderResponseModel) -> Result<Self, Self::Error> {
        Ok(Folder {
            id: folder.id,
            name: require!(EncString::try_from_optional(folder.name)?),
            revision_date: require!(folder.revision_date).parse()?,
        })
    }
}
