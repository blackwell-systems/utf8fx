//! Component handlers for native component expansion
//!
//! Each handler module implements the logic for expanding a specific
//! native component type into a Primitive or ComponentOutput.

pub mod donut;
pub mod gauge;
#[cfg(feature = "fetch")]
pub mod github;
pub mod license;
pub mod progress;
pub mod rating;
pub mod row;
pub mod sparkline;
pub mod swatch;
pub mod tech;
pub mod tech_group;
pub mod version;
pub mod waveform;

#[cfg(feature = "fetch")]
pub use github::FetchContext;
