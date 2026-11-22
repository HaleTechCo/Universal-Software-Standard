//! `standard/applicability.md`: the vocabulary of applicability facts.

use std::collections::{BTreeMap, BTreeSet};

use super::Diagnostic;
use super::markdown::parse_doc;
use super::registry::ID_RE;

/// Where the value of a fact comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FactSource {
    /// The developer is asked this question.
    Question(String),
    /// A rule sets the fact; holds the text of the `Enabled by` field.
    EnabledBy(String),
}

/// One applicability fact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fact {
    /// The fact's ID (lowercase, hyphenated).
    pub id: String,
    /// Where its value comes from.
    pub source: FactSource,
    /// When the answer is yes.
    pub definition: String,
    /// IDs of the facts that are also true when this one is.
    pub implies: Vec<String>,
    /// File the fact is defined in.
    pub file: String,
    /// 1-based line of the fact's heading.
    pub line: usize,
}

/// The fields a fact section may have.
const ALLOWED: [&str; 4] = ["Question", "Definition", "Enabled by", "Implies"];

/// Parses the fact vocabulary in `data`.
///
/// Facts whose section has an invalid ID are left out of the map; the
/// diagnostics say why.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the rule-file parser and the catalog loader use it from the next tasks"
    )
)]
pub(crate) fn parse_facts(file: &str, data: &[u8]) -> (BTreeMap<String, Fact>, Vec<Diagnostic>) {
    let (doc, mut diags) = parse_doc(file, data);
    let at = |line: usize, message: String| Diagnostic {
        file: file.to_string(),
        line,
        message,
    };
    let mut facts: BTreeMap<String, Fact> = BTreeMap::new();

    for s in &doc.sections {
        let id = s.heading.as_str();
        if !ID_RE.is_match(id) {
            diags.push(at(s.line, format!("invalid fact ID \"{id}\"")));
            continue;
        }
        if facts.contains_key(id) {
            diags.push(at(s.line, format!("duplicate fact \"{id}\"")));
            continue;
        }
        for (name, f) in &s.fields {
            if !ALLOWED.contains(&name.as_str()) {
                diags.push(at(f.line, format!("unknown field \"{name}\"")));
            }
        }
        let definition = s.fields.get("Definition").map(|f| f.value.clone());
        if definition.is_none() {
            diags.push(at(s.line, "missing field \"Definition\"".to_string()));
        }
        let source = match (s.fields.get("Question"), s.fields.get("Enabled by")) {
            (Some(q), None) => Some(FactSource::Question(q.value.clone())),
            (None, Some(e)) => Some(FactSource::EnabledBy(e.value.clone())),
            _ => {
                diags.push(at(
                    s.line,
                    "needs exactly one of \"Question\" or \"Enabled by\"".to_string(),
                ));
                None
            }
        };
        let implies = s.fields.get("Implies").map_or_else(Vec::new, |f| {
            f.value
                .split(',')
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(str::to_string)
                .collect()
        });
        if let (Some(definition), Some(source)) = (definition, source) {
            facts.insert(
                id.to_string(),
                Fact {
                    id: id.to_string(),
                    source,
                    definition,
                    implies,
                    file: file.to_string(),
                    line: s.line,
                },
            );
        }
    }

    for f in facts.values() {
        for target in &f.implies {
            if !facts.contains_key(target) {
                diags.push(at(f.line, format!("unknown fact \"{target}\"")));
            }
        }
    }
    find_cycles(&facts, &at, &mut diags);
    (facts, diags)
}

/// Reports each implication cycle once, at the fact where it was found.
fn find_cycles(
    facts: &BTreeMap<String, Fact>,
    at: &dyn Fn(usize, String) -> Diagnostic,
    diags: &mut Vec<Diagnostic>,
) {
    /// Depth-first walk; `path` holds the facts being visited.
    fn visit<'a>(
        id: &'a str,
        facts: &'a BTreeMap<String, Fact>,
        done: &mut BTreeSet<&'a str>,
        path: &mut Vec<&'a str>,
        found: &mut Vec<(usize, String)>,
    ) {
        if let Some(pos) = path.iter().position(|p| *p == id) {
            let mut cycle = path[pos..].to_vec();
            cycle.push(id);
            let line = facts.get(id).map_or(0, |f| f.line);
            found.push((line, format!("implication cycle: {}", cycle.join(" -> "))));
            return;
        }
        if !done.insert(id) {
            return;
        }
        path.push(id);
        if let Some(f) = facts.get(id) {
            for next in &f.implies {
                if facts.contains_key(next) {
                    visit(next, facts, done, path, found);
                }
            }
        }
        path.pop();
    }

    let mut done = BTreeSet::new();
    let mut found = Vec::new();
    for id in facts.keys() {
        visit(id, facts, &mut done, &mut Vec::new(), &mut found);
    }
    diags.extend(found.into_iter().map(|(line, msg)| at(line, msg)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{Embedded, Source};

    fn run(src: &str) -> (BTreeMap<String, Fact>, String) {
        let (facts, diags) = parse_facts("applicability.md", src.as_bytes());
        let text = diags
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        (facts, text)
    }

    #[test]
    fn parse_facts_valid() {
        let src = "# Applicability facts\n\nIntro.\n\n### has-ui\n\n- **Question:** Does it have a UI?\n- **Definition:** People use a UI.\n\n### has-web-ui\n\n- **Question:** Does it have a web UI?\n- **Definition:** It serves a UI in a browser.\n- **Implies:** has-ui\n\n### from-rule\n\n- **Enabled by:** USS-X-001\n- **Definition:** A rule sets it.\n";
        let (facts, text) = run(src);
        assert_eq!(text, "");
        assert_eq!(facts.len(), 3);
        let web = &facts["has-web-ui"];
        assert_eq!(web.implies, ["has-ui"]);
        assert_eq!(web.line, 10);
        assert_eq!(web.file, "applicability.md");
        assert_eq!(web.definition, "It serves a UI in a browser.");
        assert_eq!(
            web.source,
            FactSource::Question("Does it have a web UI?".to_string())
        );
        assert_eq!(facts["has-ui"].implies.len(), 0);
        assert_eq!(
            facts["from-rule"].source,
            FactSource::EnabledBy("USS-X-001".to_string())
        );
    }

    #[test]
    fn parse_facts_errors() {
        let cases: [(&str, &str); 7] = [
            (
                "### Has_UI\n\n- **Question:** Q?\n- **Definition:** D.\n",
                "Has_UI",
            ),
            (
                "### a\n\n- **Question:** Q?\n- **Enabled by:** USS-X-001\n- **Definition:** D.\n",
                "exactly one of",
            ),
            ("### a\n\n- **Definition:** D.\n", "exactly one of"),
            (
                "### a\n\n- **Question:** Q?\n",
                "missing field \"Definition\"",
            ),
            (
                "### a\n\n- **Question:** Q?\n- **Definition:** D.\n- **Implies:** nope\n",
                "unknown fact \"nope\"",
            ),
            (
                "### a\n\n- **Question:** Q?\n- **Definition:** D.\n- **Implies:** b\n\n### b\n\n- **Question:** Q?\n- **Definition:** D.\n- **Implies:** a\n",
                "implication cycle",
            ),
            (
                "### a\n\n- **Question:** Q?\n- **Definition:** D.\n- **Colour:** red\n",
                "unknown field \"Colour\"",
            ),
        ];
        for (i, (body, want)) in cases.iter().enumerate() {
            let (_, text) = run(&format!("# T\n\n{body}"));
            assert!(text.contains(want), "case {i}: want {want:?} in {text:?}");
        }
    }

    #[test]
    fn parse_facts_reports_line_of_field() {
        let (_, text) =
            run("# T\n\n### a\n\n- **Question:** Q?\n- **Definition:** D.\n- **Colour:** red\n");
        assert!(
            text.contains("applicability.md:7: unknown field \"Colour\""),
            "{text}"
        );
    }

    #[test]
    fn parse_facts_duplicate_field_and_fact() {
        let (_, text) = run(
            "# T\n\n### a\n\n- **Question:** Q?\n- **Question:** R?\n- **Definition:** D.\n\n### a\n\n- **Question:** Q?\n- **Definition:** D.\n",
        );
        assert!(
            text.contains("applicability.md:6: duplicate field \"Question\""),
            "{text}"
        );
        assert!(text.contains("duplicate fact \"a\""), "{text}");
    }

    #[test]
    fn applicability_file_is_valid() {
        let data = Embedded.read("standard/applicability.md").unwrap();
        let (facts, diags) = parse_facts("standard/applicability.md", &data);
        assert!(diags.is_empty(), "{diags:?}");
        let want = [
            "has-ui",
            "has-web-ui",
            "has-cli",
            "stores-persistent-data",
            "stores-user-data",
            "uses-database",
            "uses-relational-database",
            "exposes-network-api",
            "is-network-service",
            "operates-service",
            "handles-authentication",
            "handles-sensitive-data",
            "high-assurance",
            "uses-cryptography",
            "requires-fips",
            "ships-container-image",
            "deploys-to-kubernetes",
            "is-library",
            "distributed-as-package",
            "accepts-external-contributions",
            "uses-ai-models",
            "exposes-mcp-server",
            "has-multiple-users",
            "is-commercial",
            "is-foss",
            "sold-in-eu",
            "processes-eu-personal-data",
        ];
        assert_eq!(want.len(), 27);
        let mut got: Vec<&str> = facts.keys().map(String::as_str).collect();
        let mut want_sorted = want.to_vec();
        got.sort_unstable();
        want_sorted.sort_unstable();
        assert_eq!(got, want_sorted);
    }
}
