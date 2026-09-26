use crate::Error;
use serde::de::DeserializeOwned;
use serde_json::Value;

/// A struct holding the resources linked to the current function. Derive it with `#[derive(Deserialize, Links)]`.
pub trait Links: DeserializeOwned {
    /// Reads every linked resource and deserializes them into the struct, keyed by link name.
    fn load() -> Result<Self, Error> {
        let resources = sst_sdk::Resource::init()?.into_inner();

        Ok(serde_json::from_value(Value::Object(resources.into_iter().collect()))?)
    }
}
