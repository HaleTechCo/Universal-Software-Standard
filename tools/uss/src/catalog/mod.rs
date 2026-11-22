//! The catalog of the standard: registry, rules and related data.

mod diag;
mod facts;
pub(crate) mod markdown;
pub(crate) mod registry;
mod rules;

pub use diag::Diagnostic;
pub use facts::{Fact, FactSource};
pub use registry::{Category, Registry, load_registry};
pub use rules::{Active, FileKind, Level, Rule, RuleFile, RuleState};
