//! A standard rule file: header, rules and the checks that need no other file.

use std::fmt;
use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;

use super::Diagnostic;
use super::markdown::{Field, Section, parse_doc};
use super::registry::{VERSION_RE, constant_regex};
use crate::expr::{self, Expr};

static PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| constant_regex(r"^[A-Z][A-Z0-9]{1,7}$"));
static HEADING_RE: LazyLock<Regex> =
    LazyLock::new(|| constant_regex(r"^(USS-([A-Z][A-Z0-9]{1,7})-(\d{3})): (.+)$"));
static RULE_ID_RE: LazyLock<Regex> =
    LazyLock::new(|| constant_regex(r"^USS-[A-Z][A-Z0-9]{1,7}-\d{3}$"));
static KEYWORD_RE: LazyLock<Regex> = LazyLock::new(|| {
    constant_regex(
        r"\b(MUST NOT|SHOULD NOT|SHALL NOT|NOT RECOMMENDED|MUST|SHOULD|SHALL|MAY|REQUIRED|RECOMMENDED|OPTIONAL)\b",
    )
});

/// Keywords the standard never uses.
const FORBIDDEN: [&str; 6] = [
    "SHALL NOT",
    "SHALL",
    "NOT RECOMMENDED",
    "REQUIRED",
    "RECOMMENDED",
    "OPTIONAL",
];

/// The fields a rule section may have.
const ALLOWED: [&str; 10] = [
    "Level",
    "Applies when",
    "Category",
    "Verify",
    "References",
    "Enables",
    "Refines",
    "Status",
    "Retired in",
    "Replaced by",
];

/// The requirement level of a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// `MUST`
    Must,
    /// `MUST NOT`
    MustNot,
    /// `SHOULD`
    Should,
    /// `SHOULD NOT`
    ShouldNot,
    /// `MAY`
    May,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Must => "MUST",
            Self::MustNot => "MUST NOT",
            Self::Should => "SHOULD",
            Self::ShouldNot => "SHOULD NOT",
            Self::May => "MAY",
        })
    }
}

impl FromStr for Level {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MUST" => Ok(Self::Must),
            "MUST NOT" => Ok(Self::MustNot),
            "SHOULD" => Ok(Self::Should),
            "SHOULD NOT" => Ok(Self::ShouldNot),
            "MAY" => Ok(Self::May),
            _ => Err(format!("invalid level \"{s}\"")),
        }
    }
}

/// Whether a rule file covers an area or a module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    /// A core area file.
    Area,
    /// A module file.
    Module,
}

/// The content of an active rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Active {
    /// The requirement level.
    pub level: Level,
    /// The rule's own `Applies when` text.
    pub applies_when: String,
    /// The file's condition and the rule's, combined with `and`.
    pub applies: Expr,
    /// The score category name.
    pub category: String,
    /// How to check the rule.
    pub verify: String,
    /// Sources the rule comes from.
    pub references: String,
    /// IDs of the facts a `MAY` rule enables.
    pub enables: Vec<String>,
    /// IDs of the rules a module rule refines.
    pub refines: Vec<String>,
    /// The rule text after the field block.
    pub body: String,
}

/// An active or a retired rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleState {
    /// The rule is in force.
    Active(Active),
    /// The rule is retired in place; its ID is never reused.
    Retired {
        /// The standard version that retired it.
        retired_in: String,
        /// The rule that replaces it, if any.
        replaced_by: Option<String>,
    },
}

/// One rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    /// The rule ID, such as `USS-SEC-012`.
    pub id: String,
    /// The title after the ID.
    pub title: String,
    /// The kind of file the rule is in.
    pub kind: FileKind,
    /// Path of the file.
    pub file: String,
    /// 1-based line of the rule's heading.
    pub line: usize,
    /// Active or retired.
    pub state: RuleState,
}

impl Rule {
    /// The rule's content, or `None` when it is retired.
    #[must_use]
    pub fn active(&self) -> Option<&Active> {
        match &self.state {
            RuleState::Active(a) => Some(a),
            RuleState::Retired { .. } => None,
        }
    }
}

/// A parsed rule file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleFile {
    /// Path of the file.
    pub file: String,
    /// Text of the `#` title.
    pub title: String,
    /// The prefix of the rule IDs in the file.
    pub prefix: String,
    /// Area or module.
    pub kind: FileKind,
    /// The file's `Applies when` text (`always` when absent).
    pub applies_when: String,
    /// The rules, in file order.
    pub rules: Vec<Rule>,
}

type At<'a> = &'a dyn Fn(usize, String) -> Diagnostic;

/// Splits a comma-separated field value.
fn list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_string)
        .collect()
}

/// Parses one rule file and runs the checks that need no other file.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "the catalog loader uses it from the next task")
)]
pub(crate) fn parse_rule_file(file: &str, data: &[u8]) -> (RuleFile, Vec<Diagnostic>) {
    let (doc, mut diags) = parse_doc(file, data);
    let at = |line: usize, message: String| Diagnostic {
        file: file.to_string(),
        line,
        message,
    };

    let prefix = match doc.header.get("Prefix") {
        Some(f) if PREFIX_RE.is_match(&f.value) => f.value.clone(),
        Some(f) => {
            diags.push(at(f.line, format!("invalid prefix \"{}\"", f.value)));
            String::new()
        }
        None => {
            diags.push(at(1, "missing header field \"Prefix\"".to_string()));
            String::new()
        }
    };
    let kind = match doc.header.get("Kind") {
        Some(f) if f.value == "area" => FileKind::Area,
        Some(f) if f.value == "module" => FileKind::Module,
        Some(f) => {
            diags.push(at(
                f.line,
                format!("invalid kind \"{}\" (want area or module)", f.value),
            ));
            FileKind::Area
        }
        None => {
            diags.push(at(1, "missing header field \"Kind\"".to_string()));
            FileKind::Area
        }
    };
    let (applies_when, file_applies) = condition(doc.header.get("Applies when"), &at, &mut diags);

    let mut rules = Vec::new();
    for s in &doc.sections {
        if let Some(rule) = parse_rule(s, file, &prefix, kind, &file_applies, &at, &mut diags) {
            rules.push(rule);
        }
    }
    let rf = RuleFile {
        file: file.to_string(),
        title: doc.title,
        prefix,
        kind,
        applies_when,
        rules,
    };
    (rf, diags)
}

/// Reads an optional `Applies when` field: its text and parsed expression.
fn condition(field: Option<&Field>, at: At<'_>, diags: &mut Vec<Diagnostic>) -> (String, Expr) {
    let Some(f) = field else {
        return ("always".to_string(), Expr::Always);
    };
    match expr::parse(&f.value) {
        Ok(e) => (f.value.clone(), e),
        Err(err) => {
            diags.push(at(
                f.line,
                format!("invalid \"Applies when\" \"{}\": {err}", f.value),
            ));
            (f.value.clone(), Expr::Always)
        }
    }
}

/// Parses and checks one `###` section.
fn parse_rule(
    s: &Section,
    file: &str,
    prefix: &str,
    kind: FileKind,
    file_applies: &Expr,
    at: At<'_>,
    diags: &mut Vec<Diagnostic>,
) -> Option<Rule> {
    let Some(caps) = HEADING_RE.captures(&s.heading) else {
        diags.push(at(
            s.line,
            format!("heading is not a rule: \"{}\"", s.heading),
        ));
        return None;
    };
    let id = caps[1].to_string();
    let title = caps[4].to_string();
    if !prefix.is_empty() && &caps[2] != prefix {
        diags.push(at(
            s.line,
            format!("rule {id} does not use the file's prefix \"{prefix}\""),
        ));
    }
    for (name, f) in &s.fields {
        if !ALLOWED.contains(&name.as_str()) {
            diags.push(at(f.line, format!("unknown field \"{name}\"")));
        }
    }
    let get = |name: &str| s.fields.get(name).map(|f| f.value.clone());
    let make = |state| Rule {
        id: id.clone(),
        title: title.clone(),
        kind,
        file: file.to_string(),
        line: s.line,
        state,
    };

    if let Some(f) = s.fields.get("Status") {
        match f.value.as_str() {
            "Retired" => return Some(make(retired(s, &id, at, diags))),
            "Active" => {}
            other => diags.push(at(f.line, format!("invalid status \"{other}\""))),
        }
    }

    for name in ["Level", "Applies when", "Category", "Verify", "References"] {
        if !s.fields.contains_key(name) {
            diags.push(at(s.line, format!("missing field \"{name}\"")));
        }
    }
    if s.body.is_empty() {
        diags.push(at(s.line, "empty rule body".to_string()));
    }
    let level = match s.fields.get("Level") {
        Some(f) => match f.value.parse::<Level>() {
            Ok(l) => Some(l),
            Err(e) => {
                diags.push(at(f.line, e));
                None
            }
        },
        None => None,
    }?;

    let (applies_when, rule_applies) = match s.fields.get("Applies when") {
        Some(f) => condition(Some(f), at, diags),
        None => (String::new(), Expr::Always),
    };
    let enables = s.fields.get("Enables");
    if let Some(f) = enables
        && level != Level::May
    {
        diags.push(at(
            f.line,
            "\"Enables\" is only allowed on a MAY rule".to_string(),
        ));
    }
    let refines = s.fields.get("Refines");
    if let Some(f) = refines
        && kind != FileKind::Module
    {
        diags.push(at(
            f.line,
            "\"Refines\" is only allowed in a module file".to_string(),
        ));
    }
    scan_keywords(s, level, at, diags);

    Some(make(RuleState::Active(Active {
        level,
        applies_when,
        applies: Expr::and(file_applies.clone(), rule_applies),
        category: get("Category").unwrap_or_default(),
        verify: get("Verify").unwrap_or_default(),
        references: get("References").unwrap_or_default(),
        enables: enables.map_or_else(Vec::new, |f| list(&f.value)),
        refines: refines.map_or_else(Vec::new, |f| list(&f.value)),
        body: s.body.clone(),
    })))
}

/// Checks the fields of a retired rule.
fn retired(s: &Section, id: &str, at: At<'_>, diags: &mut Vec<Diagnostic>) -> RuleState {
    let retired_in = match s.fields.get("Retired in") {
        Some(f) if VERSION_RE.is_match(&f.value) => f.value.clone(),
        Some(f) => {
            diags.push(at(
                f.line,
                format!("invalid version \"{}\" in \"Retired in\"", f.value),
            ));
            f.value.clone()
        }
        None => {
            diags.push(at(
                s.line,
                format!("retired rule {id} needs \"Retired in\""),
            ));
            String::new()
        }
    };
    let replaced_by = match s.fields.get("Replaced by") {
        Some(f) if f.value == "none" => None,
        Some(f) if RULE_ID_RE.is_match(&f.value) => Some(f.value.clone()),
        Some(f) => {
            diags.push(at(
                f.line,
                format!(
                    "invalid \"Replaced by\" \"{}\" (want a rule ID or none)",
                    f.value
                ),
            ));
            None
        }
        None => {
            diags.push(at(
                s.line,
                format!("retired rule {id} needs \"Replaced by\""),
            ));
            None
        }
    };
    RuleState::Retired {
        retired_in,
        replaced_by,
    }
}

/// Checks the requirement keywords in the prose of an active rule.
fn scan_keywords(s: &Section, level: Level, at: At<'_>, diags: &mut Vec<Diagnostic>) {
    let own = level.to_string();
    let mut seen: Vec<&str> = Vec::new();
    let mut found_own = false;
    // Hard-wrapped text may split "MUST NOT" across lines.
    let text = s.prose.split_whitespace().collect::<Vec<_>>().join(" ");
    for m in KEYWORD_RE.find_iter(&text) {
        let kw = m.as_str();
        if kw == own {
            found_own = true;
        } else if !seen.contains(&kw) {
            seen.push(kw);
            let message = if FORBIDDEN.contains(&kw) {
                format!("forbidden keyword \"{kw}\"")
            } else {
                format!("keyword \"{kw}\" in a {own} rule")
            };
            diags.push(at(s.line, message));
        }
    }
    if !found_own {
        diags.push(at(
            s.line,
            format!("body does not state the rule with \"{own}\""),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = "# Security\n\n- **Prefix:** SEC\n- **Kind:** area\n\n### USS-SEC-012: Use parameterized queries\n\n- **Level:** MUST\n- **Applies when:** uses-database\n- **Category:** Safety\n- **Verify:** No query is built by concatenating external input; all queries\n  use bound parameters, or a query builder that binds them.\n- **References:** OWASP ASVS 5.0 (injection prevention); CWE-89\n\nEvery database query that includes externally supplied values MUST pass those\nvalues as bound parameters.\n\n**Rationale:** Bound parameters keep external input from being run as SQL.\n";

    fn run(src: &str) -> (RuleFile, String) {
        let (rf, diags) = parse_rule_file("standard/core/sec.md", src.as_bytes());
        let text = diags
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        (rf, text)
    }

    /// A file with one rule section made of `fields` and `body`.
    fn rule(kind: &str, fields: &str, body: &str) -> String {
        format!(
            "# T\n\n- **Prefix:** SEC\n- **Kind:** {kind}\n\n### USS-SEC-001: Title\n\n{fields}\n{body}\n"
        )
    }

    const FIELDS: &str = "- **Level:** MUST\n- **Applies when:** always\n- **Category:** Safety\n- **Verify:** V.\n- **References:** R.\n";

    #[test]
    fn rule_file_valid() {
        let (rf, text) = run(VALID);
        assert_eq!(text, "");
        assert_eq!(rf.prefix, "SEC");
        assert_eq!(rf.kind, FileKind::Area);
        assert_eq!(rf.applies_when, "always");
        assert_eq!(rf.title, "Security");
        assert_eq!(rf.rules.len(), 1);
        let r = &rf.rules[0];
        assert_eq!(r.id, "USS-SEC-012");
        assert_eq!(r.title, "Use parameterized queries");
        assert_eq!(r.line, 6);
        assert_eq!(r.file, "standard/core/sec.md");
        let a = r.active().unwrap();
        assert_eq!(a.level, Level::Must);
        assert_eq!(a.applies_when, "uses-database");
        assert_eq!(a.applies.to_string(), "uses-database");
        assert_eq!(a.category, "Safety");
        assert_eq!(
            a.verify,
            "No query is built by concatenating external input; all queries use bound parameters, or a query builder that binds them."
        );
        assert_eq!(
            a.references,
            "OWASP ASVS 5.0 (injection prevention); CWE-89"
        );
        assert!(a.enables.is_empty() && a.refines.is_empty());
        assert!(a.body.starts_with("Every database query"), "{}", a.body);
        assert!(a.body.ends_with("as SQL."), "{}", a.body);
    }

    #[test]
    fn rule_file_module_applies_when() {
        let src = rule("module", FIELDS, "It MUST hold.").replace(
            "- **Kind:** module\n",
            "- **Kind:** module\n- **Applies when:** lang.go\n",
        );
        let (rf, text) = run(&src);
        assert_eq!(text, "");
        assert_eq!(rf.applies_when, "lang.go");
        let a = rf.rules[0].active().unwrap();
        assert_eq!(a.applies.to_string(), "lang.go");
        assert_eq!(a.applies_when, "always");
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "one table row per check keeps every pinned message in one place"
    )]
    fn rule_file_errors() {
        let ok = "It MUST hold.";
        let may_fields = FIELDS.replace("MUST", "MAY");
        let cases: Vec<(String, &str)> = vec![
            (VALID.replace("- **Prefix:** SEC\n", ""), "\"Prefix\""),
            (VALID.replace("SEC\n", "sec\n"), "invalid prefix"),
            (VALID.replace("- **Kind:** area\n", ""), "\"Kind\""),
            (VALID.replace("area\n", "other\n"), "invalid kind"),
            (
                VALID.replace(
                    "- **Kind:** area\n",
                    "- **Kind:** area\n- **Applies when:** a and\n",
                ),
                "invalid \"Applies when\"",
            ),
            (
                rule("area", &FIELDS.replace("always", "a or"), ok),
                "invalid \"Applies when\"",
            ),
            (
                VALID.replace("### USS-SEC-012:", "### Not a rule"),
                "heading is not a rule",
            ),
            (
                VALID.replace("USS-SEC-012", "USS-DB-012"),
                "does not use the file's prefix",
            ),
            (
                rule("area", &format!("{FIELDS}- **Colour:** red\n"), ok),
                "unknown field \"Colour\"",
            ),
            (
                rule("area", &FIELDS.replace("- **Level:** MUST\n", ""), ok),
                "missing field \"Level\"",
            ),
            (
                rule("area", &FIELDS.replace("- **Category:** Safety\n", ""), ok),
                "missing field \"Category\"",
            ),
            (
                rule("area", &FIELDS.replace("- **Verify:** V.\n", ""), ok),
                "missing field \"Verify\"",
            ),
            (
                rule("area", &FIELDS.replace("- **References:** R.\n", ""), ok),
                "missing field \"References\"",
            ),
            (
                rule(
                    "area",
                    &FIELDS.replace("- **Applies when:** always\n", ""),
                    ok,
                ),
                "missing field \"Applies when\"",
            ),
            (
                rule("area", &FIELDS.replace("MUST", "WANT"), ok),
                "invalid level",
            ),
            (rule("area", FIELDS, ""), "empty rule body"),
            (
                rule("area", &format!("{FIELDS}- **Enables:** has-ui\n"), ok),
                "\"Enables\" is only allowed on a MAY rule",
            ),
            (
                rule(
                    "area",
                    &format!("{may_fields}- **Refines:** USS-SEC-002\n"),
                    "It MAY hold.",
                ),
                "\"Refines\" is only allowed in a module file",
            ),
            (
                rule("area", FIELDS, "It SHALL hold. It MUST hold."),
                "forbidden keyword \"SHALL\"",
            ),
            (
                rule("area", FIELDS, "It is REQUIRED. It MUST hold."),
                "forbidden keyword \"REQUIRED\"",
            ),
            (
                rule("area", FIELDS, "It is NOT RECOMMENDED. It MUST hold."),
                "forbidden keyword \"NOT RECOMMENDED\"",
            ),
            (
                rule("area", FIELDS, "It MUST hold and SHOULD be fast."),
                "keyword \"SHOULD\" in a MUST rule",
            ),
            (
                rule("area", FIELDS, "It MUST NOT fail, and MUST hold."),
                "keyword \"MUST NOT\" in a MUST rule",
            ),
            (
                rule("area", FIELDS, "It holds."),
                "body does not state the rule with \"MUST\"",
            ),
            (
                rule("area", FIELDS, "It SHOULD hold."),
                "body does not state the rule with \"MUST\"",
            ),
        ];
        for (i, (src, want)) in cases.iter().enumerate() {
            let (_, text) = run(src);
            assert!(text.contains(want), "case {i}: want {want:?} in {text:?}");
        }
    }

    #[test]
    fn rule_file_wrapped_keywords() {
        let must_not = FIELDS.replace("MUST", "MUST NOT");
        let (_, text) = run(&rule("area", &must_not, "It MUST\nNOT fail."));
        assert_eq!(text, "");
        let (_, text) = run(&rule("area", FIELDS, "It MUST hold, not MUST\nNOT fail."));
        assert!(
            text.contains("keyword \"MUST NOT\" in a MUST rule"),
            "{text}"
        );
        let should = FIELDS.replace("MUST", "SHOULD");
        let (_, text) = run(&rule("area", &should, "It SHOULD hold, not SHOULD\n  NOT."));
        assert!(
            text.contains("keyword \"SHOULD NOT\" in a SHOULD rule"),
            "{text}"
        );
    }

    #[test]
    fn rule_file_diagnostic_lines() {
        let (_, text) = run(&rule(
            "area",
            &format!("{FIELDS}- **Colour:** red\n"),
            "It MUST.",
        ));
        assert!(text.contains("sec.md:13: unknown field"), "{text}");
        let (_, text) = run(&rule("area", FIELDS, ""));
        assert!(text.contains("sec.md:6: empty rule body"), "{text}");
    }

    #[test]
    fn rule_file_code_fence() {
        let body = "It MUST hold.\n\n```text\n### USS-SEC-999: x\nThis SHOULD be ignored. REQUIRED.\n```\n";
        let (rf, text) = run(&rule("area", FIELDS, body));
        assert_eq!(text, "");
        assert_eq!(rf.rules.len(), 1);
        assert!(rf.rules[0].active().unwrap().body.contains("USS-SEC-999"));
    }

    #[test]
    fn rule_file_crlf_and_bom() {
        let (a, text_a) = run(VALID);
        let crlf = format!("\u{feff}{}", VALID.replace('\n', "\r\n"));
        let (b, text_b) = run(&crlf);
        assert_eq!(text_a, "");
        assert_eq!(text_b, "");
        assert_eq!(a.rules.len(), 1);
        assert_eq!(a.rules, b.rules);
    }

    #[test]
    fn rule_file_retired() {
        let head = "# T\n\n- **Prefix:** SEC\n- **Kind:** area\n\n### USS-SEC-005: Old rule\n\n";
        let (rf, text) = run(&format!(
            "{head}- **Status:** Retired\n- **Retired in:** 0.2.0\n- **Replaced by:** USS-SEC-006\n"
        ));
        assert_eq!(text, "");
        assert!(rf.rules[0].active().is_none());
        assert_eq!(
            rf.rules[0].state,
            RuleState::Retired {
                retired_in: "0.2.0".to_string(),
                replaced_by: Some("USS-SEC-006".to_string())
            }
        );
        let (rf, text) = run(&format!(
            "{head}- **Status:** Retired\n- **Retired in:** 0.2.0\n- **Replaced by:** none\n"
        ));
        assert_eq!(text, "");
        assert_eq!(
            rf.rules[0].state,
            RuleState::Retired {
                retired_in: "0.2.0".to_string(),
                replaced_by: None
            }
        );
        let (_, text) = run(&format!(
            "{head}- **Status:** Retired\n- **Retired in:** 0.2.0\n"
        ));
        assert!(text.contains("needs \"Replaced by\""), "{text}");
        let (_, text) = run(&format!(
            "{head}- **Status:** Retired\n- **Replaced by:** none\n"
        ));
        assert!(text.contains("needs \"Retired in\""), "{text}");
        let (_, text) = run(&format!(
            "{head}- **Status:** Retired\n- **Retired in:** soon\n- **Replaced by:** none\n"
        ));
        assert!(text.contains("invalid version"), "{text}");
    }

    #[test]
    fn rule_file_level_display_and_parse() {
        for l in [
            Level::Must,
            Level::MustNot,
            Level::Should,
            Level::ShouldNot,
            Level::May,
        ] {
            assert_eq!(l.to_string().parse::<Level>(), Ok(l));
        }
        assert!("must".parse::<Level>().is_err());
    }
}
