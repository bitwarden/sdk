use std::{collections::HashMap, marker::PhantomData};

use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

use crate::error::Result;

use super::{domain::*, state::State};

#[derive(Clone, Copy)]
pub(crate) struct ServiceDefinition<T: Serialize + DeserializeOwned> {
    namespace: &'static str,

    // This is only used so the compiler doesn't complain that T is unused
    _type: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> ServiceDefinition<T> {
    const fn new(namespace: &'static str) -> Self {
        let _type = PhantomData;
        Self { namespace, _type }
    }
}

pub(crate) const KEYS_SERVICE: ServiceDefinition<Option<Keys>> = ServiceDefinition::new("keys");
pub(crate) const PROFILE_SERVICE: ServiceDefinition<Option<Profile>> =
    ServiceDefinition::new("profile");
pub(crate) const CIPHERS_SERVICE: ServiceDefinition<HashMap<Uuid, Cipher>> =
    ServiceDefinition::new("ciphers");
pub(crate) const FOLDERS_SERVICE: ServiceDefinition<HashMap<Uuid, Folder>> =
    ServiceDefinition::new("folders");
pub(crate) const AUTH_SERVICE: ServiceDefinition<Auth> = ServiceDefinition::new("auth");
pub(crate) const SETTINGS_SERVICE: ServiceDefinition<Settings> = ServiceDefinition::new("settings");

pub(crate) struct StateService<'a, T: Serialize + DeserializeOwned + Default> {
    state: &'a State,
    definition: ServiceDefinition<T>,
}

impl<'a, T: Serialize + DeserializeOwned + Default> StateService<'a, T> {
    pub async fn get(&self) -> T {
        self.state
            .account
            .lock()
            .await
            .get()
            .get(self.definition.namespace)
            .map(|v| serde_json::from_value(v.clone()))
            .transpose()
            .unwrap()
            .unwrap_or_default()
    }

    pub async fn modify_opt<'b>(
        &self,
        modify_fn: impl FnOnce(&mut Option<T>) -> Result<()> + Send + 'b,
    ) -> Result<()> {
        self.state
            .account
            .lock()
            .await
            .modify(|state_opt| {
                let mut state = std::mem::take(state_opt).unwrap_or_default();

                let mut value: Option<T> = state
                    .remove(self.definition.namespace)
                    .map(|v| serde_json::from_value(v).unwrap());

                modify_fn(&mut value)?;

                if let Some(value) = value {
                    state.insert(
                        self.definition.namespace.to_owned(),
                        serde_json::to_value(value).unwrap(),
                    );
                }

                *state_opt = Some(state);
                Ok(())
            })
            .await
    }

    pub async fn modify<'b>(
        &self,
        modify_fn: impl FnOnce(&mut T) -> Result<()> + Send + 'b,
    ) -> Result<()> {
        self.modify_opt(move |state| {
            let mut value = std::mem::take(state).unwrap_or_default();
            modify_fn(&mut value)?;
            *state = Some(value);
            Ok(())
        })
        .await
    }
}

impl crate::Client {
    pub(crate) fn get_state_service<T: Serialize + DeserializeOwned + Default>(
        &self,
        definition: ServiceDefinition<T>,
    ) -> StateService<'_, T> {
        self.state.get_state_service(definition)
    }
}

impl State {
    pub(crate) fn get_state_service<T: Serialize + DeserializeOwned + Default>(
        &self,
        definition: ServiceDefinition<T>,
    ) -> StateService<'_, T> {
        StateService {
            state: &self,
            definition,
        }
    }
}
