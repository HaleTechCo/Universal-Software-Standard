//! Universal Software Standard (USS) validator library.

pub mod catalog;
#[cfg(feature = "cli")]
pub mod cli;
pub mod evaluate;
pub mod expr;
pub mod record;
pub mod report;
pub mod source;
#[cfg(test)]
mod testfixture;
pub mod yaml;
