use crate::{error::Result, Client};

pub struct ClientPasskeys<'a> {
    #[allow(dead_code)]
    pub(crate) client: &'a mut Client,
}

impl<'a> ClientPasskeys<'a> {
    pub async fn passkey_test_sync(&self, t: &dyn TestTraitSync) -> Result<String> {
        Ok(format!("Hello {}!", t.give_me_a_name()))
    }

    pub async fn passkey_test_async(&self, t: &dyn TestTraitAsync) -> Result<String> {
        Ok(format!(
            "Hello {}, we're using async!",
            t.give_me_a_name().await
        ))
    }
}

//#[cfg_attr(feature = "mobile", uniffi::export(with_foreign))]
pub trait TestTraitSync: Send + Sync {
    fn give_me_a_name(&self) -> String;
}

//#[cfg_attr(feature = "mobile", uniffi::export(with_foreign))]
#[async_trait::async_trait]
pub trait TestTraitAsync: Send + Sync {
    async fn give_me_a_name(&self) -> String;
}
