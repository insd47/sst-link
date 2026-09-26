//! Typed SST resource links for Rust.
//!
//! Declare one struct per function that mirrors its `link` array, derive [`Links`], and load it once at startup.
//! Every field is required, so a missing link fails at cold start with its name instead of halfway through a request.
//!
//! ```no_run
//! use serde::Deserialize;
//! use sst_link::{Bucket, Links, Secret};
//!
//! #[derive(Deserialize)]
//! struct Registry {
//!     audience: String,
//! }
//!
//! #[derive(Links)]
//! struct Resources {
//!     key: Secret,                    // link "Key"
//!     registry: Registry,             // link "Registry"
//!     #[links(name = "RouterStorage")] // a link whose name doesn't follow the field
//!     storage: Bucket,
//! }
//!
//! let resources = Resources::load()?;
//! # Ok::<(), sst_link::Error>(())
//! ```
//!
//! Field names map to link names in PascalCase (`key` → `Key`). Any `Deserialize` type can be a field: the shapes of
//! SST's own components ship with this crate, and your own `sst.Linkable`s bring their own structs.

pub use error::Error;
pub use links::Links;
pub use sst_link_derive::Links;
pub use types::{App, Bucket, Dynamo, Function, Realtime, Router, Secret};

mod error;
mod links;
mod types;

#[doc(hidden)]
pub mod __private {
    pub use crate::links::Source;
}
