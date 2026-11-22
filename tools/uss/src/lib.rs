//! Universal Software Standard (USS) validator library.

pub mod catalog;
#[cfg(feature = "cli")]
pub mod cli;
pub mod evaluate;
pub mod expr;
pub mod record;
pub mod release;
pub mod report;
pub mod source;
#[cfg(test)]
mod testfixture;
mod text;
pub mod yaml;
