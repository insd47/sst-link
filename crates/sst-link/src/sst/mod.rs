pub mod aws;

use serde::Deserialize;

/// The app and stage, linked to every function by SST.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct App {
    pub name: String,
    pub stage: String,
}

/// `sst.Secret`. It has no `Debug`, so a secret can't end up in a log by accident.
#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Secret {
    pub value: String,
}
