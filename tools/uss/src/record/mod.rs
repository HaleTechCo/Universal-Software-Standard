//! The project record (`.uss/record.yaml`) and its parser.
//!
//! Both files an adopting project keeps are checked against a JSON Schema
//! read from the USS [`Source`] before they are turned into typed values, so
//! every problem names the place in the file where it was found.

pub mod results;

use std::collections::BTreeMap;
use std::fmt;

use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::source::Source;
use crate::yaml;

pub use results::{Entry, Outcome, Results, Suppression, parse_results};

const RECORD_SCHEMA: &str = "schema/record.schema.json";

/// The version of the standard a project follows.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct StandardRef {
    /// A semantic version, for example `1.0.0`.
    pub version: String,
}

/// The project a record describes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Project {
    /// The project's name.
    pub name: String,
}

/// The languages, frameworks, platforms and tools a project uses.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", default)]
pub struct Stack {
    /// Programming languages.
    pub languages: Vec<String>,
    /// Frameworks.
    pub frameworks: Vec<String>,
    /// Platforms, such as databases and runtimes.
    pub platforms: Vec<String>,
    /// Other tools.
    pub tools: Vec<String>,
}

/// The decision on one optional (MAY) rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Decision {
    /// Whether the project adopts the rule.
    pub adopt: bool,
}

/// An approved deviation from a rule.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Deviation {
    /// The rule ID.
    pub rule: String,
    /// Path of the deviation document, relative to the project root.
    pub document: String,
    /// Who approved the deviation.
    pub approved_by: String,
    /// Approval date, `YYYY-MM-DD`.
    pub date: String,
    /// Date the deviation is due for review, `YYYY-MM-DD`.
    pub review_by: Option<String>,
}

/// A reference to one recorded audit.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct AuditRef {
    /// Audit date, `YYYY-MM-DD`.
    pub date: String,
    /// Path of the audit results file.
    pub results: String,
    /// Path of the generated report.
    pub report: String,
}

/// The inputs to compliance for one independently released unit.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Record {
    /// The standard version followed.
    pub standard: StandardRef,
    /// The project.
    pub project: Project,
    /// The stack.
    #[serde(default)]
    pub stack: Stack,
    /// Applicability facts, by fact key.
    pub facts: BTreeMap<String, bool>,
    /// Decisions on MAY rules, by rule ID.
    #[serde(default)]
    pub decisions: BTreeMap<String, Decision>,
    /// Approved deviations.
    #[serde(default)]
    pub deviations: Vec<Deviation>,
    /// Recorded audits.
    #[serde(default)]
    pub audits: Vec<AuditRef>,
    /// Paths of child components, each with its own record.
    #[serde(default)]
    pub components: Vec<String>,
    /// Where the record was read from. Set by the parser.
    #[serde(skip)]
    pub path: String,
}

impl Record {
    /// Returns the audit with the greatest date. On a tie, the later entry.
    #[must_use]
    pub fn latest_audit(&self) -> Option<&AuditRef> {
        self.audits
            .iter()
            .enumerate()
            .max_by(|(i, a), (j, b)| a.date.cmp(&b.date).then(i.cmp(j)))
            .map(|(_, a)| a)
    }
}

/// A file that could not be loaded, with every problem found in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadError {
    /// The file's path.
    pub path: String,
    /// One line per problem: `<JSON pointer>: <message>`.
    pub problems: Vec<String>,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.problems.join("; "))
    }
}

impl std::error::Error for LoadError {}

/// Parses and validates a project record.
///
/// # Errors
///
/// Returns a [`LoadError`] when the YAML is invalid, the schema is missing
/// from `src`, or the record does not match `schema/record.schema.json`.
pub fn parse_record(src: &dyn Source, path: &str, data: &[u8]) -> Result<Record, LoadError> {
    let mut record: Record = load(src, path, data, RECORD_SCHEMA)?;
    record.path = path.to_string();
    Ok(record)
}

/// Parses `data`, validates it against the schema at `schema_path` in `src`,
/// then converts it to `T`.
fn load<T: DeserializeOwned>(
    src: &dyn Source,
    path: &str,
    data: &[u8],
    schema_path: &str,
) -> Result<T, LoadError> {
    let fail = |problem: String| LoadError {
        path: path.to_string(),
        problems: vec![problem],
    };
    let value: serde_json::Value = yaml::from_slice(data).map_err(|e| fail(format!("/: {e}")))?;
    let schema_bytes = src
        .read(schema_path)
        .ok_or_else(|| fail(format!("/: schema {schema_path} not found")))?;
    let schema: serde_json::Value = serde_json::from_slice(&schema_bytes)
        .map_err(|e| fail(format!("/: schema {schema_path} is not valid JSON: {e}")))?;
    let validator = jsonschema::draft202012::new(&schema)
        .map_err(|e| fail(format!("/: schema {schema_path} is invalid: {e}")))?;
    let mut problems: Vec<String> = validator
        .iter_errors(&value)
        .map(|e| {
            let at = e.instance_path().to_string();
            format!("{}: {e}", if at.is_empty() { "/" } else { &at })
        })
        .collect();
    if !problems.is_empty() {
        problems.sort();
        problems.dedup();
        return Err(LoadError {
            path: path.to_string(),
            problems,
        });
    }
    serde_json::from_value(value).map_err(|e| fail(format!("/: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::Embedded;

    const RECORD: &str = include_str!("testdata/record-valid.yaml");
    const RESULTS: &str = include_str!("testdata/results-valid.yaml");

    fn record_err(text: &str) -> LoadError {
        parse_record(&Embedded, ".uss/record.yaml", text.as_bytes()).unwrap_err()
    }

    #[test]
    fn record_valid() {
        let r = parse_record(&Embedded, ".uss/record.yaml", RECORD.as_bytes()).unwrap();
        assert_eq!(r.standard.version, "1.0.0");
        assert_eq!(r.project.name, "example-service");
        assert_eq!(r.stack.languages, ["go", "typescript"]);
        assert_eq!(r.stack.frameworks, ["react"]);
        assert_eq!(r.stack.tools, Vec::<String>::new());
        assert!(r.facts["has-web-ui"]);
        assert!(!r.facts["requires-fips"]);
        assert!(r.decisions["USS-DST-004"].adopt);
        assert!(!r.decisions["USS-EXT-001"].adopt);
        assert_eq!(r.deviations.len(), 1);
        assert_eq!(r.deviations[0].rule, "USS-SUP-009");
        assert_eq!(r.deviations[0].approved_by, "Jane Doe, CTO");
        assert_eq!(r.deviations[0].date, "2026-11-21");
        assert_eq!(r.deviations[0].review_by.as_deref(), Some("2027-05-21"));
        assert_eq!(r.audits[0].results, ".uss/audits/2026-11-21.yaml");
        assert_eq!(r.components, ["libs/core", "cli"]);
        assert_eq!(r.path, ".uss/record.yaml");
    }

    #[test]
    fn record_crlf_and_bom() {
        let mut data = b"\xEF\xBB\xBF".to_vec();
        data.extend_from_slice(RECORD.replace('\n', "\r\n").as_bytes());
        let a = parse_record(&Embedded, "r.yaml", RECORD.as_bytes()).unwrap();
        let b = parse_record(&Embedded, "r.yaml", &data).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn record_schema_errors() {
        let cases = [
            ("colour: red\n", "", "colour"),
            (
                "  requires-fips: false\n",
                "  has-ui: \"yes\"\n",
                "/facts/has-ui",
            ),
            ("USS-EXT-001:", "SEC-1:", "/decisions"),
            ("    approved-by: \"Jane Doe, CTO\"\n", "", "/deviations/0"),
            (
                "    date: 2026-11-21\n    review-by",
                "    date: 21/11/2026\n    review-by",
                "/deviations/0/date",
            ),
        ];
        for (i, (find, add, want)) in cases.iter().enumerate() {
            let text = if i == 0 {
                format!("{RECORD}{find}")
            } else if add.starts_with("  has-ui") {
                RECORD.replace(find, &format!("{find}{add}"))
            } else {
                RECORD.replace(find, add)
            };
            let err = record_err(&text);
            assert!(
                err.problems.iter().any(|p| p.contains(want)),
                "{want}: {err}"
            );
            assert!(err.problems.iter().all(|p| !p.contains('\n')), "{err}");
        }
    }

    #[test]
    fn syntax_error_is_one_problem() {
        let err = record_err("standard: [\n");
        assert_eq!(err.problems.len(), 1, "{err}");
        assert!(err.to_string().starts_with(".uss/record.yaml: "), "{err}");
    }

    #[test]
    fn missing_schema_is_load_error() {
        let src = crate::source::MemSource::default();
        let err = parse_record(&src, "r.yaml", RECORD.as_bytes()).unwrap_err();
        assert!(err.problems[0].contains("record.schema.json"), "{err}");
    }

    #[test]
    fn results_valid() {
        let r = parse_results(&Embedded, ".uss/audits/x.yaml", RESULTS.as_bytes()).unwrap();
        assert_eq!(r.commit.len(), 40);
        assert_eq!(r.date, "2026-11-21");
        assert_eq!(r.auditor, "claude");
        assert_eq!(r.results["USS-SEC-001"].result, Outcome::Pass);
        assert_eq!(r.results["USS-SEC-001"].evidence, "src/main.rs:12");
        assert_eq!(r.results["USS-SEC-002"].result, Outcome::NotApplicable);
        assert_eq!(r.results["USS-SEC-002"].evidence, "");
        assert_eq!(r.suppressions[0].line, Some(12));
        assert_eq!(Outcome::NotApplicable.to_string(), "not-applicable");
        assert_eq!(Outcome::Unverified.to_string(), "unverified");
    }

    #[test]
    fn results_errors() {
        let cases = [
            (
                "commit: 0123456789abcdef0123456789abcdef01234567",
                "commit: 1ab35de",
                "/commit",
            ),
            (
                "result: pass",
                "result: passed",
                "/results/USS-SEC-001/result",
            ),
            ("    evidence: src/main.rs:12\n", "", "/results/USS-SEC-001"),
        ];
        for (find, rep, want) in cases {
            let err = parse_results(&Embedded, "x.yaml", RESULTS.replace(find, rep).as_bytes())
                .unwrap_err();
            assert!(
                err.problems.iter().any(|p| p.contains(want)),
                "{want}: {err}"
            );
        }
    }

    #[test]
    fn results_suppression_rule_is_the_tools_own() {
        let find = "rule: USS-SEC-001";
        assert!(RESULTS.contains(find), "test data lacks {find:?}");
        let tool = RESULTS.replace(find, "rule: clippy::unwrap_used");
        let r = parse_results(&Embedded, "x.yaml", tool.as_bytes()).unwrap();
        assert_eq!(r.suppressions[0].rule, "clippy::unwrap_used");

        let empty = RESULTS.replace(find, "rule: \"\"");
        let err = parse_results(&Embedded, "x.yaml", empty.as_bytes()).unwrap_err();
        assert!(
            err.problems
                .iter()
                .any(|p| p.contains("/suppressions/0/rule")),
            "{err}"
        );
    }

    #[test]
    fn record_template_matches_standard() {
        use std::collections::BTreeSet;

        use crate::catalog::{self, FactSource};
        use crate::source::Source;

        const TEMPLATE: &str = "templates/record.yaml";
        let data = Embedded
            .read(TEMPLATE)
            .expect("the record template is embedded");
        let r = parse_record(&Embedded, TEMPLATE, &data).unwrap();
        let c = catalog::load(&Embedded).unwrap();
        assert_eq!(r.standard.version, c.registry.version);
        let asked: BTreeSet<&str> = c
            .facts
            .values()
            .filter(|f| matches!(f.source, FactSource::Question(_)))
            .map(|f| f.id.as_str())
            .collect();
        let keys: BTreeSet<&str> = r.facts.keys().map(String::as_str).collect();
        assert_eq!(keys, asked);
    }

    #[test]
    fn latest_audit_tie_takes_later() {
        let audit = |date: &str, results: &str| AuditRef {
            date: date.into(),
            results: results.into(),
            report: String::new(),
        };
        let mut r = parse_record(&Embedded, "r.yaml", RECORD.as_bytes()).unwrap();
        r.audits = vec![
            audit("2026-11-21", "a"),
            audit("2026-12-01", "b"),
            audit("2026-12-01", "c"),
            audit("2026-01-01", "d"),
        ];
        assert_eq!(r.latest_audit().unwrap().results, "c");
        r.audits.clear();
        assert!(r.latest_audit().is_none());
    }
}
