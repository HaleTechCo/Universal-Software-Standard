//! The catalog of the standard: registry, rules and related data.

use std::collections::BTreeMap;
use std::path::Path;

use crate::source::Source;
use facts::parse_facts;
use rules::parse_rule_file;

/// Path of the facts file, relative to the USS root.
const FACTS_PATH: &str = "standard/applicability.md";

mod diag;
mod export;
mod facts;
pub(crate) mod markdown;
pub(crate) mod registry;
mod rules;

pub use diag::Diagnostic;
pub use export::{Export, ExportCategory, ExportFact, ExportRule};
pub use facts::{Fact, FactSource};
pub use registry::{Category, Registry, load_registry};
pub use rules::{Active, FileKind, Level, Rule, RuleFile, RuleState};

/// The loaded and cross-checked standard.
#[derive(Debug, Clone)]
pub struct Catalog {
    /// The registry.
    pub registry: Registry,
    /// The applicability facts by ID.
    pub facts: BTreeMap<String, Fact>,
    /// Every rule, retired ones included, sorted by ID.
    pub rules: Vec<Rule>,
    /// How many rule files were read.
    pub file_count: usize,
}

impl Catalog {
    /// Finds a rule by ID.
    #[must_use]
    pub fn rule(&self, id: &str) -> Option<&Rule> {
        self.rules.iter().find(|r| r.id == id)
    }

    /// Tells whether `id` is a defined fact, or a `kind.id` stack identifier
    /// that the registry lists.
    #[must_use]
    pub fn known_ident(&self, id: &str) -> bool {
        self.facts.contains_key(id)
            || id
                .split_once('.')
                .is_some_and(|(kind, stack_id)| self.registry.has_stack(kind, stack_id))
    }
}

/// Loads the whole standard from `src` and runs the cross-file checks.
///
/// # Errors
///
/// Returns every diagnostic found, sorted by file and line.
pub fn load(src: &dyn Source) -> Result<Catalog, Vec<Diagnostic>> {
    let mut diags = Vec::new();

    let registry = match load_registry(src) {
        Ok(r) => Some(r),
        Err(d) => {
            diags.extend(d);
            None
        }
    };

    let mut facts = BTreeMap::new();
    if let Some(data) = src.read(FACTS_PATH) {
        let (parsed, d) = parse_facts(FACTS_PATH, &data);
        facts = parsed;
        diags.extend(d);
    } else {
        diags.push(Diagnostic::new(FACTS_PATH, "file not found"));
    }

    let mut files = Vec::new();
    for (dir, kind) in [
        ("standard/core", FileKind::Area),
        ("standard/modules", FileKind::Module),
    ] {
        for path in src.list(dir) {
            if !Path::new(&path)
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("md"))
            {
                continue;
            }
            let Some(data) = src.read(&path) else {
                diags.push(Diagnostic::new(&path, "file not found"));
                continue;
            };
            let (file, d) = parse_rule_file(&path, &data);
            diags.extend(d);
            if file.kind != kind {
                let want = if kind == FileKind::Area {
                    "area"
                } else {
                    "module"
                };
                diags.push(Diagnostic::new(
                    &path,
                    format!("a file under {dir}/ must have Kind {want}"),
                ));
            }
            files.push(file);
        }
    }

    let file_count = files.len();
    let mut rules: Vec<Rule> = files.iter().flat_map(|f| f.rules.clone()).collect();
    rules.sort_by(|a, b| a.id.cmp(&b.id));

    let Some(registry) = registry else {
        return Err(sorted(diags));
    };
    let catalog = Catalog {
        registry,
        facts,
        rules,
        file_count,
    };
    catalog.check(&files, &mut diags);
    if diags.is_empty() {
        Ok(catalog)
    } else {
        Err(sorted(diags))
    }
}

/// Sorts diagnostics by file, then line.
fn sorted(mut diags: Vec<Diagnostic>) -> Vec<Diagnostic> {
    diags.sort_by(|a, b| (&a.file, a.line).cmp(&(&b.file, b.line)));
    diags
}

impl Catalog {
    /// Runs the checks that need more than one file.
    fn check(&self, files: &[RuleFile], diags: &mut Vec<Diagnostic>) {
        Self::check_unique(files, diags);
        for rule in &self.rules {
            let at = |message: String| Diagnostic {
                file: rule.file.clone(),
                line: rule.line,
                message,
            };
            match &rule.state {
                RuleState::Active(a) => self.check_active(rule, a, &at, diags),
                RuleState::Retired {
                    replaced_by: Some(target),
                    ..
                } if self.rule(target).is_none() => {
                    diags.push(at(format!(
                        "{} is replaced by unknown rule {target}",
                        rule.id
                    )));
                }
                RuleState::Retired { .. } => {}
            }
        }
        for fact in self.facts.values() {
            let FactSource::EnabledBy(by) = &fact.source else {
                continue;
            };
            let enables = self
                .rule(by)
                .and_then(Rule::active)
                .is_some_and(|a| a.level == Level::May && a.enables.contains(&fact.id));
            if !enables {
                diags.push(Diagnostic {
                    file: fact.file.clone(),
                    line: fact.line,
                    message: format!(
                        "fact \"{}\" is enabled by \"{by}\", which is not a MAY rule that enables it",
                        fact.id
                    ),
                });
            }
        }
    }

    /// Reports duplicate rule IDs and prefixes shared by two files.
    fn check_unique(files: &[RuleFile], diags: &mut Vec<Diagnostic>) {
        let mut first: BTreeMap<&str, &Rule> = BTreeMap::new();
        for file in files {
            for rule in &file.rules {
                match first.get(rule.id.as_str()) {
                    Some(prev) => diags.push(Diagnostic {
                        file: rule.file.clone(),
                        line: rule.line,
                        message: format!(
                            "duplicate rule ID {} (first at {}:{})",
                            rule.id, prev.file, prev.line
                        ),
                    }),
                    None => {
                        first.insert(rule.id.as_str(), rule);
                    }
                }
            }
        }
        let mut prefixes: BTreeMap<&str, &str> = BTreeMap::new();
        for file in files.iter().filter(|f| !f.prefix.is_empty()) {
            match prefixes.get(file.prefix.as_str()) {
                Some(prev) => diags.push(Diagnostic {
                    file: file.file.clone(),
                    line: 1,
                    message: format!("prefix {} is also used by {prev}", file.prefix),
                }),
                None => {
                    prefixes.insert(file.prefix.as_str(), file.file.as_str());
                }
            }
        }
    }

    /// Checks the references of one active rule.
    fn check_active(
        &self,
        rule: &Rule,
        a: &Active,
        at: &dyn Fn(String) -> Diagnostic,
        diags: &mut Vec<Diagnostic>,
    ) {
        if self.registry.category_by_name(&a.category).is_none() {
            diags.push(at(format!(
                "{} has unregistered category \"{}\"",
                rule.id, a.category
            )));
        }
        for ident in a.applies.idents() {
            if !self.known_ident(&ident) {
                diags.push(at(format!(
                    "unknown identifier \"{ident}\" in \"Applies when\""
                )));
            }
        }
        for id in &a.enables {
            match self.facts.get(id) {
                None => diags.push(at(format!("{} enables unknown fact \"{id}\"", rule.id))),
                Some(fact) => match &fact.source {
                    FactSource::EnabledBy(by) if *by == rule.id => {}
                    FactSource::EnabledBy(by) => diags.push(at(format!(
                        "fact \"{id}\" is enabled by {by}, not {}",
                        rule.id
                    ))),
                    FactSource::Question(_) => diags.push(at(format!(
                        "fact \"{id}\" is asked as a question, not enabled by {}",
                        rule.id
                    ))),
                },
            }
        }
        for target in &a.refines {
            match self.rule(target) {
                None => diags.push(at(format!("{} refines unknown rule {target}", rule.id))),
                Some(t) if t.kind != FileKind::Area => diags.push(at(format!(
                    "{} refines {target}, which is not in an area file",
                    rule.id
                ))),
                Some(_) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::MemSource;
    use crate::testfixture;

    fn load_fixture() -> Catalog {
        match load(&testfixture::standard()) {
            Ok(c) => c,
            Err(d) => panic!("fixture has diagnostics: {d:?}"),
        }
    }

    fn edited(path: &str, from: &str, to: &str) -> MemSource {
        let mut m = testfixture::standard();
        let text = String::from_utf8(m.0[path].clone()).unwrap();
        assert!(text.contains(from), "fixture lacks {from:?}");
        m.0.insert(path.to_string(), text.replace(from, to).into_bytes());
        m
    }

    fn text_of(diags: &[Diagnostic]) -> String {
        diags
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn load_fixture_ok() {
        let c = load_fixture();
        assert_eq!(c.rules.len(), 7);
        assert_eq!(c.file_count, 3);
        assert_eq!(c.rules[0].id, "USS-EXT-001");
        assert_eq!(c.facts.len(), 4);
        let go = c.rule("USS-GO-001").unwrap();
        assert_eq!(go.active().unwrap().applies.to_string(), "lang.go");
        assert!(c.rule("USS-NOPE-001").is_none());
        assert!(c.known_ident("has-ui"));
        assert!(c.known_ident("lang.go"));
        assert!(!c.known_ident("lang.cobol"));
        assert!(!c.known_ident("nope"));
    }

    #[test]
    fn load_cross_file_errors() {
        let sec = "standard/core/security.md";
        let ext = "standard/core/extensibility.md";
        let go = "standard/modules/languages/go.md";
        let app = "standard/applicability.md";
        let cases: Vec<(MemSource, &str)> = vec![
            (
                edited(ext, "USS-EXT-002: Secure", "USS-EXT-001: Secure"),
                "duplicate rule ID USS-EXT-001",
            ),
            (
                edited(go, "- **Prefix:** GO", "- **Prefix:** EXT"),
                "prefix EXT is also used",
            ),
            (
                edited(ext, "**Category:** Features", "**Category:** Nope"),
                "unregistered category \"Nope\"",
            ),
            (
                edited(
                    sec,
                    "uses-database\n- **Category",
                    "uses-databse\n- **Category",
                ),
                "unknown identifier \"uses-databse\"",
            ),
            (
                edited(
                    ext,
                    "**Enables:** has-plugin-interface",
                    "**Enables:** has-nothing",
                ),
                "unknown fact \"has-nothing\"",
            ),
            (
                edited(
                    app,
                    "**Enabled by:** USS-EXT-001",
                    "**Enabled by:** USS-EXT-002",
                ),
                "is enabled by USS-EXT-002, not USS-EXT-001",
            ),
            (
                edited(
                    app,
                    "**Enabled by:** USS-EXT-001",
                    "**Enabled by:** USS-SEC-001",
                ),
                "not a MAY rule that enables it",
            ),
            (
                edited(go, "**Refines:** USS-SEC-001", "**Refines:** USS-SEC-099"),
                "refines unknown rule USS-SEC-099",
            ),
            (
                edited(go, "**Refines:** USS-SEC-001", "**Refines:** USS-GO-001"),
                "not in an area file",
            ),
            (
                edited(
                    sec,
                    "**Replaced by:** USS-SEC-001",
                    "**Replaced by:** USS-SEC-099",
                ),
                "replaced by unknown rule USS-SEC-099",
            ),
        ];
        for (i, (src, want)) in cases.iter().enumerate() {
            let text = text_of(&load(src).unwrap_err());
            assert!(text.contains(want), "case {i}: want {want:?} in:\n{text}");
        }
    }

    #[test]
    fn load_duplicate_within_one_file() {
        let src = edited(
            "standard/core/security.md",
            "USS-SEC-002: Use",
            "USS-SEC-001: Use",
        );
        let text = text_of(&load(&src).unwrap_err());
        assert!(text.contains("duplicate rule ID USS-SEC-001"), "{text}");
    }

    #[test]
    fn load_file_kind_and_missing_dirs() {
        let src = edited(
            "standard/core/security.md",
            "**Kind:** area",
            "**Kind:** module",
        );
        let text = text_of(&load(&src).unwrap_err());
        assert!(text.contains("must have Kind area"), "{text}");

        let mut m = testfixture::standard();
        m.0.retain(|p, _| !p.starts_with("standard/modules/"));
        let c = load(&m).unwrap();
        assert_eq!((c.rules.len(), c.file_count), (6, 2));
    }

    #[test]
    fn load_sorts_diagnostics() {
        let mut m = edited(
            "standard/core/security.md",
            "- **Verify:** Check that external input is validated.\n",
            "",
        );
        m.0.remove("standard/applicability.md");
        let diags = load(&m).unwrap_err();
        let files: Vec<_> = diags.iter().map(|d| d.file.as_str()).collect();
        let mut sorted = files.clone();
        sorted.sort_unstable();
        assert_eq!(files, sorted);
        assert!(files.contains(&"standard/applicability.md"));
        assert!(files.contains(&"standard/core/security.md"));
    }
}
