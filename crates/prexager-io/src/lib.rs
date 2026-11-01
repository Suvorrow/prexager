//! I/O boundary for Prexager.
//!
//! This crate is the ONLY place in the workspace that depends on `cooler-rs`.
//! It converts third-party data into `prexager-core` types; no cooler-rs type
//! escapes this crate.

pub mod cool;

// Re-export the public API for convenience
pub use cool::{CoolError, load_cool_metadata};
