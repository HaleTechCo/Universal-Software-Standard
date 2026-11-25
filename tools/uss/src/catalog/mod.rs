//! The catalog of the standard: registry, rules and related data.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::expr::{self, Expr};
use crate::source::Source;
use facts::parse_facts;
use profiles::{check_mappings, parse_profile};
use rules::{RULE_ID_RE, list, parse_rule_file};

/// Directory of the vendor profiles.
const VENDORS_DIR: &str = "standard/vendors";

/// Directory of the regulation modules.
const REGULATIONS_DIR: &str = "standard/modules/regulations/";

/// The header of the obligations table of a regulation module.
const OBLIGATION_HEADER: [&str; 4] = ["Obligation", "Source", "Rules", "Coverage"];

/// Most identifiers that the reach check of `Refines` enumerates.
const MAX_REACH_IDENTS: usize = 16;

/// Path of the facts file, relative to the USS root.
const FACTS_PATH: &str = "standard/applicability.md";

mod diag;
mod export;
mod facts;
pub(crate) mod markdown;
mod profiles;
pub(crate) mod registry;
mod rules;

pub use diag::Diagnostic;
pub use export::{
    Export, ExportCategory, ExportFact, ExportMapping, ExportProfile, ExportRule, rule_markdown,
};
pub use facts::{Fact, FactSource};
pub use profiles::{Coverage, Maintainer, Mapping, Profile};
pub use registry::{Category, Registry, load_registry};
pub use rules::{Active, FileKind, Level, Rule, RuleFile, RuleState};

/// A module file of the standard, with the condition under which it applies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleFile {
    /// Path of the file, relative to the USS root.
    pub file: String,
    /// Text of the file's `#` title.
    pub title: String,
    /// The file's `Applies when` header condition.
    pub applies: Expr,
}

/// The loaded and cross-checked standard.
#[derive(Debug, Clone)]
pub struct Catalog {
    /// The registry.
    pub registry: Registry,
    /// The applicability facts by ID.
    pub facts: BTreeMap<String, Fact>,
    /// Every rule, retired ones included, sorted by ID.
    pub rules: Vec<Rule>,
    /// How many area, module and profile files were read.
    pub file_count: usize,
    /// Every file under `standard/modules/`, sorted by path.
    pub modules: Vec<ModuleFile>,
    /// Every vendor profile, sorted by path.
    pub profiles: Vec<Profile>,
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
    let mut texts: Vec<(String, Vec<u8>)> = Vec::new();
    if let Some(data) = src.read(FACTS_PATH) {
        texts.push((FACTS_PATH.to_string(), data.to_vec()));
    }
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
            texts.push((path.clone(), data.to_vec()));
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

    let profiles = load_profiles(src, &mut files, &mut texts, &mut diags);

    let file_count = files.len();
    let mut modules: Vec<ModuleFile> = files
        .iter()
        .filter(|f| f.file.starts_with("standard/modules/"))
        .map(|f| ModuleFile {
            file: f.file.clone(),
            title: f.title.clone(),
            applies: expr::parse(&f.applies_when).unwrap_or(Expr::Always),
        })
        .collect();
    modules.sort_by(|a, b| a.file.cmp(&b.file));
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
        modules,
        profiles,
    };
    catalog.check(&files, &texts, &mut diags);
    if diags.is_empty() {
        Ok(catalog)
    } else {
        Err(sorted(diags))
    }
}

/// Reads every profile under `standard/vendors/`, adding its rule file and
/// text to `files` and `texts`.
fn load_profiles(
    src: &dyn Source,
    files: &mut Vec<RuleFile>,
    texts: &mut Vec<(String, Vec<u8>)>,
    diags: &mut Vec<Diagnostic>,
) -> Vec<Profile> {
    let mut profiles = Vec::new();
    for path in src.list(VENDORS_DIR) {
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
        let (file, profile, d) = parse_profile(&path, &data);
        texts.push((path, data.to_vec()));
        diags.extend(d);
        files.push(file);
        profiles.push(profile);
    }
    profiles
}

/// Sorts diagnostics by file, then line.
fn sorted(mut diags: Vec<Diagnostic>) -> Vec<Diagnostic> {
    diags.sort_by(|a, b| (&a.file, a.line).cmp(&(&b.file, b.line)));
    diags
}

impl Catalog {
    /// Runs the checks that need more than one file.
    fn check(&self, files: &[RuleFile], texts: &[(String, Vec<u8>)], diags: &mut Vec<Diagnostic>) {
        Self::check_unique(files, diags);
        // The unknown IDs that an active rule's `Refines` names, by file: the
        // check of `Refines` reports them, so the text check skips them on the
        // lines of a `Refines` field.
        let unknown_refines: BTreeSet<(&str, &str)> = self
            .rules
            .iter()
            .filter_map(|r| r.active().map(|a| (r.file.as_str(), a)))
            .flat_map(|(file, a)| {
                a.refines
                    .iter()
                    .filter(|t| self.rule(t).is_none())
                    .map(move |t| (file, t.as_str()))
            })
            .collect();
        for (path, data) in texts {
            let refines_lines = markdown::field_lines(data, "Refines");
            for (line, id) in markdown::rule_refs(data) {
                let reported = refines_lines.contains(&line)
                    && unknown_refines.contains(&(path.as_str(), id.as_str()));
                if self.rule(&id).is_none() && !reported {
                    diags.push(Diagnostic {
                        file: path.clone(),
                        line,
                        message: format!("unknown rule {id}"),
                    });
                }
            }
        }
        for profile in &self.profiles {
            check_mappings(self, profile, diags);
        }
        for (path, data) in texts.iter().filter(|(p, _)| p.starts_with(REGULATIONS_DIR)) {
            self.check_regulation(path, data, diags);
        }
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

    /// Checks a regulation module: its disclaimer, its obligations table and
    /// that none of its rules uses `Refines`.
    fn check_regulation(&self, path: &str, data: &[u8], diags: &mut Vec<Diagnostic>) {
        let (doc, _) = markdown::parse_doc(path, data);
        let at = |line: usize, message: String| Diagnostic {
            file: path.to_string(),
            line,
            message,
        };
        for rule in self.rules.iter().filter(|r| r.file == path) {
            if rule.active().is_some_and(|a| !a.refines.is_empty()) {
                diags.push(at(
                    rule.line,
                    format!(
                        "{} uses \"Refines\"; a regulation rule maps through the obligations table",
                        rule.id
                    ),
                ));
            }
        }
        match doc.groups.iter().find(|g| g.heading == "Disclaimer") {
            None => diags.push(at(1, "missing section \"Disclaimer\"".to_string())),
            Some(g) if g.lines.iter().all(|l| l.text.trim().is_empty()) => {
                diags.push(at(g.line, "section \"Disclaimer\" is empty".to_string()));
            }
            Some(_) => {}
        }
        let Some(g) = doc.groups.iter().find(|g| g.heading == "Obligations") else {
            diags.push(at(1, "missing section \"Obligations\"".to_string()));
            return;
        };
        let table = match markdown::parse_table(&g.lines) {
            Ok(t) if t.header == OBLIGATION_HEADER => t,
            Ok(_) => {
                diags.push(at(
                    g.line,
                    format!(
                        "the obligations table header must be \"{}\"",
                        OBLIGATION_HEADER.join(" | ")
                    ),
                ));
                return;
            }
            Err((line, message)) => {
                diags.push(at(if line == 0 { g.line } else { line }, message));
                return;
            }
        };
        if table.rows.is_empty() {
            diags.push(at(g.line, "the obligations table has no rows".to_string()));
        }
        for row in &table.rows {
            let [obligation, source, rules, coverage] = &row.cells[..] else {
                continue;
            };
            let at = |message: String| at(row.line, message);
            for (cell, name) in [(obligation, "Obligation"), (source, "Source")] {
                if cell.is_empty() {
                    diags.push(at(format!("empty \"{name}\" cell")));
                }
            }
            let ids = if rules == "none" {
                Vec::new()
            } else {
                list(rules)
            };
            if ids.is_empty() && rules != "none" {
                diags.push(at("\"Rules\" must be none or rule IDs".to_string()));
            }
            for id in &ids {
                if !RULE_ID_RE.is_match(id) {
                    diags.push(at(format!("\"{id}\" is not a rule ID")));
                } else if self.rule(id).is_some_and(|r| r.active().is_none()) {
                    diags.push(at(format!("obligation maps retired rule {id}")));
                }
            }
            let kind = match coverage.split_once(':') {
                None if coverage == "full" => "full",
                Some((k @ ("partial" | "none"), rest)) if !rest.trim().is_empty() => k,
                _ => {
                    diags.push(at(format!(
                        "invalid coverage \"{coverage}\" (want full, partial: <what is left> or none: <what is left>)"
                    )));
                    continue;
                }
            };
            if ids.is_empty() && kind != "none" {
                diags.push(at(format!(
                    "an obligation with no rules has coverage \"{kind}\"; it must be \"none: <what is left>\""
                )));
            } else if !ids.is_empty() && kind == "none" {
                diags.push(at(
                    "an obligation with rules has coverage \"none\"; it must be full or partial"
                        .to_string(),
                ));
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
                Some(t) => self.check_reach(rule, a, t, at, diags),
            }
        }
    }

    /// Checks that `rule`, which refines `target`, applies only where
    /// `target` does.
    fn check_reach(
        &self,
        rule: &Rule,
        a: &Active,
        target: &Rule,
        at: &dyn Fn(String) -> Diagnostic,
        diags: &mut Vec<Diagnostic>,
    ) {
        let Some(t) = target.active() else {
            diags.push(at(format!(
                "{} refines retired rule {}",
                rule.id, target.id
            )));
            return;
        };
        let idents: Vec<String> = a
            .applies
            .idents()
            .union(&t.applies.idents())
            .cloned()
            .collect();
        if idents.len() > MAX_REACH_IDENTS {
            diags.push(at(format!(
                "{}: cannot check that it applies only where {} does: {} identifiers, over the limit of {MAX_REACH_IDENTS}",
                rule.id,
                target.id,
                idents.len()
            )));
            return;
        }
        for mask in 0..(1_u32 << idents.len()) {
            let mut set: BTreeSet<String> = idents
                .iter()
                .enumerate()
                .filter(|(i, _)| mask >> i & 1 == 1)
                .map(|(_, id)| id.clone())
                .collect();
            self.close_implies(&mut set);
            if a.applies.eval(&set) && !t.applies.eval(&set) {
                let shown = if set.is_empty() {
                    "nothing".to_string()
                } else {
                    set.into_iter().collect::<Vec<_>>().join(", ")
                };
                diags.push(at(format!(
                    "{} applies where {}, which it refines, does not (for example when true: {shown})",
                    rule.id, target.id
                )));
                return;
            }
        }
    }

    /// Adds to `set` every fact that a fact in it implies, directly or not.
    fn close_implies(&self, set: &mut BTreeSet<String>) {
        let mut queue: Vec<String> = set.iter().cloned().collect();
        while let Some(id) = queue.pop() {
            for next in self.facts.get(&id).map_or(&[][..], |f| &f.implies[..]) {
                if set.insert(next.clone()) {
                    queue.push(next.clone());
                }
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
        assert_eq!(c.rules.len(), 8);
        assert_eq!(c.file_count, 5);
        assert_eq!(c.rules[0].id, "USS-EXT-001");
        assert_eq!(c.facts.len(), 4);
        let go = c.rule("USS-GO-001").unwrap();
        assert_eq!(go.active().unwrap().applies.to_string(), "lang.go");
        assert!(c.rule("USS-NOPE-001").is_none());
        assert_eq!(c.modules.len(), 2);
        assert_eq!(c.modules[0].file, "standard/modules/languages/go.md");
        assert_eq!(c.modules[0].title, "Go");
        assert_eq!(c.modules[0].applies.to_string(), "lang.go");
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
    fn load_unknown_rule_reference() {
        let sec = "standard/core/security.md";
        let src = edited(
            sec,
            "Every project MUST validate external input.",
            "Every project MUST validate external input.
See USS-SEC-099.",
        );
        let text = text_of(&load(&src).unwrap_err());
        assert!(
            text.contains("standard/core/security.md:15: unknown rule USS-SEC-099"),
            "{text}"
        );

        let src = edited(
            sec,
            "Every project MUST validate external input.",
            "Every project MUST validate external input.

```
See USS-SEC-099.
```",
        );
        assert!(load(&src).is_ok());

        let src = edited(
            "standard/applicability.md",
            "Yes when it reads or writes a database.",
            "Yes when it reads or writes a database, as USS-NOPE-001 says.",
        );
        let text = text_of(&load(&src).unwrap_err());
        assert!(text.contains("unknown rule USS-NOPE-001"), "{text}");
    }

    #[test]
    fn load_refines_reach() {
        let go = "standard/modules/languages/go.md";
        let refine = |target: &str, applies: &str| {
            let src = edited(
                go,
                "**Refines:** USS-SEC-001",
                &format!("**Refines:** {target}"),
            );
            let mut m = src;
            let text = String::from_utf8(m.0[go].clone()).unwrap();
            let text = text.replacen(
                "- **Applies when:** always",
                &format!("- **Applies when:** {applies}"),
                1,
            );
            m.0.insert(go.to_string(), text.into_bytes());
            load(&m)
        };
        let text = text_of(&refine("USS-SEC-002", "always").unwrap_err());
        assert!(
            text.contains(
                "USS-GO-001 applies where USS-SEC-002, which it refines, does not (for example when true: lang.go)"
            ),
            "{text}"
        );
        assert!(refine("USS-SEC-002", "uses-database").is_ok());
        assert!(refine("USS-SEC-003", "has-web-ui").is_ok());
    }

    #[test]
    fn refines_reach_over_limit() {
        let extra: Vec<String> = (1..=12).map(|i| format!("f{i:02}")).collect();
        let facts = extra
            .iter()
            .map(|f| {
                format!("### {f}\n\n- **Question:** Is {f}?\n- **Definition:** Yes when {f}.\n\n")
            })
            .collect::<Vec<_>>()
            .concat();
        let mut src = edited(
            "standard/applicability.md",
            "### has-ui\n",
            &format!("{facts}### has-ui\n"),
        );
        let go = "standard/modules/languages/go.md";
        let text = String::from_utf8(src.0[go].clone()).unwrap();
        let wide = format!(
            "has-ui or uses-database or has-web-ui or has-plugin-interface or {}",
            extra.join(" or ")
        );
        let text = text.replacen(
            "- **Applies when:** always",
            &format!("- **Applies when:** {wide}"),
            1,
        );
        src.0.insert(go.to_string(), text.into_bytes());
        let diags = load(&src).unwrap_err();
        assert_eq!(diags.len(), 1, "{diags:?}");
        let text = text_of(&diags);
        assert!(
            text.contains("17 identifiers, over the limit of 16"),
            "{text}"
        );
    }

    #[test]
    fn load_refines_retired_rule() {
        let src = edited(
            "standard/modules/languages/go.md",
            "**Refines:** USS-SEC-001",
            "**Refines:** USS-SEC-004",
        );
        let text = text_of(&load(&src).unwrap_err());
        assert!(
            text.contains(
                "standard/modules/languages/go.md:7: USS-GO-001 refines retired rule USS-SEC-004"
            ),
            "{text}"
        );
    }

    #[test]
    fn load_unknown_refines_reported_once() {
        let src = edited(
            "standard/modules/languages/go.md",
            "**Refines:** USS-SEC-001",
            "**Refines:** USS-SEC-099",
        );
        let diags = load(&src).unwrap_err();
        assert_eq!(
            text_of(&diags),
            "standard/modules/languages/go.md:7: USS-GO-001 refines unknown rule USS-SEC-099"
        );

        // The same unknown ID elsewhere in the file is still reported.
        let src = edited(
            "standard/modules/languages/go.md",
            "**Refines:** USS-SEC-001\n\nGo code MUST handle every returned error.",
            "**Refines:** USS-SEC-099\n\nGo code MUST handle every returned error, as\nUSS-SEC-099 says.",
        );
        let text = text_of(&load(&src).unwrap_err());
        assert_eq!(
            text,
            "standard/modules/languages/go.md:7: USS-GO-001 refines unknown rule USS-SEC-099\nstandard/modules/languages/go.md:17: unknown rule USS-SEC-099"
        );
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
        assert_eq!((c.rules.len(), c.file_count), (6, 3));
    }

    const LINT: &str = "standard/vendors/acme/lint.md";

    /// The fixture profile with a prefix, a condition and one rule that
    /// refines `target`.
    fn profile_with_rule(target: &str) -> MemSource {
        let rule = format!(
            "\n## Rules\n\n### USS-LINT-001: Keep the config\n\n- **Level:** MUST\n- **Applies when:** always\n- **Category:** Safety\n- **Verify:** Check the config.\n- **References:** none\n- **Refines:** {target}\n\nThe config MUST stay.\n"
        );
        let mut m = edited(
            LINT,
            "- **Versions:** >=1.0.0, <2.0.0\n",
            "- **Versions:** >=1.0.0, <2.0.0\n- **Prefix:** LINT\n- **Applies when:** lang.go\n",
        );
        let text = String::from_utf8(m.0[LINT].clone()).unwrap();
        m.0.insert(LINT.to_string(), (text + &rule).into_bytes());
        m
    }

    #[test]
    fn load_profile_with_rules() {
        let c = load(&profile_with_rule("USS-SEC-001")).unwrap();
        assert_eq!((c.rules.len(), c.file_count), (9, 5));
        let rule = c.rule("USS-LINT-001").unwrap();
        assert_eq!(rule.kind, FileKind::Profile);
        assert_eq!(rule.file, LINT);
        let a = rule.active().unwrap();
        assert_eq!(a.applies.to_string(), "lang.go");
        assert_eq!(a.refines, ["USS-SEC-001"]);
        assert_eq!(c.profiles[0].mappings.len(), 2);

        // The reach check covers a profile rule that refines.
        let text = text_of(&load(&profile_with_rule("USS-SEC-002")).unwrap_err());
        assert!(
            text.contains("USS-LINT-001 applies where USS-SEC-002, which it refines, does not"),
            "{text}"
        );
    }

    #[test]
    fn regulation_errors() {
        let law = "standard/modules/regulations/law.md";
        let disclaimer =
            "## Disclaimer\n\nUSS compliance is not legal compliance and not legal advice.\n\n";
        let none_row = "| none | none: a lawyer confirms this |";
        let full_row = "| USS-SEC-003, USS-LAW-001 | full |";
        let cases: Vec<(MemSource, &str)> = vec![
            (
                edited(law, disclaimer, ""),
                "missing section \"Disclaimer\"",
            ),
            (
                edited(
                    law,
                    "USS compliance is not legal compliance and not legal advice.\n",
                    "",
                ),
                "section \"Disclaimer\" is empty",
            ),
            (
                edited(
                    law,
                    "| Obligation | Source | Rules | Coverage |",
                    "| Duty | Source | Rules | Coverage |",
                ),
                "the obligations table header must be \"Obligation | Source | Rules | Coverage\"",
            ),
            (
                edited(
                    law,
                    "## Obligations",
                    "## Obligations\n\nNo table.\n\n## Notes",
                ),
                "no table",
            ),
            (
                edited(law, "## Obligations", "## Duties"),
                "missing section \"Obligations\"",
            ),
            (
                edited(
                    law,
                    "| Escape output | Art. 1 | USS-SEC-003, USS-LAW-001 | full |\n| Appoint a representative | Art. 2 | none | none: a lawyer confirms this |\n",
                    "",
                ),
                "law.md:11: the obligations table has no rows",
            ),
            (
                edited(law, none_row, "| none | full |"),
                "an obligation with no rules has coverage \"full\"; it must be \"none: <what is left>\"",
            ),
            (
                edited(law, none_row, "| none | partial: some |"),
                "an obligation with no rules has coverage \"partial\"",
            ),
            (
                edited(law, full_row, "| USS-SEC-003, USS-LAW-001 | none: x |"),
                "an obligation with rules has coverage \"none\"; it must be full or partial",
            ),
            (
                edited(law, full_row, "| USS-SEC-003 | maybe |"),
                "invalid coverage \"maybe\"",
            ),
            (
                edited(law, "| USS-SEC-003, USS-LAW-001 |", "| USS-SEC-099 |"),
                "unknown rule USS-SEC-099",
            ),
            (
                edited(law, "| USS-SEC-003, USS-LAW-001 |", "| USS-SEC-004 |"),
                "obligation maps retired rule USS-SEC-004",
            ),
            (
                edited(law, "| USS-SEC-003, USS-LAW-001 |", "| soon |"),
                "\"soon\" is not a rule ID",
            ),
            (
                edited(
                    law,
                    "- **References:** Example law Art. 1\n",
                    "- **References:** Example law Art. 1\n- **Refines:** USS-SEC-001\n",
                ),
                "USS-LAW-001 uses \"Refines\"",
            ),
        ];
        for (i, (src, want)) in cases.iter().enumerate() {
            let text = text_of(&load(src).unwrap_err());
            assert!(text.contains(want), "case {i}: want {want:?} in:\n{text}");
        }
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
