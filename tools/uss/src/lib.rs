//! Universal Software Standard (USS) validator library.

// Needed by later modules (schema validation); until then only tests use it.
use serde_json as _;

pub mod catalog;
#[cfg(feature = "cli")]
pub mod cli;
pub mod expr;
pub mod source;
pub mod yaml;
