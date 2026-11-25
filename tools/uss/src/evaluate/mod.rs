//! Evaluating a project record against the catalog.

mod check;
mod components;
mod facts;
mod project;
mod score;

pub use check::check_record;
#[cfg(feature = "cli")]
pub(crate) use check::{leaves_project, normalise};
pub use components::{Found, Walk, walk};
pub use facts::{applicable, resolve_facts};
pub use project::{MemProject, ProjectFiles, SubProject, project_root};
pub use score::{Evaluation, RuleOutcome, Score, Status, evaluate};

/// The message for a record that pins another version of the standard than
/// the one `carried` by this build. `None` when the versions are equal.
#[must_use]
pub fn version_mismatch(pinned: &str, carried: &str) -> Option<String> {
    (pinned != carried).then(|| {
        format!(
            "record pins USS {pinned}; this uss carries USS {carried}. Use uss {pinned}: download it from the v{pinned} release, or run: cargo install --locked --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v{pinned} uss"
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{Catalog, Rule, load};
    use crate::record::{Record, parse_record};
    use crate::source::{Embedded, Source};
    use crate::testfixture::standard;

    fn catalog() -> Catalog {
        load(&standard()).unwrap()
    }

    fn record(extra: &str) -> Record {
        let yaml = format!("standard: {{ version: 0.1.0 }}\nproject: {{ name: p }}\n{extra}");
        let yaml = if extra.contains("facts:") {
            yaml
        } else {
            format!("{yaml}facts: {{}}\n")
        };
        parse_record(&Embedded, "r.yaml", yaml.as_bytes()).unwrap()
    }

    fn ids(rules: &[&Rule]) -> Vec<String> {
        rules.iter().map(|r| r.id.clone()).collect()
    }

    #[test]
    fn version_mismatch_text() {
        assert_eq!(version_mismatch("0.1.0", "0.1.0"), None);
        let text = version_mismatch("0.2.0", "0.1.0").unwrap();
        assert!(
            text.starts_with("record pins USS 0.2.0; this uss carries USS 0.1.0."),
            "{text}"
        );
    }

    #[test]
    fn resolve_facts_rules() {
        let c = catalog();
        let r = record(
            "stack: { languages: [go] }\nfacts: { has-web-ui: true, uses-database: false }\n",
        );
        let f = resolve_facts(&c, &r);
        assert!(f.contains("has-web-ui"));
        assert!(f.contains("has-ui"));
        assert!(f.contains("lang.go"));
        assert!(!f.contains("uses-database"));
        assert!(!f.contains("has-plugin-interface"));

        let r = record("decisions: { USS-EXT-001: { adopt: true } }\n");
        assert!(resolve_facts(&c, &r).contains("has-plugin-interface"));
        let r = record("decisions: { USS-EXT-001: { adopt: false } }\n");
        assert!(!resolve_facts(&c, &r).contains("has-plugin-interface"));
    }

    #[test]
    fn resolve_facts_stack_kinds() {
        let c = catalog();
        let r = record("stack: { frameworks: [react], platforms: [docker], tools: [make] }\n");
        let f = resolve_facts(&c, &r);
        for want in ["framework.react", "platform.docker", "tool.make"] {
            assert!(f.contains(want), "{want} missing from {f:?}");
        }
    }

    #[test]
    fn applicable_sets() {
        let c = catalog();
        let a = applicable(&c, &record(""));
        assert_eq!(ids(&a), ["USS-EXT-001", "USS-SEC-001"]);

        let a = applicable(&c, &record("decisions: { USS-EXT-001: { adopt: true } }\n"));
        assert_eq!(ids(&a), ["USS-EXT-001", "USS-EXT-002", "USS-SEC-001"]);

        let a = applicable(&c, &record("stack: { languages: [go] }\n"));
        assert_eq!(ids(&a), ["USS-EXT-001", "USS-GO-001", "USS-SEC-001"]);
        assert!(!ids(&a).contains(&"USS-SEC-004".to_string()));
    }

    #[test]
    fn applicable_follows_implied_facts() {
        let c = catalog();
        let a = applicable(&c, &record("facts: { has-web-ui: true }\n"));
        assert!(ids(&a).contains(&"USS-SEC-003".to_string()));
    }

    /// The light floor of spec §6.4: the shipped record template, whose
    /// facts are all false, is the smallest project. The spec's budget is at
    /// most a quarter of the core rules by the 0.1.0 release. The first
    /// assertion pins today's counts, so every change to them is a deliberate
    /// edit that a review sees and a plan handover reports. The second holds
    /// the budget.
    #[test]
    fn all_false_record_floor() {
        let c = load(&Embedded).unwrap();
        let path = "templates/record.yaml";
        let data = Embedded.read(path).unwrap();
        let r = parse_record(&Embedded, path, &data).unwrap();
        let is_core = |rule: &Rule| rule.file.starts_with("standard/core/");
        let core = c
            .rules
            .iter()
            .filter(|rule| is_core(rule) && rule.active().is_some())
            .count();
        let floor = applicable(&c, &r)
            .into_iter()
            .filter(|rule| is_core(rule))
            .count();
        assert_eq!((floor, core), (59, 318));
        assert!(
            applicable(&c, &r).iter().all(|rule| is_core(rule)),
            "a non-core rule applies to the all-false record"
        );
        assert!(
            floor * 4 <= core,
            "{floor} of {core} core rules is over the spec §6.4 budget"
        );
    }
}
