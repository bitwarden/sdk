use std::sync::Arc;

use crate::{error::Result, Client};

#[derive(uniffi::Object)]
pub struct ClientPasskeys(pub(crate) Arc<Client>);

#[uniffi::export(async_runtime = "tokio")]
impl ClientPasskeys {
    pub async fn passkey_test_sync(&self, t: Arc<dyn TestTraitSync>) -> Result<String> {
        Ok(self
            .0
             .0
            .write()
            .await
            .platform()
            .passkeys()
            .passkey_test_sync(&UniffiTraitBridge(t.as_ref()))
            .await?)
    }

    pub async fn passkey_test_async(&self, t: Arc<dyn TestTraitAsync>) -> Result<String> {
        Ok(self
            .0
             .0
            .write()
            .await
            .platform()
            .passkeys()
            .passkey_test_async(&UniffiTraitBridge(t.as_ref()))
            .await?)
    }
}

// Note that uniffi doesn't support external traits for now it seems, so we have to duplicate them here.

#[uniffi::export(with_foreign)]
pub trait TestTraitSync: Send + Sync {
    fn give_me_a_name(&self) -> String;
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait TestTraitAsync: Send + Sync {
    async fn give_me_a_name(&self) -> String;
}

struct UniffiTraitBridge<T>(T);

impl bitwarden::platform::TestTraitSync for UniffiTraitBridge<&dyn TestTraitSync> {
    fn give_me_a_name(&self) -> String {
        self.0.give_me_a_name()
    }
}

#[async_trait::async_trait]
impl bitwarden::platform::TestTraitAsync for UniffiTraitBridge<&dyn TestTraitAsync> {
    async fn give_me_a_name(&self) -> String {
        self.0.give_me_a_name().await
    }
}
