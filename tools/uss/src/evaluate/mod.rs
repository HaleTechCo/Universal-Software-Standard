//! Evaluating a project record against the catalog.

mod check;
mod facts;
mod project;
mod score;

pub use check::check_record;
#[cfg(feature = "cli")]
pub(crate) use check::{leaves_project, normalise};
pub use facts::{applicable, resolve_facts};
pub use project::{MemProject, ProjectFiles, SubProject};
pub use score::{Evaluation, RuleOutcome, Score, Status, evaluate};

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
    /// most a quarter of the core rules by the 0.1.0 release. This test pins
    /// today's counts, so every change to them is a deliberate edit that a
    /// review sees and a plan handover reports.
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
        assert_eq!((floor, core), (59, 295));
    }
}
