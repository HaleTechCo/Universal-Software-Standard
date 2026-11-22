//! `standard/registry.yaml`: version, categories and stack IDs.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::LazyLock;

use regex::Regex;
use serde::Deserialize;

use super::Diagnostic;
use crate::source::Source;
use crate::yaml;

/// Path of the registry file, relative to the USS root.
const REGISTRY_PATH: &str = "standard/registry.yaml";

/// The valid stack kinds.
pub(crate) const STACK_KINDS: [&str; 4] = ["lang", "framework", "platform", "tool"];

/// Compiles a pattern that is a constant of this crate.
#[expect(
    clippy::expect_used,
    reason = "the patterns are constants, so a failure is a bug caught by the tests"
)]
pub(crate) fn constant_regex(pattern: &str) -> Regex {
    Regex::new(pattern).expect("constant regex is valid")
}

pub(crate) static VERSION_RE: LazyLock<Regex> =
    LazyLock::new(|| constant_regex(r"^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$"));
pub(crate) static ID_RE: LazyLock<Regex> =
    LazyLock::new(|| constant_regex(r"^[a-z0-9]+(-[a-z0-9]+)*$"));

/// A score category.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Category {
    /// Machine ID (lowercase, hyphenated).
    pub id: String,
    /// Display name.
    pub name: String,
}

/// The parsed `standard/registry.yaml`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    /// The standard's version.
    pub version: String,
    /// Score categories, in display order.
    pub categories: Vec<Category>,
    /// Known stack IDs by kind (`lang`, `framework`, `platform`, `tool`).
    pub stack: BTreeMap<String, Vec<String>>,
}

impl Registry {
    /// Finds a category by its display name.
    #[must_use]
    pub fn category_by_name(&self, name: &str) -> Option<&Category> {
        self.categories.iter().find(|c| c.name == name)
    }

    /// Tells whether `id` is a known stack ID of kind `kind`.
    #[must_use]
    pub fn has_stack(&self, kind: &str, id: &str) -> bool {
        self.stack
            .get(kind)
            .is_some_and(|ids| ids.iter().any(|i| i == id))
    }

    /// Checks the registry's contents, returning one message per problem.
    fn problems(&self) -> Vec<String> {
        let mut out = Vec::new();
        if !VERSION_RE.is_match(&self.version) {
            out.push(format!(
                "version {:?} is not a semantic version",
                self.version
            ));
        }
        let mut ids = BTreeSet::new();
        let mut names = BTreeSet::new();
        for c in &self.categories {
            if !ID_RE.is_match(&c.id) {
                out.push(format!("category id {:?} is not a valid ID", c.id));
            }
            if !ids.insert(c.id.as_str()) {
                out.push(format!("duplicate category id {:?}", c.id));
            }
            if !names.insert(c.name.as_str()) {
                out.push(format!("duplicate category name {:?}", c.name));
            }
        }
        for (kind, items) in &self.stack {
            if !STACK_KINDS.contains(&kind.as_str()) {
                out.push(format!("unknown stack kind {kind:?}"));
            }
            let mut seen = BTreeSet::new();
            for id in items {
                if !ID_RE.is_match(id) {
                    out.push(format!("stack {kind} id {id:?} is not a valid ID"));
                }
                if !seen.insert(id.as_str()) {
                    out.push(format!("duplicate stack {kind} id {id:?}"));
                }
            }
        }
        out
    }
}

/// Loads and validates `standard/registry.yaml` from `src`.
///
/// # Errors
///
/// Returns diagnostics when the file is missing, does not parse (unknown keys
/// are errors), or has invalid contents.
pub fn load_registry(src: &dyn Source) -> Result<Registry, Vec<Diagnostic>> {
    let Some(bytes) = src.read(REGISTRY_PATH) else {
        return Err(vec![Diagnostic::new(REGISTRY_PATH, "file not found")]);
    };
    let registry: Registry =
        yaml::from_slice(&bytes).map_err(|m| vec![Diagnostic::new(REGISTRY_PATH, m)])?;
    let problems = registry.problems();
    if problems.is_empty() {
        Ok(registry)
    } else {
        Err(problems
            .into_iter()
            .map(|m| Diagnostic::new(REGISTRY_PATH, m))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{Embedded, MemSource};

    const REAL: &[u8] = include_bytes!("../../../../standard/registry.yaml");

    fn mem(yaml: &str) -> MemSource {
        let mut m = MemSource::default();
        m.0.insert(REGISTRY_PATH.to_string(), yaml.as_bytes().to_vec());
        m
    }

    #[test]
    fn load_registry_valid() {
        let mut m = MemSource::default();
        m.0.insert(REGISTRY_PATH.to_string(), REAL.to_vec());
        let r = load_registry(&m).unwrap();
        assert_eq!(r.version, "0.1.0");
        assert_eq!(r.categories.len(), 9);
        assert_eq!(r.categories[7].name, "Accessibility & Usability");
        assert!(r.has_stack("lang", "go"));
        assert!(!r.has_stack("lang", "cobol"));
        assert!(!r.has_stack("os", "go"));
        assert_eq!(
            r.category_by_name("Safety").map(|c| c.id.as_str()),
            Some("safety")
        );
        assert!(r.category_by_name("Nope").is_none());
    }

    #[test]
    fn load_registry_errors() {
        let cases: [(&str, &str); 5] = [
            ("version: \"1.0\"\ncategories: []\nstack: {}\n", "version"),
            (
                "version: 0.1.0\ncategories:\n  - { id: a, name: X }\n  - { id: b, name: X }\nstack: {}\n",
                "duplicate category",
            ),
            (
                "version: 0.1.0\ncategories: []\nstack:\n  os: [linux]\n",
                "unknown stack kind \"os\"",
            ),
            (
                "version: 0.1.0\ncategories: []\nstack: {}\ncolour: red\n",
                "colour",
            ),
            ("", "standard/registry.yaml"),
        ];
        for (i, (yaml, want)) in cases.iter().enumerate() {
            let src = if i == 4 {
                MemSource::default()
            } else {
                mem(yaml)
            };
            let diags = load_registry(&src).unwrap_err();
            let text = diags
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n");
            assert!(text.contains(want), "case {i}: want {want:?} in {text}");
        }
    }

    #[test]
    fn crate_version_matches_registry() {
        let r = load_registry(&Embedded).unwrap();
        assert_eq!(env!("CARGO_PKG_VERSION"), r.version);
    }
}
