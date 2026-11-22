//! The catalog of the standard: registry, rules and related data.

mod diag;
pub(crate) mod registry;

pub use diag::Diagnostic;
pub use registry::{Category, Registry, load_registry};
