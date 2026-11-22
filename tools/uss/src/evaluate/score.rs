//! Compliance status and category scores.

use std::collections::BTreeSet;

use crate::catalog::{Catalog, Diagnostic, Level, Rule};
use crate::record::{Deviation, Outcome, Record, Results, Suppression};

use super::applicable;

/// The outcome of one applicable rule.
#[derive(Debug, Clone)]
pub struct RuleOutcome<'c> {
    /// The rule.
    pub rule: &'c Rule,
    /// Its outcome, `Unverified` when the audit recorded none.
    pub outcome: Outcome,
    /// The evidence recorded for it.
    pub evidence: String,
}

/// The compliance status and the counts behind it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    /// True when every count is zero.
    pub compliant: bool,
    /// `MUST` and `MUST NOT` rules that failed.
    pub must_failures: usize,
    /// `MUST` and `MUST NOT` rules with a documented deviation.
    pub must_deviations: usize,
    /// `SHOULD` and `SHOULD NOT` rules that failed.
    pub should_failures: usize,
    /// Rules of any level that are unverified.
    pub unverified: usize,
    /// Applicable `MAY` rules without a recorded decision.
    pub missing_decisions: usize,
}

/// The score of one category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Score {
    /// The category's display name.
    pub category: String,
    /// Applicable rules that passed.
    pub pass: usize,
    /// Applicable rules.
    pub applicable: usize,
    /// `pass * 100 / applicable`, rounded down. `None` when nothing applies.
    pub percent: Option<u32>,
}

/// The result of evaluating a project against the catalog.
#[derive(Debug, Clone)]
pub struct Evaluation<'c> {
    /// The standard version audited.
    pub version: String,
    /// The audit date.
    pub date: String,
    /// The audited commit.
    pub commit: String,
    /// Who or what ran the audit.
    pub auditor: String,
    /// The compliance status.
    pub status: Status,
    /// One score per registry category, in registry order.
    pub scores: Vec<Score>,
    /// One outcome per applicable rule, sorted by ID.
    pub rules: Vec<RuleOutcome<'c>>,
    /// The suppressions the audit recorded.
    pub suppressions: Vec<Suppression>,
    /// The deviations the record lists.
    pub deviations: Vec<Deviation>,
}

/// Evaluates `r` against `c` using the audit `res`.
///
/// # Errors
///
/// Returns every consistency problem between the results, the record and the
/// catalog, as diagnostics on the results file.
pub fn evaluate<'c>(
    c: &'c Catalog,
    r: &Record,
    res: &Results,
) -> Result<Evaluation<'c>, Vec<Diagnostic>> {
    let rules = applicable(c, r);
    let diags = consistency(c, r, res, &rules);
    if !diags.is_empty() {
        return Err(diags);
    }

    let outcomes: Vec<RuleOutcome<'c>> = rules
        .iter()
        .map(|&rule| match res.results.get(&rule.id) {
            Some(e) => RuleOutcome {
                rule,
                outcome: e.result,
                evidence: e.evidence.clone(),
            },
            None => RuleOutcome {
                rule,
                outcome: Outcome::Unverified,
                evidence: "no result recorded".to_string(),
            },
        })
        .collect();

    let mut status = Status {
        compliant: false,
        must_failures: 0,
        must_deviations: 0,
        should_failures: 0,
        unverified: 0,
        missing_decisions: 0,
    };
    for o in &outcomes {
        let Some(level) = o.rule.active().map(|a| a.level) else {
            continue;
        };
        let strong = matches!(level, Level::Must | Level::MustNot);
        let weak = matches!(level, Level::Should | Level::ShouldNot);
        match o.outcome {
            Outcome::Fail if strong => status.must_failures += 1,
            Outcome::Deviation if strong => status.must_deviations += 1,
            Outcome::Fail if weak => status.should_failures += 1,
            Outcome::Unverified => status.unverified += 1,
            _ => {}
        }
        if level == Level::May && !r.decisions.contains_key(&o.rule.id) {
            status.missing_decisions += 1;
        }
    }
    status.compliant = status.must_failures == 0
        && status.must_deviations == 0
        && status.should_failures == 0
        && status.unverified == 0
        && status.missing_decisions == 0;

    let scores = c
        .registry
        .categories
        .iter()
        .map(|cat| {
            let in_cat = || {
                outcomes
                    .iter()
                    .filter(|o| o.rule.active().is_some_and(|a| a.category == cat.name))
            };
            let applicable = in_cat().count();
            let pass = in_cat().filter(|o| o.outcome == Outcome::Pass).count();
            let percent = (pass * 100)
                .checked_div(applicable)
                .and_then(|p| u32::try_from(p).ok());
            Score {
                category: cat.name.clone(),
                pass,
                applicable,
                percent,
            }
        })
        .collect();

    Ok(Evaluation {
        version: res.standard.version.clone(),
        date: res.date.clone(),
        commit: res.commit.clone(),
        auditor: res.auditor.clone(),
        status,
        scores,
        rules: outcomes,
        suppressions: res.suppressions.clone(),
        deviations: r.deviations.clone(),
    })
}

/// Checks the results against the record and the catalog.
fn consistency(c: &Catalog, r: &Record, res: &Results, rules: &[&Rule]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    let mut add = |message: String| {
        diags.push(Diagnostic {
            file: res.path.clone(),
            line: 0,
            message,
        });
    };
    if res.standard.version != r.standard.version {
        add(format!(
            "results are for USS {} but the record is for USS {}",
            res.standard.version, r.standard.version
        ));
    }
    let applies: BTreeSet<&str> = rules.iter().map(|rule| rule.id.as_str()).collect();
    for (id, entry) in &res.results {
        let Some(rule) = c.rule(id) else {
            add(format!("{id}: unknown rule"));
            continue;
        };
        let Some(level) = rule.active().map(|a| a.level) else {
            add(format!("{id}: retired rule"));
            continue;
        };
        if !applies.contains(id.as_str()) {
            add(format!("{id}: rule does not apply to this project"));
            continue;
        }
        let out = entry.result;
        let adopt = r.decisions.get(id).map(|d| d.adopt);
        match out {
            Outcome::NotApplicable => {
                add(format!("{id}: not-applicable, but the rule applies"));
            }
            Outcome::Deviation if level == Level::May => {
                add(format!("{id}: a MAY rule cannot have a deviation"));
            }
            Outcome::Deviation if !r.deviations.iter().any(|d| &d.rule == id) => {
                add(format!("{id}: deviation without a deviation in the record"));
            }
            Outcome::Declined if level != Level::May => {
                add(format!("{id}: only a MAY rule can be declined"));
            }
            Outcome::Declined if adopt != Some(false) => {
                add(format!(
                    "{id}: declined without `adopt: false` in the record"
                ));
            }
            _ => {}
        }
        if level == Level::May && adopt == Some(false) && out != Outcome::Declined {
            add(format!(
                "{id}: the record declines this MAY rule, so the result must be declined"
            ));
        }
    }
    diags
}

fn count(n: usize, singular: &str, plural: &str) -> String {
    format!("{n} {}", if n == 1 { singular } else { plural })
}

impl Evaluation<'_> {
    /// The one-sentence compliance statement.
    #[must_use]
    pub fn statement(&self) -> String {
        let s = &self.status;
        if s.compliant {
            return format!("Compliant with USS {} as of {}.", self.version, self.date);
        }
        let parts: Vec<String> = [
            (s.must_failures, "MUST failure", "MUST failures"),
            (
                s.must_deviations,
                "documented MUST deviation",
                "documented MUST deviations",
            ),
            (s.should_failures, "SHOULD failure", "SHOULD failures"),
            (s.unverified, "unverified rule", "unverified rules"),
            (
                s.missing_decisions,
                "missing MAY decision",
                "missing MAY decisions",
            ),
        ]
        .into_iter()
        .filter(|(n, _, _)| *n > 0)
        .map(|(n, one, many)| count(n, one, many))
        .collect();
        format!(
            "Not compliant with USS {} as of {}: {}.",
            self.version,
            self.date,
            parts.join(", ")
        )
    }

    /// The per-category score line.
    #[must_use]
    pub fn score_line(&self) -> String {
        let parts: Vec<String> = self
            .scores
            .iter()
            .map(|s| match s.percent {
                Some(p) => format!("{} {p}%", s.category),
                None => format!("{} n/a", s.category),
            })
            .collect();
        format!("{}.", parts.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::load;
    use crate::record::{Entry, StandardRef, parse_record};
    use crate::source::Embedded;
    use crate::testfixture::standard;

    fn catalog() -> Catalog {
        load(&standard()).unwrap()
    }

    fn record(extra: &str) -> Record {
        let yaml = format!("standard: {{ version: 0.1.0 }}\nproject: {{ name: p }}\n{extra}");
        parse_record(&Embedded, "r.yaml", yaml.as_bytes()).unwrap()
    }

    fn results(entries: &[(&str, Outcome)]) -> Results {
        Results {
            standard: StandardRef {
                version: "0.1.0".to_string(),
            },
            commit: "a".repeat(40),
            date: "2026-11-21".to_string(),
            auditor: "tester".to_string(),
            results: entries
                .iter()
                .map(|(id, result)| {
                    (
                        (*id).to_string(),
                        Entry {
                            result: *result,
                            evidence: "e".to_string(),
                        },
                    )
                })
                .collect(),
            suppressions: Vec::new(),
            path: "res.yaml".to_string(),
        }
    }

    const DECLINE_EXT: &str = "decisions: { USS-EXT-001: { adopt: false } }\n";
    const ADOPT_EXT: &str = "decisions: { USS-EXT-001: { adopt: true } }\n";
    const DEVIATION: &str = "deviations:\n  - { rule: USS-SEC-001, document: d.md, approved-by: a, date: 2026-11-01 }\n";

    #[test]
    fn compliant_with_declined_may() {
        let c = catalog();
        let r = record(&format!("facts: {{}}\n{DECLINE_EXT}"));
        let res = results(&[
            ("USS-SEC-001", Outcome::Pass),
            ("USS-EXT-001", Outcome::Declined),
        ]);
        let e = evaluate(&c, &r, &res).unwrap();
        assert_eq!(e.statement(), "Compliant with USS 0.1.0 as of 2026-11-21.");
        assert_eq!(e.score_line(), "Safety 100%, Features 0%.");
    }

    #[test]
    fn not_compliant_parts() {
        let c = catalog();
        let r = record(&format!("facts: {{ uses-database: true }}\n{DEVIATION}"));
        let res = results(&[
            ("USS-SEC-001", Outcome::Deviation),
            ("USS-SEC-002", Outcome::Fail),
        ]);
        let e = evaluate(&c, &r, &res).unwrap();
        assert_eq!(
            e.statement(),
            "Not compliant with USS 0.1.0 as of 2026-11-21: 1 documented MUST deviation, 1 SHOULD failure, 1 unverified rule, 1 missing MAY decision."
        );
    }

    #[test]
    fn plural_parts() {
        let c = catalog();
        let r = record(&format!("facts: {{ has-ui: true }}\n{DECLINE_EXT}"));
        let res = results(&[
            ("USS-SEC-001", Outcome::Fail),
            ("USS-SEC-003", Outcome::Fail),
            ("USS-EXT-001", Outcome::Declined),
        ]);
        let e = evaluate(&c, &r, &res).unwrap();
        assert_eq!(
            e.statement(),
            "Not compliant with USS 0.1.0 as of 2026-11-21: 2 MUST failures."
        );
    }

    #[test]
    fn floor_rounding() {
        let c = catalog();
        let r = record(&format!(
            "facts: {{ uses-database: true, has-ui: true }}\n{DECLINE_EXT}"
        ));
        let res = results(&[
            ("USS-SEC-001", Outcome::Pass),
            ("USS-SEC-002", Outcome::Pass),
            ("USS-SEC-003", Outcome::Fail),
            ("USS-EXT-001", Outcome::Declined),
        ]);
        let e = evaluate(&c, &r, &res).unwrap();
        assert_eq!(e.scores[0].category, "Safety");
        assert_eq!(e.scores[0].percent, Some(66));
        assert!(e.statement().contains("1 MUST failure"));
    }

    #[test]
    fn score_is_na_when_nothing_applies() {
        let mut c = catalog();
        c.registry.categories.push(crate::catalog::Category {
            id: "fairness".to_string(),
            name: "Fairness".to_string(),
        });
        let r = record(&format!("facts: {{}}\n{DECLINE_EXT}"));
        let e = evaluate(&c, &r, &results(&[])).unwrap();
        assert_eq!(e.scores[2].percent, None);
        assert_eq!(e.score_line(), "Safety 0%, Features 0%, Fairness n/a.");
    }

    #[test]
    fn missing_result_is_unverified() {
        let c = catalog();
        let r = record(&format!("facts: {{}}\n{DECLINE_EXT}"));
        let e = evaluate(&c, &r, &results(&[("USS-EXT-001", Outcome::Declined)])).unwrap();
        let o = e.rules.iter().find(|o| o.rule.id == "USS-SEC-001").unwrap();
        assert_eq!(o.outcome, Outcome::Unverified);
        assert_eq!(o.evidence, "no result recorded");
        assert_eq!(e.status.unverified, 1);
        assert!(!e.status.compliant);
    }

    #[test]
    fn consistency_errors() {
        let c = catalog();
        let cases: [(&str, &str, Outcome, &str); 9] = [
            (
                "USS-NOPE-001",
                "",
                Outcome::Pass,
                "USS-NOPE-001: unknown rule",
            ),
            (
                "USS-SEC-004",
                "",
                Outcome::Pass,
                "USS-SEC-004: retired rule",
            ),
            (
                "USS-SEC-002",
                "",
                Outcome::Pass,
                "USS-SEC-002: rule does not apply",
            ),
            (
                "USS-SEC-001",
                "",
                Outcome::NotApplicable,
                "USS-SEC-001: not-applicable, but the rule applies",
            ),
            (
                "USS-EXT-001",
                ADOPT_EXT,
                Outcome::Deviation,
                "USS-EXT-001: a MAY rule cannot have a deviation",
            ),
            (
                "USS-SEC-001",
                "",
                Outcome::Deviation,
                "USS-SEC-001: deviation without a deviation in the record",
            ),
            (
                "USS-SEC-001",
                "",
                Outcome::Declined,
                "USS-SEC-001: only a MAY rule can be declined",
            ),
            (
                "USS-EXT-001",
                ADOPT_EXT,
                Outcome::Declined,
                "USS-EXT-001: declined without `adopt: false`",
            ),
            (
                "USS-EXT-001",
                DECLINE_EXT,
                Outcome::Pass,
                "USS-EXT-001: the record declines this MAY rule",
            ),
        ];
        for (id, extra, outcome, want) in cases {
            let r = record(&format!("facts: {{}}\n{extra}"));
            let diags = evaluate(&c, &r, &results(&[(id, outcome)])).unwrap_err();
            assert!(
                diags.iter().any(|d| d.message.contains(want)),
                "{want}: got {diags:?}"
            );
            assert!(diags.iter().all(|d| d.file == "res.yaml" && d.line == 0));
        }
    }

    #[test]
    fn consistency_version_mismatch_and_collects_all() {
        let c = catalog();
        let r = record("facts: {}\n");
        let mut res = results(&[
            ("USS-NOPE-001", Outcome::Pass),
            ("USS-SEC-004", Outcome::Pass),
        ]);
        res.standard.version = "0.2.0".to_string();
        let diags = evaluate(&c, &r, &res).unwrap_err();
        assert_eq!(diags.len(), 3);
        assert!(diags[0].message.contains("0.2.0") && diags[0].message.contains("0.1.0"));
    }
}
