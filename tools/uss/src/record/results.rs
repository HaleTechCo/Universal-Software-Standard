//! Audit results (`.uss/audits/<date>.yaml`).

use std::collections::BTreeMap;
use std::fmt;

use serde::Deserialize;

use super::{LoadError, StandardRef, load};
use crate::source::Source;

const RESULTS_SCHEMA: &str = "schema/audit-results.schema.json";

/// The outcome of one rule in an audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Outcome {
    /// The rule is met.
    Pass,
    /// The rule is not met.
    Fail,
    /// The rule is not met, under an approved deviation.
    Deviation,
    /// An optional rule the project decided not to adopt.
    Declined,
    /// The auditor could not decide.
    Unverified,
    /// The rule does not apply to the project.
    NotApplicable,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Deviation => "deviation",
            Self::Declined => "declined",
            Self::Unverified => "unverified",
            Self::NotApplicable => "not-applicable",
        })
    }
}

/// The result and evidence for one rule.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Entry {
    /// The outcome.
    pub result: Outcome,
    /// Evidence, such as `file:line`, a config key or a CI job.
    #[serde(default)]
    pub evidence: String,
}

/// A recorded suppression of a finding.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Suppression {
    /// The file the suppression is in.
    pub file: String,
    /// The line, counted from 1.
    pub line: Option<u32>,
    /// The rule ID.
    pub rule: String,
    /// Why the finding is suppressed.
    pub reason: String,
}

/// The results of one audit.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Results {
    /// The standard version audited against.
    pub standard: StandardRef,
    /// The full 40-character SHA of the audited commit.
    pub commit: String,
    /// Audit date, `YYYY-MM-DD`.
    pub date: String,
    /// Who or what ran the audit.
    #[serde(default)]
    pub auditor: String,
    /// Result per rule ID.
    pub results: BTreeMap<String, Entry>,
    /// Recorded suppressions.
    #[serde(default)]
    pub suppressions: Vec<Suppression>,
    /// Where the results were read from. Set by the parser.
    #[serde(skip)]
    pub path: String,
}

/// Parses and validates an audit results file.
///
/// # Errors
///
/// Returns a [`LoadError`] when the YAML is invalid, the schema is missing
/// from `src`, or the file does not match `schema/audit-results.schema.json`.
pub fn parse_results(src: &dyn Source, path: &str, data: &[u8]) -> Result<Results, LoadError> {
    let mut results: Results = load(src, path, data, RESULTS_SCHEMA)?;
    results.path = path.to_string();
    Ok(results)
}
