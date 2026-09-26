use crate::Error;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

/// A struct holding the resources linked to the current function. Derive it with `#[derive(Links)]`.
pub trait Links: Sized {
    /// Reads every linked resource the struct declares.
    fn load() -> Result<Self, Error>;
}

#[doc(hidden)]
pub struct Source {
    resources: HashMap<String, Value>,
}

impl Source {
    pub fn init() -> Result<Self, Error> {
        Ok(Self {
            resources: sst_sdk::Resource::init()?.into_inner(),
        })
    }

    pub fn get<T: DeserializeOwned>(&self, name: &str) -> Result<T, Error> {
        let value = self
            .resources
            .get(name)
            .ok_or_else(|| Error::Missing(name.to_string()))?;

        serde_json::from_value(value.clone()).map_err(|source| Error::Shape {
            name: name.to_string(),
            source,
        })
    }
}
