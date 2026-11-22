//! Renders the Markdown audit report from an evaluation.

use minijinja::value::Serde;
use minijinja::{AutoEscape, Environment};
use serde::Serialize;

use crate::catalog::Level;
use crate::evaluate::Evaluation;
use crate::record::Outcome;
use crate::source::Source;

/// Where the report template lives in the standard.
const TEMPLATE: &str = "templates/audit-report.md.j2";

/// The outcome sections, in report order, with their headings.
const SECTIONS: [(Outcome, &str); 5] = [
    (Outcome::Fail, "Fail"),
    (Outcome::Unverified, "Unverified"),
    (Outcome::Deviation, "Deviation"),
    (Outcome::Declined, "Declined"),
    (Outcome::Pass, "Pass"),
];

/// What the template sees.
#[derive(Serialize)]
struct View {
    project: String,
    statement: String,
    commit: String,
    date: String,
    auditor: String,
    scores: Vec<ScoreRow>,
    sections: Vec<Section>,
    deviations: Vec<DeviationRow>,
    suppressions: Vec<SuppressionRow>,
}

#[derive(Serialize)]
struct ScoreRow {
    category: String,
    percent: Option<u32>,
    pass: usize,
    applicable: usize,
}

/// One outcome section; only built when it has rows.
#[derive(Serialize)]
struct Section {
    heading: &'static str,
    rows: Vec<RuleRow>,
}

#[derive(Serialize)]
struct RuleRow {
    id: String,
    level: String,
    category: String,
    title: String,
    evidence: String,
}

#[derive(Serialize)]
struct DeviationRow {
    rule: String,
    document: String,
    approved_by: String,
    date: String,
    review_by: String,
}

#[derive(Serialize)]
struct SuppressionRow {
    file: String,
    line: String,
    rule: String,
    reason: String,
}

/// Sort position of a level: `MUST` first, `MAY` last.
fn level_rank(level: Level) -> u8 {
    match level {
        Level::Must => 0,
        Level::MustNot => 1,
        Level::Should => 2,
        Level::ShouldNot => 3,
        Level::May => 4,
    }
}

fn view(project: &str, ev: &Evaluation) -> View {
    let sections = SECTIONS
        .iter()
        .filter_map(|&(outcome, heading)| {
            let mut rows: Vec<(u8, RuleRow)> = ev
                .rules
                .iter()
                .filter(|o| o.outcome == outcome)
                .filter_map(|o| {
                    let active = o.rule.active()?;
                    Some((
                        level_rank(active.level),
                        RuleRow {
                            id: o.rule.id.clone(),
                            level: active.level.to_string(),
                            category: active.category.clone(),
                            title: o.rule.title.clone(),
                            evidence: o.evidence.clone(),
                        },
                    ))
                })
                .collect();
            rows.sort_by(|a, b| (a.0, &a.1.id).cmp(&(b.0, &b.1.id)));
            (!rows.is_empty()).then(|| Section {
                heading,
                rows: rows.into_iter().map(|(_, row)| row).collect(),
            })
        })
        .collect();
    View {
        project: project.to_string(),
        statement: ev.statement(),
        commit: ev.commit.clone(),
        date: ev.date.clone(),
        auditor: ev.auditor.clone(),
        scores: ev
            .scores
            .iter()
            .map(|s| ScoreRow {
                category: s.category.clone(),
                percent: s.percent,
                pass: s.pass,
                applicable: s.applicable,
            })
            .collect(),
        sections,
        deviations: ev
            .deviations
            .iter()
            .map(|d| DeviationRow {
                rule: d.rule.clone(),
                document: d.document.clone(),
                approved_by: d.approved_by.clone(),
                date: d.date.clone(),
                review_by: d.review_by.clone().unwrap_or_default(),
            })
            .collect(),
        suppressions: ev
            .suppressions
            .iter()
            .map(|s| SuppressionRow {
                file: s.file.clone(),
                line: s.line.map_or_default(|n| n.to_string()),
                rule: s.rule.clone(),
                reason: s.reason.clone(),
            })
            .collect(),
    }
}

/// Template filter: `Option<u32>` as `N%`, or `n/a`.
fn percent(value: Option<u32>) -> String {
    value.map_or_else(|| "n/a".to_string(), |p| format!("{p}%"))
}

/// Template filter: makes text safe inside a Markdown table cell. `&`, `<`
/// and `>` become entities, so text such as `Option<u32>` is not read as
/// HTML; `|` is escaped, and a line break becomes `<br>`.
fn cell(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('|', "\\|")
        .replace("\r\n", "<br>")
        .replace(['\n', '\r'], "<br>")
}

/// Renders the audit report for `project` from the template in `src`.
///
/// The report is LF-only and ends with exactly one newline.
///
/// # Errors
///
/// Returns a message when the template is missing or fails to render.
pub fn render(src: &dyn Source, project: &str, ev: &Evaluation) -> Result<String, String> {
    let bytes = src
        .read(TEMPLATE)
        .ok_or_else(|| format!("{TEMPLATE}: template not found"))?;
    let text = String::from_utf8(bytes.into_owned())
        .map_err(|_| format!("{TEMPLATE}: template is not valid UTF-8"))?;
    let text = text
        .strip_prefix('\u{feff}')
        .unwrap_or(&text)
        .replace("\r\n", "\n");

    let syntax = minijinja::syntax::SyntaxConfig::builder()
        .trim_blocks(true)
        .lstrip_blocks(true)
        .build()
        .map_err(|e| format!("{TEMPLATE}: {e}"))?;
    let mut env = Environment::new();
    env.set_syntax(syntax);
    env.set_auto_escape_callback(|_| AutoEscape::None);
    env.add_filter("percent", percent);
    env.add_filter("cell", cell);
    env.add_template_owned(TEMPLATE, text)
        .map_err(|e| format!("{TEMPLATE}: {e:#}"))?;

    let rendered = env
        .get_template(TEMPLATE)
        .and_then(|t| t.render(Serde(view(project, ev))))
        .map_err(|e| format!("{TEMPLATE}: {e:#}"))?;
    Ok(format!("{}\n", rendered.trim_end()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluate::evaluate;
    use crate::record::{Outcome, Suppression};
    use crate::source::{Embedded, MemSource};
    use crate::testfixture::{DEVIATION, catalog, record, results};

    const GOLDEN: &str = "src/testdata/report.golden.md";

    #[test]
    fn render_golden() {
        let c = catalog();
        let r = record(&format!("facts: {{ uses-database: true }}\n{DEVIATION}"));
        let mut res = results(&[
            ("USS-SEC-001", Outcome::Deviation),
            ("USS-SEC-002", Outcome::Fail),
        ]);
        res.suppressions.push(Suppression {
            file: "src/db.go".to_string(),
            line: Some(42),
            rule: "USS-SEC-002".to_string(),
            reason: "legacy | account".to_string(),
        });
        let ev = evaluate(&c, &r, &res).unwrap();
        let text = render(&Embedded, "p", &ev).unwrap();
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(GOLDEN);
        if std::env::var("USS_UPDATE_GOLDEN").is_ok_and(|v| v == "1") {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, &text).unwrap();
        }
        let want = std::fs::read_to_string(&path).unwrap();
        assert_eq!(text, want);
    }

    #[test]
    fn render_escapes_cells() {
        let c = catalog();
        let r = record("facts: {}\ndecisions: { USS-EXT-001: { adopt: false } }\n");
        let mut res = results(&[("USS-SEC-001", Outcome::Fail)]);
        res.results.get_mut("USS-SEC-001").unwrap().evidence =
            "a|b\nc returns Option<u32> & more".to_string();
        res.suppressions.push(Suppression {
            file: "src/a.rs".to_string(),
            line: None,
            rule: "lint|<T>&".to_string(),
            reason: "r".to_string(),
        });
        let ev = evaluate(&c, &r, &res).unwrap();
        let text = render(&Embedded, "p", &ev).unwrap();
        assert!(
            text.contains("| a\\|b<br>c returns Option&lt;u32&gt; &amp; more |"),
            "{text}"
        );
        assert!(
            text.contains("| src/a.rs |  | lint\\|&lt;T&gt;&amp; | r |"),
            "{text}"
        );
    }

    #[test]
    fn render_sorts_by_level_then_id() {
        let c = catalog();
        let r = record(
            "facts: { uses-database: true, has-ui: true }\ndecisions: { USS-EXT-001: { adopt: false } }\n",
        );
        let res = results(&[
            ("USS-SEC-001", Outcome::Fail),
            ("USS-SEC-002", Outcome::Fail),
            ("USS-SEC-003", Outcome::Fail),
        ]);
        let ev = evaluate(&c, &r, &res).unwrap();
        let text = render(&Embedded, "p", &ev).unwrap();
        let at = |id: &str| text.find(&format!("| {id} |")).unwrap();
        assert!(at("USS-SEC-001") < at("USS-SEC-003"), "{text}");
        assert!(at("USS-SEC-003") < at("USS-SEC-002"), "{text}");
        assert!(text.ends_with("|\n") && !text.ends_with("\n\n"), "{text}");
        assert!(!text.contains('\r'));
    }

    #[test]
    fn render_template_crlf_and_bom() {
        let c = catalog();
        let r = record(&format!("facts: {{ uses-database: true }}\n{DEVIATION}"));
        let ev = evaluate(&c, &r, &results(&[("USS-SEC-001", Outcome::Deviation)])).unwrap();
        let lf = render(&Embedded, "p", &ev).unwrap();

        let template = Embedded.read(TEMPLATE).unwrap();
        let template = String::from_utf8(template.into_owned()).unwrap();
        let crlf = format!("\u{feff}{}", template.replace('\n', "\r\n"));
        let mut src = MemSource::default();
        src.0.insert(TEMPLATE.to_string(), crlf.into_bytes());
        let text = render(&src, "p", &ev).unwrap();
        assert!(
            !text.contains('\r') && !text.contains('\u{feff}'),
            "{text:?}"
        );
        assert_eq!(text, lf);
    }

    #[test]
    fn render_missing_template() {
        let c = catalog();
        let r = record("facts: {}\ndecisions: { USS-EXT-001: { adopt: false } }\n");
        let ev = evaluate(&c, &r, &results(&[])).unwrap();
        let err = render(&MemSource::default(), "p", &ev).unwrap_err();
        assert!(err.contains("templates/audit-report.md.j2"), "{err}");
    }
}
