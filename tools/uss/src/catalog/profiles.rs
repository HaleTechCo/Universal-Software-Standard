//! Vendor profiles: a tool's baseline configuration and its mapping to rules.

use std::collections::BTreeMap;

use super::markdown::{Doc, Group, Line, parse_doc, parse_table};
use super::rules::{FileKind, PREFIX_RE, RULE_ID_RE, RuleFile, condition, parse_rule};
use super::{Catalog, Diagnostic};
use crate::expr::Expr;

/// The fields a profile's header may have.
const HEADER: [&str; 7] = [
    "Kind",
    "Product",
    "Vendor",
    "Maintainer",
    "Versions",
    "Prefix",
    "Applies when",
];

/// The `##` sections that every profile has.
const SECTIONS: [&str; 3] = [
    "Baseline configuration",
    "Mappings",
    "Check-integrity guards",
];

/// The section that holds the tool-specific rules.
const RULES: &str = "Rules";

/// The header of the mapping table.
const MAPPING_HEADER: [&str; 5] = [
    "Rule",
    "Category",
    "Tool rules or settings",
    "Coverage",
    "How to verify",
];

/// Who maintains a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Maintainer {
    /// Written by USS contributors.
    Community,
    /// Written or taken over by the tool's vendor.
    Vendor,
}

/// How far a tool's configuration satisfies a rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Coverage {
    /// The baseline configuration decides the rule.
    Full,
    /// Something else is needed; the text says what.
    Partial(String),
}

/// One row of a profile's mapping table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mapping {
    /// 1-based line of the row.
    pub line: usize,
    /// The rule ID.
    pub rule: String,
    /// The rule's category name.
    pub category: String,
    /// The tool's rules or settings that satisfy the rule.
    pub tool: String,
    /// How far they satisfy it.
    pub coverage: Coverage,
    /// How an auditor checks it.
    pub verify: String,
}

/// A vendor profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    /// Path of the file, relative to the USS root.
    pub file: String,
    /// The tool's name.
    pub product: String,
    /// The vendor's name.
    pub vendor: String,
    /// Who maintains the profile.
    pub maintainer: Maintainer,
    /// The tool versions that the profile was checked against.
    pub versions: String,
    /// The mapping table, in file order.
    pub mappings: Vec<Mapping>,
}

/// Parses one profile file and runs the checks that need no other file.
/// Returns the profile and its rules as a [`RuleFile`].
pub(crate) fn parse_profile(file: &str, data: &[u8]) -> (RuleFile, Profile, Vec<Diagnostic>) {
    let (doc, mut diags) = parse_doc(file, data);
    let at = |line: usize, message: String| Diagnostic {
        file: file.to_string(),
        line,
        message,
    };
    if !path_ok(file) {
        diags.push(at(
            0,
            "a profile must be at standard/vendors/<vendor>/<product>.md".to_string(),
        ));
    }

    let header = |name: &str| {
        doc.header
            .get(name)
            .map(|f| f.value.clone())
            .filter(|v| !v.is_empty())
    };
    match doc.header.get("Kind") {
        Some(f) if f.value == "profile" => {}
        Some(f) => diags.push(at(
            f.line,
            format!("invalid kind \"{}\" (want profile)", f.value),
        )),
        None => diags.push(at(1, "missing header field \"Kind\"".to_string())),
    }
    for name in ["Product", "Vendor", "Versions"] {
        if header(name).is_none() {
            diags.push(at(1, format!("missing header field \"{name}\"")));
        }
    }
    let maintainer = match doc.header.get("Maintainer") {
        Some(f) if f.value == "community" => Maintainer::Community,
        Some(f) if f.value == "vendor" => Maintainer::Vendor,
        Some(f) => {
            diags.push(at(
                f.line,
                format!(
                    "invalid maintainer \"{}\" (want community or vendor)",
                    f.value
                ),
            ));
            Maintainer::Community
        }
        None => {
            diags.push(at(1, "missing header field \"Maintainer\"".to_string()));
            Maintainer::Community
        }
    };
    for (name, f) in &doc.header {
        if !HEADER.contains(&name.as_str()) {
            diags.push(at(f.line, format!("unknown header field \"{name}\"")));
        }
    }

    let rules_at = check_sections(&doc, &at, &mut diags);
    let (prefix, applies_when, file_applies) =
        rule_header(&doc, rules_at.is_some(), &at, &mut diags);
    let rules = doc
        .sections
        .iter()
        .filter_map(|s| {
            parse_rule(
                s,
                file,
                &prefix,
                FileKind::Profile,
                &file_applies,
                &at,
                &mut diags,
            )
        })
        .collect();

    let mappings = doc
        .groups
        .iter()
        .find(|g| g.heading == "Mappings")
        .map_or_default(|g| parse_mappings(g, &at, &mut diags));

    let profile = Profile {
        file: file.to_string(),
        product: header("Product").unwrap_or_default(),
        vendor: header("Vendor").unwrap_or_default(),
        maintainer,
        versions: header("Versions").unwrap_or_default(),
        mappings,
    };
    let rule_file = RuleFile {
        file: file.to_string(),
        title: doc.title,
        prefix,
        kind: FileKind::Profile,
        applies_when,
        rules,
    };
    (rule_file, profile, diags)
}

/// Tells whether `path` is `standard/vendors/<vendor>/<product>.md`.
fn path_ok(path: &str) -> bool {
    path.strip_prefix("standard/vendors/").is_some_and(|rest| {
        let parts: Vec<&str> = rest.split('/').collect();
        parts.len() == 2 && parts.iter().all(|p| !p.is_empty())
    })
}

/// Checks the `##` sections and where the rules are. Returns the line of the
/// `## Rules` heading when the profile has rules.
fn check_sections(
    doc: &Doc,
    at: &dyn Fn(usize, String) -> Diagnostic,
    diags: &mut Vec<Diagnostic>,
) -> Option<usize> {
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for g in &doc.groups {
        let name = g.heading.as_str();
        if !SECTIONS.contains(&name) && name != RULES {
            diags.push(at(g.line, format!("unknown section \"{name}\"")));
        } else if seen.insert(name, g.line).is_some() {
            diags.push(at(g.line, format!("duplicate section \"{name}\"")));
        }
    }
    for name in SECTIONS {
        if !seen.contains_key(name) {
            diags.push(at(1, format!("missing section \"{name}\"")));
        }
    }
    if let Some(g) = doc
        .groups
        .iter()
        .find(|g| g.heading == "Baseline configuration")
    {
        let blocks = code_blocks(&g.lines);
        if blocks != 1 {
            diags.push(at(
                g.line,
                format!(
                    "\"Baseline configuration\" must have exactly one code block (found {blocks})"
                ),
            ));
        }
    }

    let rules_at = seen.get(RULES).copied();
    match (rules_at, doc.sections.first()) {
        (Some(line), None) => diags.push(at(line, "section \"Rules\" has no rules".to_string())),
        (_, Some(first)) => {
            for s in &doc.sections {
                let owner = doc.groups.iter().rev().find(|g| g.line < s.line);
                if owner.is_none_or(|g| g.heading != RULES) {
                    diags.push(at(
                        s.line,
                        "a rule must be under the \"Rules\" section".to_string(),
                    ));
                }
            }
            if rules_at.is_none() {
                diags.push(at(first.line, "rules need a \"Rules\" section".to_string()));
            }
        }
        (None, None) => {}
    }
    rules_at
}

/// Counts the fenced code blocks in `lines`.
fn code_blocks(lines: &[Line]) -> usize {
    let mut count = 0;
    let mut open: Option<&str> = None;
    for l in lines.iter().filter(|l| l.fenced) {
        match open {
            Some(marker) if l.text.starts_with(marker) => open = None,
            Some(_) => {}
            None => {
                open = ["```", "~~~"].into_iter().find(|m| l.text.starts_with(m));
                count += 1;
            }
        }
    }
    count
}

/// Reads `Prefix` and `Applies when`, which only a profile with rules has.
fn rule_header(
    doc: &Doc,
    has_rules: bool,
    at: &dyn Fn(usize, String) -> Diagnostic,
    diags: &mut Vec<Diagnostic>,
) -> (String, String, Expr) {
    let has_sections = !doc.sections.is_empty();
    for name in ["Prefix", "Applies when"] {
        match doc.header.get(name) {
            Some(f) if !has_rules && !has_sections => diags.push(at(
                f.line,
                format!("header field \"{name}\" is only allowed in a profile with rules"),
            )),
            None if has_sections => diags.push(at(
                doc.sections[0].line,
                format!("a profile with rules needs the header field \"{name}\""),
            )),
            _ => {}
        }
    }
    let prefix = match doc.header.get("Prefix") {
        Some(f) if PREFIX_RE.is_match(&f.value) => f.value.clone(),
        Some(f) => {
            diags.push(at(f.line, format!("invalid prefix \"{}\"", f.value)));
            String::new()
        }
        None => String::new(),
    };
    let (applies_when, applies) = condition(doc.header.get("Applies when"), at, diags);
    (prefix, applies_when, applies)
}

/// Reads the mapping table of the `## Mappings` section.
fn parse_mappings(
    g: &Group,
    at: &dyn Fn(usize, String) -> Diagnostic,
    diags: &mut Vec<Diagnostic>,
) -> Vec<Mapping> {
    let table = match parse_table(&g.lines) {
        Ok(t) => t,
        Err((line, message)) => {
            diags.push(at(if line == 0 { g.line } else { line }, message));
            return Vec::new();
        }
    };
    if table.header != MAPPING_HEADER {
        diags.push(at(
            g.line,
            format!(
                "the mapping table header must be \"{}\"",
                MAPPING_HEADER.join(" | ")
            ),
        ));
        return Vec::new();
    }
    let mut out = Vec::new();
    for row in table.rows {
        let [rule, category, tool, coverage, verify] = &row.cells[..] else {
            continue;
        };
        for (cell, name) in [(tool, MAPPING_HEADER[2]), (verify, MAPPING_HEADER[4])] {
            if cell.is_empty() {
                diags.push(at(row.line, format!("empty \"{name}\" cell for {rule}")));
            }
        }
        let coverage = if coverage == "full" {
            Coverage::Full
        } else if let Some(rest) = coverage.strip_prefix("partial:")
            && !rest.trim().is_empty()
        {
            Coverage::Partial(rest.trim().to_string())
        } else {
            diags.push(at(
                row.line,
                format!(
                    "invalid coverage \"{coverage}\" for {rule} (want full or partial: <what else is needed>)"
                ),
            ));
            continue;
        };
        out.push(Mapping {
            line: row.line,
            rule: rule.clone(),
            category: category.clone(),
            tool: tool.clone(),
            coverage,
            verify: verify.clone(),
        });
    }
    out
}

/// Checks the mappings of `profile` against the rules and the registry.
pub(crate) fn check_mappings(catalog: &Catalog, profile: &Profile, diags: &mut Vec<Diagnostic>) {
    let mut first: BTreeMap<&str, usize> = BTreeMap::new();
    let mut prev: Option<(usize, &str)> = None;
    for m in &profile.mappings {
        let mut report = |message: String| {
            diags.push(Diagnostic {
                file: profile.file.clone(),
                line: m.line,
                message,
            });
        };
        if !RULE_ID_RE.is_match(&m.rule) {
            report(format!("\"{}\" is not a rule ID", m.rule));
            continue;
        }
        // The check of the rule IDs in the text reports an unknown rule.
        let Some(rule) = catalog.rule(&m.rule) else {
            continue;
        };
        let Some(active) = rule.active() else {
            report(format!("mapping to retired rule {}", m.rule));
            continue;
        };
        if active.category != m.category {
            report(format!(
                "{} is in category \"{}\", not \"{}\"",
                m.rule, active.category, m.category
            ));
        }
        if let Some(line) = first.get(m.rule.as_str()) {
            report(format!("{} is mapped twice (first at line {line})", m.rule));
            continue;
        }
        first.insert(&m.rule, m.line);
        let Some(index) = catalog
            .registry
            .categories
            .iter()
            .position(|c| c.name == m.category)
        else {
            continue;
        };
        if prev.is_some_and(|p| p > (index, m.rule.as_str())) {
            report(format!(
                "{} is out of order: rows go by registry category order, then by rule ID",
                m.rule
            ));
        }
        prev = Some((index, &m.rule));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::load;
    use crate::source::MemSource;
    use crate::testfixture;

    const LINT: &str = "standard/vendors/acme/lint.md";

    fn text_of(src: &MemSource) -> String {
        match load(src) {
            Ok(_) => String::new(),
            Err(d) => d
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }

    /// The fixture with `from` replaced by `to` in the profile.
    fn edited(from: &str, to: &str) -> MemSource {
        let mut m = testfixture::standard();
        let text = String::from_utf8(m.0[LINT].clone()).unwrap();
        assert!(text.contains(from), "fixture lacks {from:?}");
        m.0.insert(LINT.to_string(), text.replace(from, to).into_bytes());
        m
    }

    #[test]
    fn profile_valid() {
        let c = load(&testfixture::standard()).unwrap();
        assert_eq!(c.profiles.len(), 1);
        let p = &c.profiles[0];
        assert_eq!(p.file, LINT);
        assert_eq!((p.product.as_str(), p.vendor.as_str()), ("Lint", "Acme"));
        assert_eq!(p.maintainer, Maintainer::Community);
        assert_eq!(p.versions, ">=1.0.0, <2.0.0");
        assert_eq!(p.mappings.len(), 2);
        assert_eq!(p.mappings[0].rule, "USS-SEC-001");
        assert_eq!(p.mappings[0].category, "Safety");
        assert_eq!(p.mappings[0].coverage, Coverage::Full);
        assert_eq!(p.mappings[0].tool, "`select = [\"ALL\"]`");
        assert_eq!(p.mappings[1].rule, "USS-EXT-002");
        assert_eq!(
            p.mappings[1].coverage,
            Coverage::Partial("a reviewer checks the plugin rights".to_string())
        );
        assert_eq!(p.mappings[1].line, 21);
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "one table row per check keeps every pinned message in one place"
    )]
    fn profile_errors() {
        let row1 = "| USS-SEC-001 | Safety | `select = [\"ALL\"]` | full | Run `lint` and read the report. |";
        let row2 = "| USS-EXT-002 | Features | `plugin-rights` | partial: a reviewer checks the plugin rights | Read the plugin settings. |";
        let mut moved = testfixture::standard();
        let data = moved.0.remove(LINT).unwrap();
        moved
            .0
            .insert("standard/vendors/lint.md".to_string(), data.clone());
        let mut deep = testfixture::standard();
        deep.0.remove(LINT);
        deep.0
            .insert("standard/vendors/acme/x/lint.md".to_string(), data);
        let rule = "\n## Rules\n\n### USS-LINT-001: Rule\n\n- **Level:** MUST\n- **Applies when:** always\n- **Category:** Safety\n- **Verify:** V.\n- **References:** none\n\nIt MUST hold.\n";
        let mut with_rule = testfixture::standard();
        let text = String::from_utf8(with_rule.0[LINT].clone()).unwrap();
        with_rule
            .0
            .insert(LINT.to_string(), (text + rule).into_bytes());

        let cases: Vec<(MemSource, &str)> = vec![
            (
                moved,
                "standard/vendors/lint.md: a profile must be at standard/vendors/<vendor>/<product>.md",
            ),
            (
                deep,
                "a profile must be at standard/vendors/<vendor>/<product>.md",
            ),
            (
                edited(
                    "- **Product:** Lint\n",
                    "- **Product:** Lint\n- **Extra:** x\n",
                ),
                "unknown header field \"Extra\"",
            ),
            (
                edited("**Kind:** profile", "**Kind:** module"),
                "invalid kind \"module\" (want profile)",
            ),
            (
                edited("- **Vendor:** Acme\n", ""),
                "missing header field \"Vendor\"",
            ),
            (
                edited("**Maintainer:** community", "**Maintainer:** someone"),
                "invalid maintainer \"someone\" (want community or vendor)",
            ),
            (
                edited("- **Versions", "- **Prefix:** LINT\n- **Versions"),
                "header field \"Prefix\" is only allowed in a profile with rules",
            ),
            (
                edited(
                    "## Check-integrity guards\n\n- `unused-ignore` reports an ignore that suppresses nothing.\n",
                    "",
                ),
                "missing section \"Check-integrity guards\"",
            ),
            (
                edited(
                    "## Check-integrity guards",
                    "## Notes\n\nx\n\n## Check-integrity guards",
                ),
                "unknown section \"Notes\"",
            ),
            (
                edited(
                    "```\n\n## Mappings",
                    "```\n\n```toml\nb = 1\n```\n\n## Mappings",
                ),
                "\"Baseline configuration\" must have exactly one code block (found 2)",
            ),
            (
                edited("```toml\n[lint]\nselect = [\"ALL\"]\n```\n", ""),
                "\"Baseline configuration\" must have exactly one code block (found 0)",
            ),
            (
                edited("| Rule | Category |", "| Rule | Kind |"),
                "the mapping table header must be \"Rule | Category | Tool rules or settings | Coverage | How to verify\"",
            ),
            (
                edited("| USS-SEC-001 |", "| USS-NOPE-001 |"),
                "unknown rule USS-NOPE-001",
            ),
            (
                edited("| USS-SEC-001 |", "| SEC-1 |"),
                "\"SEC-1\" is not a rule ID",
            ),
            (
                edited("| USS-SEC-001 |", "| USS-SEC-004 |"),
                "mapping to retired rule USS-SEC-004",
            ),
            (
                edited("| USS-EXT-002 | Features |", "| USS-EXT-002 | Safety |"),
                "USS-EXT-002 is in category \"Features\", not \"Safety\"",
            ),
            (
                edited(&format!("{row1}\n{row2}"), &format!("{row2}\n{row1}")),
                "USS-SEC-001 is out of order: rows go by registry category order, then by rule ID",
            ),
            (
                edited(row2, row1),
                "USS-SEC-001 is mapped twice (first at line 20)",
            ),
            (
                edited("| full |", "| partly |"),
                "invalid coverage \"partly\" for USS-SEC-001",
            ),
            (
                edited(
                    "| partial: a reviewer checks the plugin rights |",
                    "| partial: |",
                ),
                "invalid coverage \"partial:\" for USS-EXT-002",
            ),
            (
                edited("| Read the plugin settings. |", "|  |"),
                "empty \"How to verify\" cell for USS-EXT-002",
            ),
            (
                edited("| `plugin-rights` |", "|  |"),
                "empty \"Tool rules or settings\" cell for USS-EXT-002",
            ),
            (
                edited(
                    "| USS-EXT-002 | Features | `plugin-rights` | partial: a reviewer checks the plugin rights | Read the plugin settings. |\n",
                    "| USS-EXT-002 | Features | x | full |\n",
                ),
                "table row has 4 cells, the header has 5",
            ),
            (
                with_rule,
                "a profile with rules needs the header field \"Prefix\"",
            ),
        ];
        for (i, (src, want)) in cases.iter().enumerate() {
            let text = text_of(src);
            assert!(text.contains(want), "case {i}: want {want:?} in:\n{text}");
        }
    }
}
