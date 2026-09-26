/// Why the linked resources could not be loaded.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// SST's resource payload could not be read or decrypted.
    #[error("failed to read SST resources: {0}")]
    Resource(#[from] sst_sdk::ResourceError),

    /// The function has no link with this name.
    #[error("`{0}` is not linked to this function")]
    Missing(String),

    /// The link exists but doesn't match the field's type.
    #[error("`{name}` does not match its declared shape: {source}")]
    Shape { name: String, source: serde_json::Error },
}
