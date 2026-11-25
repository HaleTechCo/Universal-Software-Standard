//! Checking a project record against the standard and the project's files.

use std::collections::BTreeSet;

use crate::catalog::{Catalog, Diagnostic, FactSource, Level};
use crate::evaluate::components::walk;
use crate::evaluate::project::{ProjectFiles, SubProject};
use crate::evaluate::{applicable, resolve_facts};
use crate::record::Record;
use crate::source::Source;

/// Checks `r` against the catalog `c` and the project's files.
///
/// Returns one diagnostic per problem, each with `file` set to the record's
/// path and no line number. Component records are found with [`walk`] and
/// checked in the same way; their diagnostics carry their own path.
#[must_use]
pub fn check_record(
    src: &dyn Source,
    c: &Catalog,
    r: &Record,
    project: &dyn ProjectFiles,
) -> Vec<Diagnostic> {
    let walked = walk(src, r, project);
    let mut out = Vec::new();
    for found in &walked.records {
        if found.dir.is_empty() {
            check_own(c, &found.record, project, &mut out);
        } else {
            let sub = SubProject {
                inner: project,
                prefix: found.dir.clone(),
            };
            check_own(c, &found.record, &sub, &mut out);
        }
    }
    out.extend(walked.problems);
    out
}

/// Normalises a path from a record: forward slashes only, with `.` and
/// empty segments dropped.
pub(crate) fn normalise(path: &str) -> String {
    path.replace('\\', "/")
        .split('/')
        .filter(|s| !s.is_empty() && *s != ".")
        .collect::<Vec<_>>()
        .join("/")
}

/// Returns whether a path segment starts with a drive prefix such as `C:`.
fn is_drive(segment: &str) -> bool {
    let mut chars = segment.chars();
    matches!(
        (chars.next(), chars.next()),
        (Some(a), Some(':')) if a.is_ascii_alphabetic()
    )
}

/// Returns whether the raw record path `raw` is absolute, or has a segment
/// that is `..` or starts with a drive prefix.
pub(crate) fn leaves_project(raw: &str) -> bool {
    let path = raw.replace('\\', "/");
    path.starts_with('/') || path.split('/').any(|s| s == ".." || is_drive(s))
}

/// Checks facts, decisions, deviations and audits of `r`.
fn check_own(c: &Catalog, r: &Record, project: &dyn ProjectFiles, out: &mut Vec<Diagnostic>) {
    let file = r.path.as_str();
    let mut add = |message: String| out.push(Diagnostic::new(file, message));

    for key in r.facts.keys() {
        match c.facts.get(key).map(|f| &f.source) {
            None => add(format!("unknown fact \"{key}\"")),
            Some(FactSource::EnabledBy(rule)) => add(format!(
                "fact \"{key}\" is set by {rule}, not answered directly"
            )),
            Some(FactSource::Question(_)) => {}
        }
    }
    for (id, fact) in &c.facts {
        if matches!(fact.source, FactSource::Question(_)) && !r.facts.contains_key(id) {
            add(format!("fact \"{id}\" is not answered"));
        }
    }

    let facts = resolve_facts(c, r);
    let applies = |id: &str| {
        c.rule(id)
            .and_then(|rule| rule.active())
            .is_some_and(|a| a.applies.eval(&facts))
    };
    let level = |id: &str| c.rule(id).and_then(|rule| rule.active()).map(|a| a.level);

    for id in r.decisions.keys() {
        if c.rule(id).is_none() {
            add(format!("decision for unknown rule {id}"));
        } else if let Some(l) = level(id).filter(|l| *l != Level::May) {
            add(format!(
                "decision for {id}, a {l} rule; only MAY rules take decisions"
            ));
        } else if !applies(id) {
            add(format!("decision for {id}, which does not apply"));
        }
    }
    for rule in applicable(c, r) {
        let is_may = rule.active().is_some_and(|a| a.level == Level::May);
        if is_may && !r.decisions.contains_key(&rule.id) {
            add(format!("no decision for applicable MAY rule {}", rule.id));
        }
    }

    let mut seen = BTreeSet::new();
    for d in &r.deviations {
        let id = d.rule.as_str();
        if c.rule(id).is_none() {
            add(format!("deviation for unknown rule {id}"));
        } else if level(id) == Some(Level::May) {
            add(format!(
                "deviation for {id}, a MAY rule; MAY rules take decisions, not deviations"
            ));
        } else if !applies(id) {
            add(format!("deviation for {id}, which does not apply"));
        } else if !seen.insert(id) {
            add(format!("more than one deviation for {id}"));
        }
        if leaves_project(&d.document) {
            add(format!(
                "deviation document \"{}\" leaves the project",
                d.document
            ));
        } else if !project.exists(&normalise(&d.document)) {
            add(format!("deviation document \"{}\" not found", d.document));
        }
        if d.review_by
            .as_deref()
            .is_some_and(|review| review < d.date.as_str())
        {
            add(format!("review-by is before date for {id}"));
        }
    }

    for a in &r.audits {
        if leaves_project(&a.results) {
            add(format!(
                "audit results \"{}\" leaves the project",
                a.results
            ));
        } else if !project.exists(&normalise(&a.results)) {
            add(format!("audit results \"{}\" not found", a.results));
        }
        if leaves_project(&a.report) {
            add(format!("audit report \"{}\" leaves the project", a.report));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::load;
    use crate::evaluate::project::MemProject;
    use crate::record::parse_record;
    use crate::source::{Embedded, MemSource};
    use crate::testfixture::standard;

    const FACTS: &str = "facts: { has-ui: false, has-web-ui: false, uses-database: true }\n";
    const DECISIONS: &str = "decisions: { USS-EXT-001: { adopt: true } }\n";
    const DEVIATION: &str = concat!(
        "deviations:\n",
        "  - { rule: USS-SEC-002, document: docs/deviations/USS-SEC-002.md,",
        " approved-by: zac, date: 2026-01-01, review-by: 2027-01-01 }\n",
    );
    const DOC: &str = "docs/deviations/USS-SEC-002.md";
    const AUDIT_RESULTS: &str = ".uss/audits/a.yaml";
    const AUDIT_REPORT: &str = ".uss/audits/a.md";

    fn catalog() -> Catalog {
        load(&standard()).unwrap()
    }

    fn record_text(version: &str, body: &str) -> String {
        format!("standard: {{ version: {version} }}\nproject: {{ name: p }}\n{body}")
    }

    fn record(body: &str) -> Record {
        parse_record(
            &Embedded,
            ".uss/record.yaml",
            record_text("0.1.0", body).as_bytes(),
        )
        .unwrap()
    }

    fn project(files: &[(&str, &str)]) -> MemProject {
        MemProject(
            files
                .iter()
                .map(|(p, t)| ((*p).to_string(), t.as_bytes().to_vec()))
                .collect(),
        )
    }

    fn messages(c: &Catalog, r: &Record, p: &MemProject) -> Vec<String> {
        check_record(&Embedded, c, r, p)
            .into_iter()
            .map(|d| d.message)
            .collect()
    }

    fn valid_body() -> String {
        format!("{FACTS}{DECISIONS}{DEVIATION}")
    }

    #[test]
    fn check_valid() {
        let r = record(&valid_body());
        let found = check_record(&Embedded, &catalog(), &r, &project(&[(DOC, "doc")]));
        assert_eq!(found, Vec::<Diagnostic>::new());
    }

    #[test]
    #[expect(clippy::too_many_lines, reason = "one table row per check")]
    fn check_errors() {
        let all_facts = "has-ui: false, has-web-ui: false, uses-database: true";
        let cases: Vec<(String, &str)> = vec![
            (
                format!("facts: {{ {all_facts}, bogus: true }}\n{DECISIONS}{DEVIATION}"),
                "unknown fact \"bogus\"",
            ),
            (
                format!(
                    "facts: {{ {all_facts}, has-plugin-interface: true }}\n{DECISIONS}{DEVIATION}"
                ),
                "fact \"has-plugin-interface\" is set by USS-EXT-001, not answered directly",
            ),
            (
                format!(
                    "facts: {{ has-web-ui: false, uses-database: true }}\n{DECISIONS}{DEVIATION}"
                ),
                "fact \"has-ui\" is not answered",
            ),
            (
                format!(
                    "{FACTS}decisions: {{ USS-EXT-001: {{ adopt: true }}, USS-ZZZ-001: {{ adopt: true }} }}\n{DEVIATION}"
                ),
                "decision for unknown rule USS-ZZZ-001",
            ),
            (
                format!(
                    "{FACTS}decisions: {{ USS-EXT-001: {{ adopt: true }}, USS-SEC-001: {{ adopt: true }} }}\n{DEVIATION}"
                ),
                "decision for USS-SEC-001, a MUST rule; only MAY rules take decisions",
            ),
            (
                format!("{FACTS}{DEVIATION}"),
                "no decision for applicable MAY rule USS-EXT-001",
            ),
            (
                format!(
                    "{FACTS}{DECISIONS}deviations:\n  - {{ rule: USS-ZZZ-001, document: {DOC}, approved-by: z, date: 2026-01-01 }}\n"
                ),
                "deviation for unknown rule USS-ZZZ-001",
            ),
            (
                format!(
                    "{FACTS}{DECISIONS}deviations:\n  - {{ rule: USS-EXT-001, document: {DOC}, approved-by: z, date: 2026-01-01 }}\n"
                ),
                "deviation for USS-EXT-001, a MAY rule; MAY rules take decisions, not deviations",
            ),
            (
                format!(
                    "{FACTS}{DECISIONS}deviations:\n  - {{ rule: USS-SEC-003, document: {DOC}, approved-by: z, date: 2026-01-01 }}\n"
                ),
                "deviation for USS-SEC-003, which does not apply",
            ),
            (
                format!(
                    "{FACTS}{DECISIONS}{DEVIATION}{}",
                    DEVIATION.replace("deviations:\n", "")
                ),
                "more than one deviation for USS-SEC-002",
            ),
            (
                format!(
                    "{FACTS}{DECISIONS}deviations:\n  - {{ rule: USS-SEC-002, document: docs/missing.md, approved-by: z, date: 2026-01-01 }}\n"
                ),
                "deviation document \"docs/missing.md\" not found",
            ),
            (
                format!(
                    "{FACTS}{DECISIONS}deviations:\n  - {{ rule: USS-SEC-002, document: '/x.md', approved-by: z, date: 2026-01-01 }}\n"
                ),
                "deviation document \"/x.md\" leaves the project",
            ),
            (
                format!(
                    "{FACTS}{DECISIONS}deviations:\n  - {{ rule: USS-SEC-002, document: 'C:\\x.md', approved-by: z, date: 2026-01-01 }}\n"
                ),
                "deviation document \"C:\\x.md\" leaves the project",
            ),
            (
                format!(
                    "{FACTS}{DECISIONS}deviations:\n  - {{ rule: USS-SEC-002, document: '../x.md', approved-by: z, date: 2026-01-01 }}\n"
                ),
                "deviation document \"../x.md\" leaves the project",
            ),
            (
                format!(
                    "{FACTS}{DECISIONS}deviations:\n  - {{ rule: USS-SEC-002, document: {DOC}, approved-by: z, date: 2026-02-01, review-by: 2026-01-01 }}\n"
                ),
                "review-by is before date for USS-SEC-002",
            ),
            (
                format!(
                    "{}audits:\n  - {{ date: 2026-01-01, results: audits/r.yaml, report: audits/r.md }}\n",
                    valid_body()
                ),
                "audit results \"audits/r.yaml\" not found",
            ),
            (
                format!(
                    "{}audits:\n  - {{ date: 2026-01-01, results: '../r.yaml', report: {AUDIT_REPORT} }}\n",
                    valid_body()
                ),
                "audit results \"../r.yaml\" leaves the project",
            ),
            (
                format!(
                    "{}audits:\n  - {{ date: 2026-01-01, results: {AUDIT_RESULTS}, report: '/r.md' }}\n",
                    valid_body()
                ),
                "audit report \"/r.md\" leaves the project",
            ),
            (
                format!("{}components: [\"../x\"]\n", valid_body()),
                "component path \"../x\" leaves the project",
            ),
            (
                format!("{}components: [\"/x\"]\n", valid_body()),
                "component path \"/x\" leaves the project",
            ),
            (
                format!("{}components: [\"C:\\\\x\"]\n", valid_body()),
                "component path \"C:\\x\" leaves the project",
            ),
            (
                format!("{}components: [\"svc/C:x\"]\n", valid_body()),
                "component path \"svc/C:x\" leaves the project",
            ),
            (
                format!("{}components: [svc]\n", valid_body()),
                "component record \"svc/.uss/record.yaml\" not found",
            ),
        ];
        let c = catalog();
        let files = project(&[(DOC, "doc"), (AUDIT_RESULTS, "results")]);
        for (body, want) in cases {
            let r = record(&body);
            let got = messages(&c, &r, &files);
            assert_eq!(got, [want], "body: {body}");
        }
    }

    #[test]
    fn check_may_decision_not_applicable() {
        let mut src: MemSource = standard();
        let extra = "\n### USS-SEC-005: Optional thing\n\n- **Level:** MAY\n- **Applies when:** uses-database\n- **Category:** Safety\n- **Verify:** Check it.\n- **References:** none\n\nA project MAY do it.\n";
        let file = src.0.get_mut("standard/core/security.md").unwrap();
        file.extend_from_slice(extra.as_bytes());
        let c = load(&src).unwrap();
        let body = "facts: { has-ui: false, has-web-ui: false, uses-database: false }\ndecisions: { USS-EXT-001: { adopt: true }, USS-SEC-005: { adopt: true } }\n";
        let got = messages(&c, &record(body), &project(&[]));
        assert_eq!(got, ["decision for USS-SEC-005, which does not apply"]);
    }

    #[test]
    fn check_backslash_path() {
        let body = valid_body().replace(DOC, "docs\\deviations\\USS-SEC-002.md");
        let r = record(&body);
        assert!(r.deviations[0].document.contains('\\'));
        let found = check_record(&Embedded, &catalog(), &r, &project(&[(DOC, "doc")]));
        assert_eq!(found, Vec::<Diagnostic>::new());
    }

    #[test]
    fn check_dot_and_empty_segments() {
        let body = valid_body().replace(DOC, "./docs//deviations/USS-SEC-002.md");
        let r = record(&body);
        let found = check_record(&Embedded, &catalog(), &r, &project(&[(DOC, "doc")]));
        assert_eq!(found, Vec::<Diagnostic>::new());
    }

    #[test]
    fn check_escaping_document_leaves_the_project() {
        let body = valid_body().replace(DOC, "../x.md");
        let got = messages(&catalog(), &record(&body), &project(&[(DOC, "doc")]));
        assert_eq!(got, ["deviation document \"../x.md\" leaves the project"]);
    }

    fn component_body(components: &str) -> String {
        format!("{}components: {components}\n", valid_body())
    }

    #[test]
    fn check_components() {
        let child = record_text(
            "0.1.0",
            "facts: { has-web-ui: false, uses-database: false }\ndecisions: { USS-EXT-001: { adopt: false } }\n",
        );
        let r = record(&component_body("[svc]"));
        let p = project(&[(DOC, "doc"), ("svc/.uss/record.yaml", &child)]);
        let found = check_record(&Embedded, &catalog(), &r, &p);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].file, "svc/.uss/record.yaml");
        assert_eq!(found[0].message, "fact \"has-ui\" is not answered");
    }

    #[test]
    fn check_components_nested_and_documents() {
        let leaf = record_text(
            "0.1.0",
            "facts: { has-ui: false, has-web-ui: false, uses-database: true }\ndecisions: { USS-EXT-001: { adopt: false } }\ndeviations:\n  - { rule: USS-SEC-002, document: docs/d.md, approved-by: z, date: 2026-01-01 }\n",
        );
        let mid = record_text(
            "0.1.0",
            "facts: { has-ui: false, has-web-ui: false, uses-database: false }\ndecisions: { USS-EXT-001: { adopt: false } }\ncomponents: [b]\n",
        );
        let r = record(&component_body("[a]"));
        let c = catalog();

        let with_doc = project(&[
            (DOC, "doc"),
            ("a/.uss/record.yaml", &mid),
            ("a/b/.uss/record.yaml", &leaf),
            ("a/b/docs/d.md", "doc"),
        ]);
        assert_eq!(check_record(&Embedded, &c, &r, &with_doc), []);

        let without = project(&[
            (DOC, "doc"),
            ("a/.uss/record.yaml", &mid),
            ("a/b/.uss/record.yaml", &leaf),
        ]);
        let found = check_record(&Embedded, &c, &r, &without);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].file, "a/b/.uss/record.yaml");
        assert_eq!(
            found[0].message,
            "deviation document \"docs/d.md\" not found"
        );
    }

    #[test]
    fn check_components_version_and_parse_errors() {
        let c = catalog();
        let r = record(&component_body("[svc]"));
        let other = record_text("9.9.9", "facts: {}\n");
        let found = check_record(
            &Embedded,
            &c,
            &r,
            &project(&[(DOC, "doc"), ("svc/.uss/record.yaml", &other)]),
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].file, ".uss/record.yaml");
        assert_eq!(
            found[0].message,
            "component record \"svc/.uss/record.yaml\" pins USS 9.9.9; the root record pins USS 0.1.0"
        );

        let found = check_record(
            &Embedded,
            &c,
            &r,
            &project(&[(DOC, "doc"), ("svc/.uss/record.yaml", "not: [a record\n")]),
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().all(|d| d.file == "svc/.uss/record.yaml"));
    }

    #[test]
    fn check_components_own_directory() {
        let r = record(&component_body("[\".\"]"));
        let got = messages(&catalog(), &r, &project(&[(DOC, "doc")]));
        assert_eq!(got, ["component path \".\" is the record's own directory"]);
    }

    #[test]
    fn sub_project_prefixes_paths() {
        let p = project(&[("a/x.txt", "x")]);
        let sub = SubProject {
            inner: &p,
            prefix: "a".to_string(),
        };
        assert!(sub.exists("x.txt"));
        assert_eq!(sub.read("x.txt"), Some(b"x".to_vec()));
        assert!(!sub.exists("y.txt"));
        assert_eq!(sub.read("y.txt"), None);
    }
}
