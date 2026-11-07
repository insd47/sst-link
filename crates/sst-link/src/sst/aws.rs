use serde::Deserialize;

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
