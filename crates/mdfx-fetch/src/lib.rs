//! mdfx-fetch: Data fetching for dynamic badges
//!
//! This crate provides the infrastructure for fetching live data from external APIs
//! (GitHub, npm, crates.io, etc.) to power dynamic badges in mdfx.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
//! │  Fetcher    │───▶│   Cache     │───▶│ DataSource  │
//! │  (facade)   │    │  (disk)     │    │  (API)      │
//! └─────────────┘    └─────────────┘    └─────────────┘
//! ```
//!
//! # Example
//!
//! ```rust,no_run
//! use mdfx_fetch::{Fetcher, FetchConfig, sources::GitHubSource};
//!
//! let config = FetchConfig::default();
//! let fetcher = Fetcher::new(config).unwrap();
//!
//! // Fetch GitHub stars
//! let stars = fetcher.fetch("github", "rust-lang/rust", "stars").unwrap();
//! println!("Rust has {} stars", stars);
//! ```

pub mod cache;
pub mod error;
pub mod fetcher;
pub mod sources;
pub mod value;

pub use cache::{Cache, CacheConfig};
pub use error::{FetchError, Result};
pub use fetcher::{FetchConfig, Fetcher};
pub use sources::DataSource;
pub use value::DataValue;
