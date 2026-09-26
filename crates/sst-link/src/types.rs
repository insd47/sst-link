use serde::Deserialize;

/// The app and stage, linked to every function by SST.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct App {
    pub name: String,
    pub stage: String,
}

/// `sst.aws.Bucket`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Bucket {
    pub name: String,
}

/// `sst.aws.Dynamo`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Dynamo {
    pub name: String,
}

/// `sst.aws.Function`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Function {
    pub name: String,
}

/// `sst.aws.Realtime`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Realtime {
    pub endpoint: String,
    pub authorizer: String,
}

/// `sst.aws.Router`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Router {
    pub url: String,
}

/// `sst.Secret`. It has no `Debug`, so a secret can't end up in a log by accident.
#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Secret {
    pub value: String,
}
