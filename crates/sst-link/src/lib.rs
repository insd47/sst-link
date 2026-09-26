//! Typed SST resource links for Rust.
//!
//! Declare one struct per function that mirrors its `link` array, derive `Deserialize` and [`Links`], and load it once
//! at startup. Every field is required, so a missing link fails at cold start instead of halfway through a request.
//!
//! ```no_run
//! use serde::Deserialize;
//! use sst_link::{Bucket, Links, Secret};
//!
//! #[derive(Deserialize, Links)]
//! #[serde(rename_all = "PascalCase")]
//! struct Resources {
//!     key: Secret,                       // link "Key"
//!     #[serde(rename = "RouterStorage")] // a link whose name doesn't follow the field
//!     storage: Bucket,
//! }
//!
//! let resources = Resources::load()?;
//! # Ok::<(), sst_link::Error>(())
//! ```
//!
//! Links are keyed by their SST names, and serde does the mapping, so everything serde offers (`rename_all`, `rename`,
//! `default`, `flatten`) works as usual. The shapes of SST's own components ship with this crate; your own
//! `sst.Linkable`s bring their own `Deserialize` structs.

pub use error::Error;
pub use links::Links;
pub use sst_link_derive::Links;
pub use types::{App, Bucket, Dynamo, Function, Realtime, Router, Secret};

mod error;
mod links;
mod types;
