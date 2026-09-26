/// Why the linked resources could not be loaded.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// SST's resource payload could not be read or decrypted.
    #[error("failed to read SST resources: {0}")]
    Resource(#[from] sst_sdk::ResourceError),

    /// A link is missing or doesn't match its field's type. serde's message names the field.
    #[error("linked resources don't match: {0}")]
    Deserialize(#[from] serde_json::Error),
}
